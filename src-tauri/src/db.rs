// src-tauri/src/db.rs
use serde::{Serialize, Deserialize};
use ts_rs::TS;

// ─── Domain types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct Section {
    pub level: u16,
    pub pier:  String,
    pub w:     f64,
    pub d:     f64,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct Force {
    pub level: u16,
    pub pier:  String,
    pub combo: String,
    pub force: f64,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct StressResult {
    pub level:  u16,
    pub pier:   String,
    pub combo:  String,
    pub area:   f64,
    pub force:  f64,
    pub stress: f64,
    pub id:     String,
}

/// User-defined calculation parameters
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]  // ← ADDED
pub struct StressParams {
    pub load_factors: LoadFactors,
    pub level_range:  (u16, u16),
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]  // ← ADDED
pub struct LoadFactors {
    pub gravity: f64,
    pub wind:    f64,
    pub seismic: f64,
}


/// All pre-generated data, created once and stored as managed State.
/// Now configurable — generate for any max_level the user requests.
pub struct AppData {
    pub sections: Vec<Section>,
    pub forces:   Vec<Force>,
    pub stress:   Vec<StressResult>,
    pub max_level: u16,
}

// ─── Fake data generation ────────────────────────────────────────────────────

const PIERS:  [&str; 5] = ["P1", "P2", "P3", "P4", "P5"];
const COMBOS: [&str; 3] = ["Gravity", "Wind", "Seismic"];

fn base_dims(pier: &str) -> (f64, f64) {
    match pier {
        "P1" => (1.2, 0.6),
        "P2" => (1.0, 0.5),
        "P3" => (0.9, 0.45),
        "P4" => (1.1, 0.55),
        _    => (0.8, 0.4),
    }
}

/// Taper factor: dimensions shrink linearly to 40% at max_level
fn taper(level: u16, max_level: u16) -> f64 {
    if max_level <= 1 {
        return 1.0;
    }
    let t = (level as f64 - 1.0) / (max_level as f64 - 1.0);
    1.0 - 0.6 * t
}

fn base_force(pier: &str, combo: &str) -> f64 {
    let pier_factor = match pier {
        "P1" => 1.0,
        "P2" => 0.85,
        "P3" => 0.75,
        "P4" => 0.92,
        _    => 0.68,
    };
    let combo_factor = match combo {
        "Gravity" => 5000.0,
        "Wind"    => 2200.0,
        _         => 3100.0,
    };
    pier_factor * combo_factor
}

/// Force drops off with height (gravity accumulates downward; lateral loads peak mid-height)
fn force_at_level(base: f64, level: u16, max_level: u16, combo: &str) -> f64 {
    if max_level <= 1 {
        return base;
    }
    let norm = (level as f64 - 1.0) / (max_level as f64 - 1.0); // 0..1
    match combo {
        "Gravity" => base * (1.0 - 0.85 * norm),
        "Wind"    => base * (0.3 + 0.7 * (1.0 - norm)),
        _         => {
            // Seismic: triangular — peaks around mid-height
            let mid  = 0.5_f64;
            let dist = (norm - mid).abs();
            base * (1.0 - 0.6 * dist)
        }
    }
}

// ─── Public: build data for a given max level ────────────────────────────────

