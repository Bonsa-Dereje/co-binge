use tauri::{command, Manager, WindowEvent, Emitter, AppHandle};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Child};
use std::thread;
use std::time::Duration;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::io::{Write, BufRead, BufReader};

use screenshots::Screen;

use reqwest;
use mongodb::{Client};
use mongodb::bson::doc;
use chrono::Timelike;

use regex::Regex;
use arboard::Clipboard;

const VIDEO_EXTENSIONS: [&str; 7] = ["mp4", "mkv", "mov", "avi", "flv", "wmv", "webm"];

const MONGO_URI: &str =
    "mongodb+srv://coBinge:31211@cobinge.eicnmv9.mongodb.net/?appName=coBinge";

#[command]
fn get_device_id(app_handle: AppHandle) -> String {
    let mut path: PathBuf = app_handle
        .path()
        .app_data_dir()
        .expect("failed to get app data dir");

    fs::create_dir_all(&path).ok();
    path.push("device_id.txt");

    if path.exists() {
        if let Ok(id) = fs::read_to_string(&path) {
            return id.trim().to_string();
        }
    }

    let mut rng = thread_rng();
    let id: String = (0..12)
        .map(|_| char::from(rng.sample(Alphanumeric)).to_ascii_uppercase())
        .collect();

    fs::write(&path, &id).expect("failed to write device id");
    id
}

#[command]
async fn set_hosting_true(device_id: String) -> Result<(), String> {
    let client = Client::with_uri_str(MONGO_URI)
        .await
        .map_err(|e| e.to_string())?;

    let db = client.database("timeSync");
    let collection = db.collection::<mongodb::bson::Document>("timeSync");

    collection.update_one(
        doc! { "_id": device_id.clone() },
        doc! { "$set": { "hosting": true } },
        mongodb::options::UpdateOptions::builder().upsert(true).build(),
    )
    .await
    .map_err(|e| e.to_string())?;

    println!("✅ Hosting TRUE set for {}", device_id);
    Ok(())
}

