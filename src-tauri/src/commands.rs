use tauri::State;
use crate::db::{self, AppData};

// All commands are async — they spawn on a worker thread, not the main thread.
// This is the correct pattern per Tauri docs, and is essential when these
// become async DB calls with SeaORM later.

#[tauri::command]
pub async fn get_sections(data: State<'_, AppData>) -> Result<Vec<db::Section>, ()> {
    Ok(data.sections.clone())
}

#[tauri::command]
pub async fn get_forces(data: State<'_, AppData>) -> Result<Vec<db::Force>, ()> {
    Ok(data.forces.clone())
}

#[tauri::command]
pub async fn get_stress_results(data: State<'_, AppData>) -> Result<Vec<db::StressResult>, ()> {
    Ok(data.stress.clone())
}

/// Recalculate stress with user-defined parameters.
/// This is the MUTATION command — takes params, applies them to base data, returns new results.
#[tauri::command]
pub async fn calculate_stress(
    data:   State<'_, AppData>,
    params: db::StressParams,
) -> Result<Vec<db::StressResult>, ()> {
    Ok(db::calculate_with_params(&data, &params))
}