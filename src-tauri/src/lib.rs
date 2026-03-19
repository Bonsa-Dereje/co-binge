use tauri::command;
// Note: You can remove std::fs and uuid imports if not needed elsewhere
// use std::fs;
// use uuid::Uuid;

// 🔑 Device ID command - now returns a dummy ID for testing
#[command]
fn get_device_id(app_handle: tauri::AppHandle) -> String {
    // Return a dummy device ID for testing
    "TEST-DEVICE-12345".to_string()
    
    /* Original code commented out for reference:
    // Get app data directory
    let mut path = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("failed to get app data dir");

    // File where ID is stored
    path.push("device_id.txt");

    // If file exists → read and return
    if let Ok(id) = fs::read_to_string(&path) {
        return id;
    }

    // If not → generate new UUID
    let id = Uuid::new_v4().to_string();

    // Save it
    fs::write(&path, &id).expect("failed to write device id");

    id
    */
}

// 🚀 Tauri app entry
pub fn run() {
    tauri::Builder::default()
        // Register commands here
        .invoke_handler(tauri::generate_handler![get_device_id])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}