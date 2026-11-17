use std::path::{Path, PathBuf};

fn config_dir() -> PathBuf {
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        let p = PathBuf::from(xdg);
        if p.is_absolute() {
            return p.join("lemonade-studio");
        }
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".config/lemonade-studio")
}

fn data_dir() -> PathBuf {
    if let Ok(xdg) = std::env::var("XDG_DATA_HOME") {
        let p = PathBuf::from(xdg);
        if p.is_absolute() {
            return p.join("lemonade-studio");
        }
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".local/share/lemonade-studio")
}

fn legacy_settings_candidates() -> Vec<PathBuf> {
    vec![
        // Historical in-tree location (non-portable, kept for migration only).
        PathBuf::from("/home/salah/00_Projects/lemonade-studio-rs/settings.json"),
        PathBuf::from("settings.json"),
    ]
}

/// Canonical user settings location. Prefers XDG config; migrates legacy files once.
pub fn settings_file() -> PathBuf {
    let canonical = config_dir().join("settings.json");
    if canonical.exists() {
        return canonical;
    }
    for legacy in legacy_settings_candidates() {
        if legacy.exists() && legacy != canonical {
            if let Some(parent) = canonical.parent() {
                std::fs::create_dir_all(parent).ok();
            }
            // Best-effort migration; keep the legacy file in place so nothing is lost.
            std::fs::copy(&legacy, &canonical).ok();
            return canonical;
        }
    }
    canonical
}

pub fn default_output_dir() -> PathBuf {
    data_dir().join("outputs")
}

pub fn cache_dir() -> PathBuf {
    std::env::temp_dir().join("lemonade-studio")
}

/// Repository-relative asset lookup that also works inside an AppDir.
pub fn resolve_asset(subpath: &str) -> PathBuf {
    let sub = Path::new(subpath);
    // 1. Next to the running executable (AppDir: ../share/lemonade-studio).
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let p = parent.join("../share/lemonade-studio").join(sub);
            if p.exists() {
                return p;
            }
        }
    }
    // 2. Development tree relative to this crate.
    let dev = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join(sub);
    if dev.exists() {
        return dev;
    }
    // 3. Current working directory fallback.
    let cwd = PathBuf::from("assets").join(sub);
    if cwd.exists() {
        return cwd;
    }
    dev
}

pub fn resolve_ai_script() -> Option<PathBuf> {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let p = dir.join("../share/lemonade-studio/scripts/ai_rmbg.py");
            if p.exists() {
                return Some(p);
            }
        }
    }
    if let Ok(appdir) = std::env::var("APPDIR") {
        let p = PathBuf::from(appdir).join("usr/share/lemonade-studio/scripts/ai_rmbg.py");
        if p.exists() {
            return Some(p);
        }
    }
    let dev = PathBuf::from(env!("CARGO_MANIFEST_DIR").to_string()).join("scripts/ai_rmbg.py");
    if dev.exists() {
        return Some(dev);
    }
    let rel = PathBuf::from("scripts/ai_rmbg.py");
    if rel.exists() {
        return Some(rel);
    }
    None
}

pub fn rmbg_model_candidates() -> Vec<PathBuf> {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    let home = PathBuf::from(home);
    vec![
        home.join("models/rmbg/rmbg-1.4.onnx"),
        home.join("models/rmbg/rmbg-2.0.onnx"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_file_is_absolute() {
        let p = settings_file();
        assert!(p.is_absolute() || p == PathBuf::from("settings.json"));
        assert_eq!(p.file_name().unwrap(), "settings.json");
    }

    #[test]
    fn default_output_dir_is_not_project_tree() {
        let p = default_output_dir();
        assert!(!p.starts_with("/home/salah/00_Projects"));
    }
}
