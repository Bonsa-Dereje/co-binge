use tauri::{command, Manager, WindowEvent, Emitter};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::fs;
use std::path::PathBuf;

#[command]
fn get_device_id(app_handle: tauri::AppHandle) -> String {
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

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            // ✅ CLONE for closure
            let window_clone = window.clone();

            window.on_window_event(move |event| {
                if let WindowEvent::DragDrop(event) = event {
                    println!("Drag event: {:?}", event);

                    if let tauri::DragDropEvent::Drop { paths, .. } = event {
                        let _ = window_clone.emit("custom://file-drop", paths);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_device_id])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}