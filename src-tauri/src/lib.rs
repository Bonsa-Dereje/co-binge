use tauri::command;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

#[command]
fn get_device_id(_app_handle: tauri::AppHandle) -> String {
    let mut rng = thread_rng();

    let id: String = (0..12)
        .map(|_| char::from(rng.sample(Alphanumeric)).to_ascii_uppercase())
        .collect();

    id
}

// 🚀 Tauri app entry point
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_device_id])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}