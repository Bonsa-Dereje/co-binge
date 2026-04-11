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

use screenshots::Screen;

// Common video file extensions
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

// 🔥 Screenshot loop with stop control
fn start_screenshot_loop(app_handle: AppHandle, running: Arc<AtomicBool>) {
    thread::spawn(move || {
        let mut output_dir = app_handle
            .path()
            .app_data_dir()
            .expect("failed to get app data dir");

        // ✅ REQUIRED STRUCTURE
        output_dir.push("syncCalc");
        output_dir.push("Images");

        fs::create_dir_all(&output_dir).ok();

        let screen = Screen::all()
            .unwrap()
            .first()
            .expect("No screen found")
            .clone();

        let mut counter = 0;

        while running.load(Ordering::Relaxed) {
            match screen.capture() {
                Ok(image) => {
                    let file_path = output_dir.join(format!("screenshot_{}.png", counter));
                    if let Err(e) = image.save(&file_path) {
                        println!("Failed to save screenshot: {:?}", e);
                    } else {
                        println!("Saved: {:?}", file_path);
                    }
                }
                Err(e) => println!("Capture failed: {:?}", e),
            }

            counter += 1;
            thread::sleep(Duration::from_secs(5));
        }

        // 🔥 CLEANUP AFTER VLC CLOSES
        if let Ok(entries) = fs::read_dir(&output_dir) {
            for entry in entries.flatten() {
                let _ = fs::remove_file(entry.path());
            }
        }

        println!("🧹 All screenshots deleted");
    });
}

// 🔥 Monitor VLC process
fn monitor_vlc(mut child: Child, running: Arc<AtomicBool>) {
    thread::spawn(move || {
        let _ = child.wait(); // waits until VLC closes
        println!("🎬 VLC closed");

        running.store(false, Ordering::Relaxed);
    });
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            let window_clone = window.clone();

            let app_handle = app.handle();
            let app_handle_for_event = app_handle.clone();

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

                        if let Some(path) = video_path {
                            println!("Opening in VLC: {:?}", path);

                            match Command::new("C:\\Program Files\\VideoLAN\\VLC\\vlc.exe")
                                .arg(path)
                                .spawn()
                            {
                                Ok(child) => {
                                    let running = Arc::new(AtomicBool::new(true));

                                    // start screenshots
                                    start_screenshot_loop(
                                        app_handle_for_event.clone(),
                                        running.clone(),
                                    );

                                    // monitor VLC close
                                    monitor_vlc(child, running.clone());
                                }
                                Err(e) => println!("Failed to start VLC: {:?}", e),
                            }
                        }

                        let _ = window_clone.emit("file:isVideo", is_video);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_device_id])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}