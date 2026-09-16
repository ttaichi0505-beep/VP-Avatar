//! Utilities and helper functions

use std::path::Path;

/// Validates a file path
pub fn validate_path<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    if path.as_os_str().is_empty() {
        return false;
    }
    true
}

/// Gets file extension
pub fn get_extension<P: AsRef<Path>>(path: P) -> Option<String> {
    path.as_ref()
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_string())
}

/// Checks if file has specific extension
pub fn has_extension<P: AsRef<Path>>(path: P, ext: &str) -> bool {
    get_extension(&path).map_or(false, |e| e.eq_ignore_ascii_case(ext))
}

/// Gets file stem (name without extension)
pub fn get_file_stem<P: AsRef<Path>>(path: P) -> Option<String> {
    path.as_ref()
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(|s| s.to_string())
}

/// Clamps a value between min and max
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_extension() {
        assert_eq!(get_extension("test.json"), Some("json".to_string()));
        assert!(has_extension("avatar.vrm", "vrm"));
    }

    #[test]
    fn test_file_stem() {
        assert_eq!(get_file_stem("avatar.json"), Some("avatar".to_string()));
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(0.5, 0.0, 1.0), 0.5);
        assert_eq!(clamp(-0.5, 0.0, 1.0), 0.0);
        assert_eq!(clamp(1.5, 0.0, 1.0), 1.0);
    }
}
