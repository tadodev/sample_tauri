use crate::db;

#[tauri::command]
pub fn get_sections() -> Vec<db::Section> {
    db::generate_sections()
}

#[tauri::command]
pub fn get_forces() -> Vec<db::Force> {
    db::generate_forces()
}

#[tauri::command]
pub fn get_stress_results() -> Vec<db::StressResult> {
    db::generate_stress_results()
}