/// Build app data for any number of levels.
/// Default is 100, but user can request more (e.g., 200-story building).
pub fn build_app_data_with_levels(max_level: u16) -> AppData {
    let max_level = max_level.max(1); // ensure at least 1 level

    // sections
    let mut sections = Vec::with_capacity((max_level as usize) * PIERS.len());
    for level in 1..=max_level {
        let t = taper(level, max_level);
        for &pier in &PIERS {
            let (w, d) = base_dims(pier);
            sections.push(Section {
                level,
                pier: pier.to_string(),
                w:    (w * t * 1000.0).round() / 1000.0,
                d:    (d * t * 1000.0).round() / 1000.0,
            });
        }
    }

    // forces
    let mut forces = Vec::with_capacity((max_level as usize) * PIERS.len() * COMBOS.len());
    for level in 1..=max_level {
        for &pier in &PIERS {
            for &combo in &COMBOS {
                let base = base_force(pier, combo);
                let f    = force_at_level(base, level, max_level, combo);
                forces.push(Force {
                    level,
                    pier:  pier.to_string(),
                    combo: combo.to_string(),
                    force: (f * 100.0).round() / 100.0,
                });
            }
        }
    }

    // stress — cross-table join on (level, pier)
    use std::collections::HashMap;
    let section_map: HashMap<(u16, &str), &Section> = sections
        .iter()
        .map(|s| ((s.level, s.pier.as_str()), s))
        .collect();

    let mut stress = Vec::with_capacity(forces.len());
    for f in &forces {
        if let Some(sec) = section_map.get(&(f.level, f.pier.as_str())) {
            let area   = sec.w * sec.d;
            let s      = if area > 0.0 { f.force / area } else { 0.0 };
            stress.push(StressResult {
                level:  f.level,
                pier:   f.pier.clone(),
                combo:  f.combo.clone(),
                area:   (area * 1000.0).round() / 1000.0,
                force:  f.force,
                stress: (s * 100.0).round() / 100.0,
                id:     format!("{}_{}", f.pier, f.level),
            });
        }
    }

    AppData { sections, forces, stress, max_level }
}

/// Default: build for 100 levels (backward compatible)
pub fn build_app_data() -> AppData {
    build_app_data_with_levels(100)
}

/// Apply user-defined parameters to base data and recalculate stress.
/// This is a PURE function — reads base data, applies factors/filters, returns new results.
/// Does NOT mutate AppData.
///
/// If user requests a level range beyond AppData.max_level, we auto-extend on the fly.
pub fn calculate_with_params(base: &AppData, params: &StressParams) -> Vec<StressResult> {
    use std::collections::HashMap;

    let (min_level, max_level) = params.level_range;

    // If requested max exceeds base data, generate extended data on the fly
    let extended_data;
    let data_to_use = if max_level > base.max_level {
        extended_data = build_app_data_with_levels(max_level);
        &extended_data
    } else {
        base
    };

    // Filter sections to requested level range
    let sections: Vec<_> = data_to_use.sections
        .iter()
        .filter(|s| s.level >= min_level && s.level <= max_level)
        .collect();

    let section_map: HashMap<(u16, &str), &Section> = sections
        .iter()
        .map(|s| ((s.level, s.pier.as_str()), *s))
        .collect();

    // Apply load factors to forces and filter to level range
    let mut results = Vec::new();
    for f in &data_to_use.forces {
        if f.level < min_level || f.level > max_level {
            continue;
        }

        let factor = match f.combo.as_str() {
            "Gravity" => params.load_factors.gravity,
            "Wind"    => params.load_factors.wind,
            _         => params.load_factors.seismic,
        };

        let adjusted_force = f.force * factor;

        if let Some(sec) = section_map.get(&(f.level, f.pier.as_str())) {
            let area   = sec.w * sec.d;
            let stress = if area > 0.0 { adjusted_force / area } else { 0.0 };
            results.push(StressResult {
                level:  f.level,
                pier:   f.pier.clone(),
                combo:  f.combo.clone(),
                area:   (area * 1000.0).round() / 1000.0,
                force:  (adjusted_force * 100.0).round() / 100.0,
                stress: (stress * 100.0).round() / 100.0,
                id:     format!("{}_{}", f.pier, f.level),
            });
        }
    }

    results
}



// Add this at the END of the file
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn export_typescript_bindings() {
//         // These will write to ../src/types/bindings/
//         Section::export(&Default::default()).expect("Failed to export Section");
//         Force::export(&Default::default()).expect("Failed to export Force");
//         StressResult::export(&Default::default()).expect("Failed to export StressResult");
//         StressParams::export(&Default::default()).expect("Failed to export StressParams");
//         LoadFactors::export(&Default::default()).expect("Failed to export LoadFactors");
//     }
// }