#[command]
async fn join_pairing(app_handle: AppHandle) -> Result<(), String> {
    println!("JOIN FUNCTION CALLED");

    let device_id = get_device_id(app_handle.clone());
    println!("Current device id: {}", device_id);

    let clipboard = Clipboard::new()
        .map_err(|e| e.to_string())?
        .get_text()
        .map_err(|e| e.to_string())?
        .trim()
        .to_string();

    println!("Clipboard text: {}", clipboard);

    let re = Regex::new(r"^[A-Z0-9]{12}$").map_err(|e| e.to_string())?;
    if !re.is_match(&clipboard) {
        return Err("Invalid pairing code format".to_string());
    }

    let client = Client::with_uri_str(MONGO_URI)
        .await
        .map_err(|e| e.to_string())?;

    let db = client.database("timeSync");
    let collection = db.collection::<mongodb::bson::Document>("timeSync");

    let result = collection.update_one(
        doc! { "_id": clipboard.clone() },
        doc! { "$set": { "paired_to": device_id.clone() } },
        mongodb::options::UpdateOptions::builder().upsert(false).build(),
    )
    .await
    .map_err(|e| e.to_string())?;

    if result.matched_count == 0 {
        return Err("Host device not found in database".to_string());
    }

    println!("🔗 Paired client {} -> host {}", device_id, clipboard);

    // ---------------- NEW LOGIC ----------------
    // Ping Mongo every second looking for "fileName"

    let collection_clone = collection.clone();
    let pairing_code = clipboard.clone();

    tauri::async_runtime::spawn(async move {
        loop {
            match collection_clone
                .find_one(doc! { "_id": pairing_code.clone() }, None)
                .await
            {
                Ok(Some(doc)) => {
                    match doc.get_str("fileName") {
                        Ok(file_name) => {
                            println!("fileName found: {}", file_name);
                        }
                        Err(_) => {
                            println!("Waiting for fileName...");
                        }
                    }
                }

                Ok(None) => {
                    println!("Host document not found");
                }

                Err(e) => {
                    println!("Mongo polling error: {}", e);
                }
            }

            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });

    Ok(())
}
#[command]
async fn pair_checker(app_handle: AppHandle) -> Result<(), String> {
    println!("checking .....");

    let device_id = get_device_id(app_handle.clone());
    println!("Current device id: {}", device_id);

    set_hosting_true(device_id.clone()).await?;

    let client = Client::with_uri_str(MONGO_URI)
        .await
        .map_err(|e| e.to_string())?;

    let db = client.database("timeSync");
    let collection = db.collection::<mongodb::bson::Document>("timeSync");

    loop {
        let result = collection
            .find_one(doc! { "_id": device_id.clone() }, None)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(document) = result {
            if let Some(paired_to) = document.get_str("paired_to").ok() {
                println!("PAIRED");
                println!("paired_to -> {}", paired_to);
                break;
            } else {
                println!("not paired");
            }
        } else {
            println!("not paired");
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}

// ---------------- TIME + MONGO ----------------

async fn fetch_host_time() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let res = reqwest::get("https://time.now/developer/api/ip").await?;
    let json: serde_json::Value = res.json().await?;

    let datetime_str = json["datetime"]
        .as_str()
        .ok_or("Missing datetime field")?;

    let mut date = chrono::DateTime::parse_from_rfc3339(datetime_str)?
        .with_timezone(&chrono::Utc);

    date = date + chrono::Duration::seconds(5);

    let sync_time = format!(
        "{:02}:{:02}:{:02}",
        date.hour(),
        date.minute(),
        date.second()
    );

    Ok(sync_time)
}

async fn save_time_to_mongo(
    device_id: String,
    sync_time: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::with_uri_str(MONGO_URI).await?;
    let db = client.database("timeSync");
    let collection = db.collection::<mongodb::bson::Document>("timeSync");

    collection.update_one(
        doc! { "_id": device_id.clone() },
        doc! { "$set": { "syncTime": &sync_time } },
        mongodb::options::UpdateOptions::builder().upsert(true).build(),
    )
    .await?;

    println!("✅ Saved for device {} -> {}", device_id, sync_time);
    Ok(())
}

async fn run_time_sync(device_id: String) {
    if let Ok(time) = fetch_host_time().await {
        println!("Fetched time: {}", time);
        if let Err(e) = save_time_to_mongo(device_id, time).await {
            println!("Mongo save error: {:?}", e);
        }
    }
}

// ---------------- HELPERS ----------------

/// Parse "HH:MM:SS" into total seconds
fn parse_time_to_seconds(time_str: &str) -> Option<u32> {
    let parts: Vec<&str> = time_str.trim().split(':').collect();
    if parts.len() != 3 {
        return None;
    }
    let h: u32 = parts[0].parse().ok()?;
    let m: u32 = parts[1].parse().ok()?;
    let s: u32 = parts[2].parse().ok()?;
    Some(h * 3600 + m * 60 + s)
}

// ---------------- VLC ----------------

fn send_vlc_command(rc_port: u16, command: &str) -> Result<String, String> {
    use std::net::TcpStream;
    use std::io::{BufReader, Write, BufRead};

    match TcpStream::connect_timeout(
        &format!("127.0.0.1:{}", rc_port).parse().unwrap(),
        Duration::from_secs(2),
    ) {
        Ok(mut stream) => {
            if let Err(e) = write!(stream, "{}\n", command) {
                return Err(format!("Failed to send command: {}", e));
            }

            let mut reader = BufReader::new(stream);
            let mut response = String::new();
            let _ = reader.read_line(&mut response);

            Ok(response)
        }
        Err(e) => Err(format!("Failed to connect to VLC RC interface: {}", e)),
    }
}

fn start_vlc_with_rc(video_path: &PathBuf, rc_port: u16) -> Result<Child, std::io::Error> {
    Command::new("C:\\Program Files\\VideoLAN\\VLC\\vlc.exe")
        .arg("--video-on-top")
        .arg("--extraintf=rc")
        .arg("--rc-host=127.0.0.1:".to_string() + &rc_port.to_string())
        .arg("--rc-quiet")
        .arg(video_path)
        .spawn()
}

fn control_vlc_timeline(app_handle: AppHandle, rc_port: u16, device_id: String) {
    thread::spawn(move || {

        // ── PHASE 1: wait for VLC to be open, then reset to 0:00 ──

        thread::sleep(Duration::from_secs(5));

        loop {
            match send_vlc_command(rc_port, "get_time") {
                Ok(current_time) => {
                    let cleaned_time = current_time.trim();
                    println!("VLC current time: {}", cleaned_time);

                    if cleaned_time == "0" {
                        println!("VLC is now at 0:00");

                        let _ = app_handle.emit(
                            "vlc:controlComplete",
                            "VLC initialized at 0:00",
                        );

                        break;
                    }

                    let _ = send_vlc_command(rc_port, "seek 0");
                    let _ = send_vlc_command(rc_port, "stop");
                    println!("Attempted to reset VLC timeline to 0:00");
                }
                Err(e) => {
                    println!("Failed to get VLC time: {}", e);
                }
            }

            thread::sleep(Duration::from_secs(1));
        }

        // ── PHASE 2: read syncTime from Mongo, add 8 s → target_time ──

        println!("⏳ Fetching syncTime from Mongo for target_time...");

        let target_seconds: u32 = {
            let rt = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");

            rt.block_on(async {
                let client = match Client::with_uri_str(MONGO_URI).await {
                    Ok(c) => c,
                    Err(e) => {
                        println!("Mongo connect error (target fetch): {}", e);
                        return 0u32;
                    }
                };

                let collection = client
                    .database("timeSync")
                    .collection::<mongodb::bson::Document>("timeSync");

                let doc_result = collection
                    .find_one(doc! { "_id": device_id.clone() }, None)
                    .await;

                match doc_result {
                    Ok(Some(document)) => {
                        if let Ok(sync_time_str) = document.get_str("syncTime") {
                            println!("📥 syncTime from Mongo: {}", sync_time_str);

                            if let Some(secs) = parse_time_to_seconds(sync_time_str) {
                                let target = secs + 8; // ✅ CHANGED: +5 → +8
                                println!("🎯 target_time (seconds): {}", target);
                                return target;
                            }
                        }
                        println!("⚠️ syncTime field missing or unreadable");
                        0u32
                    }
                    Ok(None) => {
                        println!("⚠️ Device document not found in Mongo");
                        0u32
                    }
                    Err(e) => {
                        println!("Mongo find error: {}", e);
                        0u32
                    }
                }
            })
        };

        if target_seconds == 0 {
            println!("❌ Could not resolve target_time — aborting sync-to-play");
            return;
        }

        println!(
            "⏱️ Polling API — will play VLC when seconds >= {}",
            target_seconds
        );

        // ── FIX: NO %60 MATCHING (this caused missed trigger) ──

        let mut rt = tokio::runtime::Runtime::new().expect("tokio rt");

        loop {
            let ping_time_str = rt.block_on(async {
                match reqwest::get("https://time.now/developer/api/ip").await {
                    Ok(res) => match res.json::<serde_json::Value>().await {
                        Ok(json) => json["datetime"]
                            .as_str()
                            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                            .map(|dt| {
                                let utc = dt.with_timezone(&chrono::Utc);
                                format!(
                                    "{:02}:{:02}:{:02}",
                                    utc.hour(),
                                    utc.minute(),
                                    utc.second()
                                )
                            }),
                        Err(e) => {
                            println!("JSON parse error: {}", e);
                            None
                        }
                    },
                    Err(e) => {
                        println!("API fetch error: {}", e);
                        None
                    }
                }
            });

            if let Some(ping_str) = ping_time_str {
                println!("🕐 ping_time: {}", ping_str);

                if let Some(ping_seconds) = parse_time_to_seconds(&ping_str) {
                    println!(
                        "   ping={} | target={}",
                        ping_seconds, target_seconds
                    );

                    // ✅ FIXED: reliable trigger (no missed second window)
                    if ping_seconds >= target_seconds {
                        println!("▶️ Time reached — firing VLC play!");

                        let _ = send_vlc_command(rc_port, "play");

                        let _ = app_handle.emit(
                            "vlc:syncPlay",
                            format!("Played at ping_time={}", ping_str),
                        );

                        break;
                    }
                }
            }

            thread::sleep(Duration::from_secs(1));
        }
    });
}

// ---------------- SCREENSHOTS ----------------

fn start_screenshot_loop(app_handle: AppHandle, running: Arc<AtomicBool>) {
    thread::spawn(move || {
        let mut output_dir = app_handle
            .path()
            .app_data_dir()
            .expect("failed to get app data dir");

        output_dir.push("syncCalc");
        output_dir.push("Images");
        fs::create_dir_all(&output_dir).ok();

        let screen = Screen::all().unwrap().first().unwrap().clone();
        let mut counter = 0;

        while running.load(Ordering::Relaxed) {
            if let Ok(image) = screen.capture() {
                let file_path = output_dir.join(format!("screenshot_{}.png", counter));
                let _ = image.save(&file_path);
            }
            counter += 1;
            thread::sleep(Duration::from_secs(5));
        }
    });
}

fn monitor_vlc(mut child: Child, running: Arc<AtomicBool>) {
    thread::spawn(move || {
        let _ = child.wait();
        running.store(false, Ordering::Relaxed);
    });
}

// ---------------- MAIN ----------------

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            let window_for_event = window.clone();
            let app_handle = app.handle().clone();

            window.on_window_event(move |event| {
                if let WindowEvent::DragDrop(event) = event {
                    if let tauri::DragDropEvent::Drop { paths, .. } = event {

                        let video_path = paths.iter().find(|path| {
                            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                                VIDEO_EXTENSIONS.contains(&ext.to_lowercase().as_str())
                            } else {
                                false
                            }
                        });

                        let is_video = video_path.is_some();
                        println!("Is video? {}", is_video);

                        // ── EXISTING ASYNC LOGIC ──

                        if let Some(path) = video_path {

                            let device_id = get_device_id(app_handle.clone());

                            // NEW: extract filename
                            let file_name = path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("unknown")
                                .to_string();

                            tauri::async_runtime::spawn(async move {

                                run_time_sync(device_id.clone()).await;

                                let client = match Client::with_uri_str(MONGO_URI).await {
                                    Ok(client) => client,
                                    Err(e) => {
                                        println!("Mongo connection error: {}", e);
                                        return;
                                    }
                                };

                                let db = client.database("timeSync");
                                let collection =
                                    db.collection::<mongodb::bson::Document>("timeSync");

                                // UPDATED: session_hot + fileName
                                match collection.update_one(
                                    doc! { "_id": device_id.clone() },
                                    doc! {
                                        "$set": {
                                            "session_hot": true,
                                            "fileName": file_name.clone()
                                        }
                                    },
                                    mongodb::options::UpdateOptions::builder()
                                        .upsert(true)
                                        .build(),
                                )
                                .await
                                {
                                    Ok(_) => {
                                        println!(
                                            "✅ session_hot=true and fileName={} saved",
                                            file_name
                                        );
                                    }
                                    Err(e) => {
                                        println!("Failed updating Mongo fields: {}", e);
                                    }
                                }

                                let result = match collection
                                    .find_one(doc! { "_id": device_id.clone() }, None)
                                    .await
                                {
                                    Ok(res) => res,
                                    Err(e) => {
                                        println!("Find error: {}", e);
                                        return;
                                    }
                                };

                                if let Some(document) = result {
                                    let hosting = document.get_bool("hosting").unwrap_or(false);

                                    if hosting {
                                        println!("Hosting is TRUE");

                                        match fetch_host_time().await {
                                            Ok(sync_start_time) => {
                                                println!(
                                                    "Fetched sync_start_time -> {}",
                                                    sync_start_time
                                                );
                                            }
                                            Err(e) => {
                                                println!("Failed fetching API time: {}", e);
                                            }
                                        }
                                    } else {
                                        println!("hosting != true");
                                    }
                                } else {
                                    println!("Device not found in DB");
                                }
                            });

                            // ── EXISTING VLC LOGIC — now passes device_id to control_vlc_timeline ──

                            let rc_port = 42123;
                            let device_id_for_vlc = get_device_id(app_handle.clone());

                            if let Ok(child) = start_vlc_with_rc(&path, rc_port) {
                                let running = Arc::new(AtomicBool::new(true));

                                start_screenshot_loop(app_handle.clone(), running.clone());
                                monitor_vlc(child, running.clone());

                                // Pass device_id so the sync thread can read syncTime
                                control_vlc_timeline(
                                    app_handle.clone(),
                                    rc_port,
                                    device_id_for_vlc,
                                );
                            }
                        }

                        let _ = window_for_event.emit("file:isVideo", is_video);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_device_id,
            set_hosting_true,
            join_pairing,
            pair_checker
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}