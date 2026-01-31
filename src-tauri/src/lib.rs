mod commands;
mod db;

use tauri::Manager;
use tauri::menu::{MenuBuilder, SubmenuBuilder};

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        // ── Setup: menu + event handler ──────────────────────────────
        .setup(|app| {
            // File submenu — uses .text("id", "label") shorthand
            let file_menu = SubmenuBuilder::new(app, "File")
                .text("quit", "Quit")
                .build()?;

            // View submenu
            let view_menu = SubmenuBuilder::new(app, "View")
                .text("reload", "Reload")
                .build()?;

            // Assemble and attach to app
            let menu = MenuBuilder::new(app)
                .items(&[&file_menu, &view_menu])
                .build()?;
            app.set_menu(menu)?;

            // Menu event handler lives inside setup, bound to this app handle
            app.on_menu_event(|app_handle, event| {
                match event.id().0.as_str() {
                    "quit" => std::process::exit(0),
                    "reload" => {
                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = window.reload();
                        }
                    }
                    _ => {}
                }
            });

            Ok(())
        })
        // ── Commands ─────────────────────────────────────────────────
        .invoke_handler(tauri::generate_handler![
            commands::get_sections,
            commands::get_forces,
            commands::get_stress_results,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}