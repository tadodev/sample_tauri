fn main() {
    tauri_build::build()
}

#[cfg(test)]
mod export_bindings {
    use super::*;
    
    #[test]
    fn export_typescript_bindings() {
        // Export to src/types/bindings/
        Section::export().unwrap();
        Force::export().unwrap();
        StressResult::export().unwrap();
        StressParams::export().unwrap();
        LoadFactors::export().unwrap();
    }
}