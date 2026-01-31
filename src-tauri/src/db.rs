use serde::Serialize;

// ─── Domain types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct Section {
    pub level: u16,
    pub pier:  String,
    pub w:     f64,   // width (m)
    pub d:     f64,   // depth (m)
}

#[derive(Debug, Clone, Serialize)]
pub struct Force {
    pub level: u16,
    pub pier:  String,
    pub combo: String,  // "Gravity" | "Wind" | "Seismic"
    pub force: f64,     // kN
}

#[derive(Debug, Clone, Serialize)]
pub struct StressResult {
    pub level:  u16,
    pub pier:   String,
    pub combo:  String,
    pub area:   f64,    // m²  = W * D
    pub force:  f64,    // kN
    pub stress: f64,    // kPa = force / area
    pub id:     String, // composite key: "{pier}_{level}"
}

// ─── Fake data generation ────────────────────────────────────────────────────
// Walls taper as you go up (wider at base, narrower at top).
// Forces decrease with height too — base carries the most load.

const PIERS:  [&str; 5] = ["P1", "P2", "P3", "P4", "P5"];
const COMBOS: [&str; 3] = ["Gravity", "Wind", "Seismic"];
const LEVELS: u16       = 100;

/// Base dimensions per pier (ground floor, in metres)
fn base_dims(pier: &str) -> (f64, f64) {
    match pier {
        "P1" => (1.2, 0.6),
        "P2" => (1.0, 0.5),
        "P3" => (0.9, 0.45),
        "P4" => (1.1, 0.55),
        _    => (0.8, 0.4),   // P5
    }
}

/// Taper factor: dimensions shrink linearly to 40 % at roof
fn taper(level: u16) -> f64 {
    let t = (level as f64 - 1.0) / (LEVELS as f64 - 1.0); // 0 at L1, 1 at L100
    1.0 - 0.6 * t
}

/// Base force magnitude per pier per combo (kN at ground)
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
        _         => 3100.0,  // Seismic
    };
    pier_factor * combo_factor
}

/// Force drops off with height (gravity accumulates downward; lateral loads peak mid-height)
fn force_at_level(base: f64, level: u16, combo: &str) -> f64 {
    let norm = (level as f64 - 1.0) / (LEVELS as f64 - 1.0); // 0..1
    match combo {
        "Gravity" => base * (1.0 - 0.85 * norm),          // drops to 15 % at top
        "Wind"    => base * (0.3 + 0.7 * (1.0 - norm)),   // peaks at base, 30 % at top
        _         => {
            // Seismic: triangular — peaks around mid-height
            let mid = 0.5_f64;
            let dist = (norm - mid).abs();
            base * (1.0 - 0.6 * dist)
        }
    }
}

// ─── Public generators ───────────────────────────────────────────────────────

pub fn generate_sections() -> Vec<Section> {
    let mut out = Vec::with_capacity((LEVELS as usize) * PIERS.len());
    for level in 1..=LEVELS {
        let t = taper(level);
        for &pier in &PIERS {
            let (w, d) = base_dims(pier);
            out.push(Section {
                level,
                pier:  pier.to_string(),
                w:     (w * t * 1000.0).round() / 1000.0,
                d:     (d * t * 1000.0).round() / 1000.0,
            });
        }
    }
    out
}

pub fn generate_forces() -> Vec<Force> {
    let mut out = Vec::with_capacity((LEVELS as usize) * PIERS.len() * COMBOS.len());
    for level in 1..=LEVELS {
        for &pier in &PIERS {
            for &combo in &COMBOS {
                let base = base_force(pier, combo);
                let f    = force_at_level(base, level, combo);
                out.push(Force {
                    level,
                    pier:  pier.to_string(),
                    combo: combo.to_string(),
                    force: (f * 100.0).round() / 100.0,
                });
            }
        }
    }
    out
}

/// Cross-table join: match section + force on (level, pier), compute stress
pub fn generate_stress_results() -> Vec<StressResult> {
    let sections = generate_sections();
    let forces   = generate_forces();

    // Build a quick lookup map for sections: key = (level, pier)
    use std::collections::HashMap;
    let section_map: HashMap<(u16, &str), &Section> = sections
        .iter()
        .map(|s| ((s.level, s.pier.as_str()), s))
        .collect();

    let mut out = Vec::with_capacity(forces.len());
    for f in &forces {
        if let Some(sec) = section_map.get(&(f.level, f.pier.as_str())) {
            let area   = sec.w * sec.d;
            let stress = if area > 0.0 { f.force / area } else { 0.0 };
            out.push(StressResult {
                level:  f.level,
                pier:   f.pier.clone(),
                combo:  f.combo.clone(),
                area:   (area   * 1000.0).round() / 1000.0,
                force:  f.force,
                stress: (stress * 100.0).round() / 100.0,
                id:     format!("{}_{}", f.pier, f.level),
            });
        }
    }
    out
}