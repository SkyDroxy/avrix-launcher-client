use crate::logger::{emit_app_log, info, warn};
use anyhow::Result;
use std::io::Read;
use std::{fs, path::PathBuf};
use tauri::{Emitter, Window};
use zip::ZipArchive;

pub fn find_game_root(start: &PathBuf) -> Option<PathBuf> {
    let mut cur = start.clone();
    for _ in 0..6 {
        if contains_game_dirs(&cur) {
            return Some(cur);
        }
        if !cur.pop() {
            break;
        }
    }
    None
}

fn contains_game_dirs(base: &PathBuf) -> bool {
    ["zombie", "se", "fmod", "javax"]
        .iter()
        .all(|d| base.join(d).is_dir())
}

pub fn parse_name_version_simple(file_name: &str) -> Option<(String, String)> {
    let base = file_name.trim_end_matches(".jar");
    let re = regex::Regex::new(r"(?i)^(.+?)[-_]v?(\d+\.\d+(?:\.\d+)?)(?:[-_].*)?$").ok()?;
    if let Some(caps) = re.captures(base) {
        let disp = caps.get(1)?.as_str().to_string();
        let ver = caps.get(2)?.as_str().to_string();
        return Some((disp, ver));
    }
    None
}

pub fn resolve_plugins_dir() -> PathBuf {
    let base = std::env::current_dir().unwrap_or_else(|_| std::env::temp_dir());
    if let Some(root) = find_game_root(&base) {
        return root.join("plugins");
    }
    if let Ok(custom) = std::env::var("AVRIX_PLUGINS_DIR") {
        let p = PathBuf::from(custom);
        if p.is_absolute() {
            return p;
        }
    }
    let mut p = std::env::temp_dir();
    p.push("avrix-dev-plugins");
    p
}

