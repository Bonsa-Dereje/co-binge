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

// NEW IMPORTS
use reqwest;
use mongodb::{Client};
use mongodb::bson::doc;
use chrono::{Timelike};

const VIDEO_EXTENSIONS: [&str; 7] = ["mp4", "mkv", "mov", "avi", "flv", "wmv", "webm"];

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

async fn save_time_to_mongo(sync_time: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let uri = std::env::var("MONGO_URI")?;
    let client = Client::with_uri_str(uri).await?;

    let db = client.database("timeSync");
    let collection = db.collection::<mongodb::bson::Document>("timeSync");

    collection.update_one(
        doc! { "_id": "hostTime" },
        doc! {
            "$set": {
                "syncTime": sync_time
            }
        },
        mongodb::options::UpdateOptions::builder().upsert(true).build(),
    ).await?;

    println!("✅ Saved syncTime to MongoDB");
    Ok(())
}

async fn run_time_sync() {
    if let Ok(time) = fetch_host_time().await {
        println!("Fetched time: {}", time);

        if let Err(e) = save_time_to_mongo(time).await {
            println!("Mongo save error: {:?}", e);
        }
    }
}

// ---------------- VLC ----------------

fn send_vlc_command(rc_port: u16, command: &str) -> Result<String, String> {
    use std::net::TcpStream;

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

fn control_vlc_timeline(app_handle: AppHandle, rc_port: u16) {
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));

        let _ = send_vlc_command(rc_port, "seek +0.1");
        thread::sleep(Duration::from_millis(100));
        let _ = send_vlc_command(rc_port, "pause");

        thread::sleep(Duration::from_secs(5));
        let _ = send_vlc_command(rc_port, "pause");

        thread::sleep(Duration::from_secs(5));
        let _ = send_vlc_command(rc_port, "seek +60");

        thread::sleep(Duration::from_millis(200));
        let _ = send_vlc_command(rc_port, "pause");

        thread::sleep(Duration::from_secs(3));
        let _ = send_vlc_command(rc_port, "pause");

        let _ = app_handle.emit("vlc:controlComplete", "Sequence complete");
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

            // ✅ FIX: clone BEFORE move
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

                        // ---------------- TIME SYNC ----------------
                        if is_video {
                            tauri::async_runtime::spawn(async {
                                run_time_sync().await;
                            });
                        }

                        // ---------------- VLC ----------------
                        if let Some(path) = video_path {
                            let rc_port = 42123;

                            if let Ok(child) = start_vlc_with_rc(&path, rc_port) {
                                let running = Arc::new(AtomicBool::new(true));

                                start_screenshot_loop(app_handle.clone(), running.clone());
                                monitor_vlc(child, running.clone());
                                control_vlc_timeline(app_handle.clone(), rc_port);
                            }
                        }

                        let _ = window_for_event.emit("file:isVideo", is_video);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_device_id])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}