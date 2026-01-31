use tauri::{Builder, Wry};
use tauri::menu::MenuBuilder;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    //delay to simulate a long running task
    std::thread::sleep(std::time::Duration::from_secs(3));
    format!("Hello, {}! You've been greeted from Rust!", name)
}



pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
