use serde::Serialize;

// ─── Domain types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct Section {
    pub level: u32,
    pub pier:  String,
    pub w:     f64,   // width (m)
    pub d:     f64,   // depth (m)
}

#[derive(Debug, Clone, Serialize)]
pub struct Force {
    pub level: u32,
    pub pier:  String,
    pub combo: String,  // "Gravity" | "Wind" | "Seismic"
    pub force: f64,     // kN
}

#[derive(Debug, Clone, Serialize)]
pub struct StressResult {
    pub level:  u32,
    pub pier:   String,
    pub combo:  String,
    pub area:   f64,    // m²  = W * D
    pub force:  f64,    // kN
    pub stress: f64,    // kPa = force / area
    pub id:     String, // composite key: "{pier}_{level}"
}

/// All pre-generated data, created once and stored as managed State
pub struct AppData {
    pub sections: Vec<Section>,
    pub forces:   Vec<Force>,
    pub stress:   Vec<StressResult>,
}

// ─── Fake data generation ────────────────────────────────────────────────────

const PIERS:  [&str; 5] = ["P1", "P2", "P3", "P4", "P5"];
const COMBOS: [&str; 3] = ["Gravity", "Wind", "Seismic"];
const LEVELS: u32       = 100;

fn base_dims(pier: &str) -> (f64, f64) {
    match pier {
        "P1" => (1.2, 0.6),
        "P2" => (1.0, 0.5),
        "P3" => (0.9, 0.45),
        "P4" => (1.1, 0.55),
        _    => (0.8, 0.4),
    }
}

fn taper(level: u32) -> f64 {
    let t = (level as f64 - 1.0) / (LEVELS as f64 - 1.0);
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

fn force_at_level(base: f64, level: u32, combo: &str) -> f64 {
    let norm = (level as f64 - 1.0) / (LEVELS as f64 - 1.0);
    match combo {
        "Gravity" => base * (1.0 - 0.85 * norm),
        "Wind"    => base * (0.3 + 0.7 * (1.0 - norm)),
        _         => {
            let mid  = 0.5_f64;
            let dist = (norm - mid).abs();
            base * (1.0 - 0.6 * dist)
        }
    }
}

// ─── Public: single entry point that builds everything once ─────────────────

pub fn build_app_data() -> AppData {
    // sections
    let mut sections = Vec::with_capacity((LEVELS as usize) * PIERS.len());
    for level in 1..=LEVELS {
        let t = taper(level);
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
    let mut forces = Vec::with_capacity((LEVELS as usize) * PIERS.len() * COMBOS.len());
    for level in 1..=LEVELS {
        for &pier in &PIERS {
            for &combo in &COMBOS {
                let base = base_force(pier, combo);
                let f    = force_at_level(base, level, combo);
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
    let section_map: HashMap<(u32, &str), &Section> = sections
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

    AppData { sections, forces, stress }
}