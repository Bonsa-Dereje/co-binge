use tauri::{command, Manager};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::fs;
use std::path::PathBuf;

#[command]
fn get_device_id(app_handle: tauri::AppHandle) -> String {
    // Get app data directory
    let mut path: PathBuf = app_handle
        .path()
        .app_data_dir()
        .expect("failed to get app data dir");

    // Create folder if it doesn't exist
    fs::create_dir_all(&path).ok();

    // File path: device_id.txt
    path.push("device_id.txt");

    // ✅ If file exists → read and return
    if path.exists() {
        if let Ok(id) = fs::read_to_string(&path) {
            return id.trim().to_string();
        }
    }

    // ❌ If not → generate new ID
    let mut rng = thread_rng();
    let id: String = (0..12)
        .map(|_| char::from(rng.sample(Alphanumeric)).to_ascii_uppercase())
        .collect();

    // Save to file
    fs::write(&path, &id).expect("failed to write device id");

    id
}

// 🚀 Tauri app entry point
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_device_id])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}