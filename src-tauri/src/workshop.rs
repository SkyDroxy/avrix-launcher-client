use crate::logger::{emit_app_log, info, warn};
use crate::{metadata, models::WorkshopScanResult, util::find_game_root};
use anyhow::Result;
use std::{collections::HashSet, fs, path::PathBuf};
use tauri::Emitter;

pub fn scan_workshop(window: tauri::Window) -> Result<WorkshopScanResult> {
    let emit = |m: &str| {
        info("workshop", m);
        let _ = window.emit("workshop-scan-log", m.to_string());
        let _ = emit_app_log(&window, crate::logger::Level::Info, "workshop", m);
    };
    let mut roots_raw: Vec<PathBuf> = Vec::new();

    if let Ok(override_path) = std::env::var("PZ_WORKSHOP_ROOT") {
        roots_raw.push(PathBuf::from(override_path));
    }
    if let Ok(override_path) = std::env::var("AVRIX_WORKSHOP_ROOT") {
        roots_raw.push(PathBuf::from(override_path));
    }
    if let Ok(base) = std::env::current_dir() {
        if let Some(game_root) = find_game_root(&base) {
            if let Some(steamapps) = game_root.parent().and_then(|p| p.parent()) {
                let candidate = steamapps.join("workshop").join("content/108600");
                roots_raw.push(candidate);
            }
        }
    }
    if let Ok(home) = std::env::var("USERPROFILE") {
        roots_raw.push(
            PathBuf::from(home).join("AppData/Local/Steam/steamapps/workshop/content/108600"),
        );
    }
    if let Ok(steam) = std::env::var("STEAM_LIBRARY") {
        let p = PathBuf::from(&steam);
        let candidate = if p.ends_with("steamapps") {
            p.join("workshop/content/108600")
        } else {
            p.join("steamapps/workshop/content/108600")
        };
        roots_raw.push(candidate);
    }
    #[cfg(target_os = "windows")]
    {
        for drive_letter in 'C'..='Z' {
            let candidate = PathBuf::from(format!(
                "{}:/Program Files/Steam/steamapps/workshop/content/108600",
                drive_letter
            ));
            if candidate.exists() {
                roots_raw.push(candidate);
            }
        }
    }
    let mut roots: Vec<PathBuf> = Vec::new();
    for r in roots_raw.into_iter() {
        if let Ok(canon) = r.canonicalize() {
            roots.push(canon);
        } else {
            roots.push(r);
        }
    }
    let mut seen_root = HashSet::new();
    roots.retain(|p| seen_root.insert(p.to_string_lossy().to_string()));

    let roots_list = roots
        .iter()
        .map(|p| p.to_string_lossy())
        .collect::<Vec<_>>()
        .join(" | ");
    if roots.is_empty() {
        let msg = "Workshop scan started but no roots detected (check environment variables or Steam installation)";
        warn("workshop", msg);
        let _ = emit_app_log(&window, crate::logger::Level::Warn, "workshop", msg);
        emit("Workshop scan: no roots");
    } else {
        emit(&format!("Workshop scan started in: {}", roots_list));
        if roots.len() > 1 { emit(&format!("Roots (absolute): {}", roots_list)); }
    }

    let mut seen: HashSet<String> = HashSet::new();
    let mut found: Vec<String> = Vec::new();
    for root in roots.iter() {
        if !root.exists() {
            emit(&format!("Ignore (missing): {}", root.to_string_lossy()));
            continue;
        }
        emit(&format!("Scanning root: {}", root.to_string_lossy()));
        let mut stack: Vec<PathBuf> = vec![root.clone()];
        while let Some(dir) = stack.pop() {
            if let Ok(rd) = fs::read_dir(&dir) {
                for entry in rd.flatten() {
                    let p = entry.path();
                    if p.is_dir() {
                        if p.components().count() - root.components().count() <= 4 {
                            stack.push(p);
                        }
                    } else if p
                        .extension()
                        .and_then(|s| s.to_str())
                        .map(|e| e.eq_ignore_ascii_case("jar"))
                        .unwrap_or(false)
                    {
                        if metadata::is_valid_avrix_plugin(&p) {
                            let s = p.to_string_lossy().to_string();
                            if seen.insert(s.clone()) {
                                found.push(s.clone());
                                emit(&format!("Found: {}", s));
                            }
                        }
                    }
                }
            }
        }
    }
    if found.is_empty() {
        emit(&format!(
            "Scan finished. 0 valid plugin | roots= {}",
            roots
                .iter()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>()
                .join(" | ")
        ));
    } else {
        emit(&format!("Scan finished. {} valid plugin(s)", found.len()));
    }
    Ok(WorkshopScanResult { found })
}