pub fn scan_plugins(window: &Window) -> Result<crate::models::PluginsResult> {
    use crate::{
        metadata,
        models::{PluginEntry, PluginsResult},
    };
    let base = std::env::current_dir()?;
    let game_root = find_game_root(&base).unwrap_or(base.clone());
    let plugins_dir = resolve_plugins_dir();
    let mut out: Vec<PluginEntry> = Vec::new();

    let emit = |msg: &str| {
        info("scan", msg);
        let _ = window.emit("plugin-scan-log", msg.to_string());
        let _ = emit_app_log(&window, crate::logger::Level::Info, "scan", msg);
    };

    emit(&format!(
        "Searching for Avrix-Core.jar from current directory: {}",
        base.to_string_lossy()
    ));
    if game_root != base {
        emit(&format!(
            "Game root detected: {}",
            game_root.to_string_lossy()
        ));
    }
    emit(&format!("Plugins directory: {}", plugins_dir.to_string_lossy()));

    let core_jar = base.join("Avrix-Core.jar");
    emit(&format!(
        "Trying to load core: {}",
        core_jar.to_string_lossy()
    ));
    if core_jar.exists() {
    emit("Avrix-Core.jar found, extracting metadata...");
        if let Ok(meta_fs) = fs::metadata(&core_jar) {
            if let Ok(raw) = metadata::extract_metadata_from_jar(&core_jar) {
                let modified = meta_fs
                    .modified()
                    .ok()
                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                out.push(PluginEntry {
                    name: "Avrix-Core.jar".into(),
                    size_kb: (meta_fs.len() / 1024).max(1),
                    modified,
                    display_name: raw.name.clone(),
                    version: raw.version.clone(),
                    environment: raw.environment.clone(),
                    author: raw.author.clone(),
                    license: raw.license.clone(),
                    id: raw.id.clone().or(Some("avrix-core".into())),
                    description: raw.description.clone(),
                    dependencies: raw.dependencies.clone(),
                    image: raw.image.clone(),
                    image_url: raw.image_url.clone(),
                    internal: Some(false),
                    parent_id: None,
                });
            } else {
                warn("scan", "Core metadata not found (missing or invalid metadata.yml)");
            }
        }

        if let Ok(file) = std::fs::File::open(&core_jar) {
            if let Ok(mut zip) = ZipArchive::new(file) {
                emit("Scanning embedded internal-plugins...");
                for i in 0..zip.len() {
                    if let Ok(mut zf) = zip.by_index(i) {
                        let name = zf.name().to_string();
                        if name.starts_with("internal-plugins/") && name.ends_with(".yml") {
                            emit(&format!(" - Read: {}", name));
                            let mut contents = String::new();
                            if zf.read_to_string(&mut contents).is_ok() {
                                if let Ok(raw_ip) =
                                    serde_yaml::from_str::<crate::models::RawMetadata>(&contents)
                                {
                                    if let Some(id) = raw_ip.id.clone() {
                                        if !out.iter().any(|p| {
                                            p.id.as_ref().map(|x| x == &id).unwrap_or(false)
                                        }) {
                                            emit(&format!("   -> Register internal plugin: {}", id));
                                            out.push(PluginEntry {
                                                name: format!("{} (internal)", id),
                                                size_kb: 0,
                                                modified: std::time::SystemTime::now()
                                                    .duration_since(std::time::UNIX_EPOCH)
                                                    .map(|d| d.as_secs())
                                                    .unwrap_or(0),
                                                display_name: raw_ip.name.clone(),
                                                version: raw_ip.version.clone(),
                                                environment: raw_ip.environment.clone(),
                                                author: raw_ip.author.clone(),
                                                license: raw_ip.license.clone(),
                                                id: Some(id.clone()),
                                                description: raw_ip.description.clone(),
                                                dependencies: raw_ip.dependencies.clone(),
                                                image: raw_ip.image.clone(),
                                                image_url: raw_ip.image_url.clone(),
                                                internal: Some(true),
                                                parent_id: raw_ip
                                                    .parent
                                                    .clone()
                                                    .or(Some("avrix-core".into())),
                                            });
                                            emit(&format!("   -> Internal plugin registered: {}", id));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if plugins_dir.exists() {
    emit("Scanning external plugins (.jar)...");
        for entry in fs::read_dir(&plugins_dir)? {
            let e = entry?;
            let p = e.path();
            if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
                if name.to_lowercase().ends_with(".jar")
                    && !name.to_lowercase().contains("launcher")
                    && !name.starts_with("Avrix-Core")
                {
                    emit(&format!(" - Detected plugin: {}", name));
                    let meta = fs::metadata(&p)?;
                    let modified = meta
                        .modified()
                        .ok()
                        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                        .map(|d| d.as_secs())
                        .unwrap_or(0);
                    let mut entry = PluginEntry {
                        name: name.to_string(),
                        size_kb: (meta.len() / 1024).max(1),
                        modified,
                        ..Default::default()
                    };
                    if let Ok(m) = metadata::extract_metadata_from_jar(&p) {
                        emit(&format!("   -> Extracted metadata for {}", name));
                        entry.display_name = m.name.clone();
                        entry.version = m.version.clone();
                        entry.environment = m.environment.clone();
                        entry.author = m.author.clone();
                        entry.license = m.license.clone();
                        entry.id = m.id.clone();
                        entry.description = m.description.clone();
                        entry.dependencies = m.dependencies.clone();
                        entry.image = m.image.clone();
                        entry.image_url = m.image_url.clone();
                        entry.internal = m.internal;
                        entry.parent_id = m.parent.clone();
                    } else if let Some((disp, ver)) = parse_name_version_simple(name) {
                        entry.display_name = Some(disp);
                        entry.version = Some(ver);
                    }
                    out.push(entry);
                }
            }
        }
    } else {
    warn("scan", "No plugins folder found. You may create it manually later.");
    }

    emit(&format!("Total detected plugins: {}", out.len()));

    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(PluginsResult {
        plugins: out,
        dir: plugins_dir.to_string_lossy().to_string(),
    })
}

pub fn delete_plugin(name: String) -> Result<String> {
    let plugins_dir = resolve_plugins_dir();
    if !plugins_dir.exists() {
        return Ok(format!(
            "Plugins directory not found: {}",
            plugins_dir.to_string_lossy()
        ));
    }
    if name.contains('/') || name.contains('\\') {
        anyhow::bail!("Invalid name");
    }
    let target = plugins_dir.join(&name);
    if !target.exists() {
        anyhow::bail!("Plugin not found");
    }
    if target
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.eq_ignore_ascii_case("jar"))
        .unwrap_or(false)
        == false
    {
        anyhow::bail!("File is not a .jar");
    }
    std::fs::remove_file(&target)?;
    Ok(format!(
        "Deleted: {}",
        target.file_name().and_then(|n| n.to_str()).unwrap_or("?")
    ))
}
