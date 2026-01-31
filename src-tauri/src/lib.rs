mod commands;
mod db;

use tauri::Manager;
use tauri::menu::{MenuBuilder, SubmenuBuilder};

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // ── Managed state: generate once, serve forever ─────────────
            app.manage(db::build_app_data());

            // ── Native menu ──────────────────────────────────────────────
            let file_menu = SubmenuBuilder::new(app, "File")
                .text("quit", "Quit")
                .build()?;

            let view_menu = SubmenuBuilder::new(app, "View")
                .text("reload", "Reload")
                .build()?;

            let menu = MenuBuilder::new(app)
                .items(&[&file_menu, &view_menu])
                .build()?;
            app.set_menu(menu)?;

            // ── Menu event handler ───────────────────────────────────────
            // `move` is required per docs — needed when the closure captures
            // references to menu items (e.g. for dynamic check/icon updates).
            app.on_menu_event(move |app_handle: &tauri::AppHandle, event| {
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
        // ── Commands ─────────────────────────────────────────────────────
        .invoke_handler(tauri::generate_handler![
            commands::get_sections,
            commands::get_forces,
            commands::get_stress_results,
            commands::calculate_stress,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}