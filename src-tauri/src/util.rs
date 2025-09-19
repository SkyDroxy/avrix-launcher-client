use crate::logger::{emit_app_log, info, warn};
use anyhow::Result;
use base64::Engine as _;
use std::io::Read;
use std::{fs, path::PathBuf};
use tauri::{Emitter, Window};
use zip::ZipArchive;

// Allowed image size range (in bytes)
const MAX_IMAGE_BYTES: usize = 5 * 1024 * 1024; // 5 MiB

fn guess_mime_from_ext(name: &str) -> &'static str {
    let lower = name.to_lowercase();
    if lower.ends_with(".png") {
        "image/png"
    } else if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
        "image/jpeg"
    } else if lower.ends_with(".gif") {
        "image/gif"
    } else if lower.ends_with(".webp") {
        "image/webp"
    } else if lower.ends_with(".svg") {
        // Many UIs won't render data:image/svg+xml without proper encoding; prefer raster.
        "image/svg+xml"
    } else {
        "application/octet-stream"
    }
}

fn data_url_from_bytes(name: &str, bytes: &[u8]) -> String {
    let mime = guess_mime_from_ext(name);
    let b64 = base64::engine::general_purpose::STANDARD.encode(bytes);
    format!("data:{};base64,{}", mime, b64)
}

fn is_url(s: &str) -> bool {
    let sl = s.to_lowercase();
    sl.starts_with("http://") || sl.starts_with("https://")
}

fn is_data_url(s: &str) -> bool {
    s.starts_with("data:")
}

fn is_size_allowed(bytes: usize) -> bool {
    bytes <= MAX_IMAGE_BYTES
}

// Estimate decoded payload size of a base64 data URL. Returns None if not base64 or malformed.
fn data_url_payload_len_bytes(url: &str) -> Option<usize> {
    let lower = url.to_ascii_lowercase();
    if !lower.starts_with("data:") {
        return None;
    }
    let comma_idx = url.find(',')?;
    let header = &lower[..comma_idx];
    if !header.contains(";base64") {
        return None;
    }
    let b64 = &url[comma_idx + 1..];
    let len = b64.len();
    if len == 0 {
        return Some(0);
    }
    if len % 4 != 0 {
        return None;
    }
    let padding = b64.chars().rev().take_while(|c| *c == '=').count();
    Some(len / 4 * 3 - padding)
}

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
    emit(&format!(
        "Plugins directory: {}",
        plugins_dir.to_string_lossy()
    ));

    let core_jar = base.join("Avrix-Core.jar");
    emit(&format!(
        "Trying to load core: {}",
        core_jar.to_string_lossy()
    ));
    if core_jar.exists() {
        emit("Avrix-Core.jar found, extracting metadata...");
        if let Ok(meta_fs) = fs::metadata(&core_jar) {
            if let Ok((raw, core_base)) = metadata::extract_metadata_with_base_from_jar(&core_jar) {
                let modified = meta_fs
                    .modified()
                    .ok()
                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                emit(&format!(
                    "[core] image metadata: image={:?} imageUrl={:?}",
                    raw.image, raw.image_url
                ));
                let mut image_data: Option<String> = None;
                if let Some(img_path) = raw.image.as_deref() {
                    if is_data_url(img_path) {
                        match data_url_payload_len_bytes(img_path) {
                            Some(sz) if is_size_allowed(sz) => {
                                emit(&format!(
                                    "[core] image is data URL ({} bytes) — allowed ({} bytes)",
                                    sz, MAX_IMAGE_BYTES
                                ));
                                image_data = Some(img_path.to_string());
                            }
                            Some(sz) => {
                                emit(&format!("[core] image is data URL ({} bytes) — rejected (allowed {} bytes)", sz, MAX_IMAGE_BYTES));
                            }
                            None => {
                                emit(
                                    "[core] image is data URL but not base64/malformed — rejecting",
                                );
                            }
                        }
                    } else if !is_url(img_path) {
                        let base = core_base.as_deref().unwrap_or("");
                        let img_rel = img_path.trim_start_matches('/');
                        let full = if base.is_empty() {
                            img_rel.to_string()
                        } else {
                            format!("{}/{}", base.trim_end_matches('/'), img_rel)
                        };
                        emit(&format!(
                            "[core] image is a JAR path: {} — baseDir='{}' fullCandidate='{}' — scanning entries",
                            img_path, base, full
                        ));
                        if let Ok(file) = std::fs::File::open(&core_jar) {
                            if let Ok(mut zip) = ZipArchive::new(file) {
                                for zi in 0..zip.len() {
                                    if let Ok(mut zf) = zip.by_index(zi) {
                                        let name = zf.name().to_string();
                                        if name.ends_with('/') {
                                            continue;
                                        }
                                        // Prefer exact match against full candidate, then fallback
                                        if name.eq_ignore_ascii_case(&full)
                                            || name.eq_ignore_ascii_case(img_rel)
                                            || name
                                                .to_lowercase()
                                                .ends_with(&img_rel.to_lowercase())
                                        {
                                            let mut buf = Vec::new();
                                            if std::io::Read::read_to_end(&mut zf, &mut buf).is_ok()
                                            {
                                                let size = buf.len();
                                                if is_size_allowed(size) {
                                                    let mime = guess_mime_from_ext(&name);
                                                    let data_url = data_url_from_bytes(&name, &buf);
                                                    emit(&format!(
                                                        "[core] image resolved: {} ({} bytes, mime {}, dataUrlLen {}) — allowed ({} bytes)",
                                                        name,
                                                        size,
                                                        mime,
                                                        data_url.len(),
                                                        MAX_IMAGE_BYTES
                                                    ));
                                                    image_data = Some(data_url);
                                                } else {
                                                    emit(&format!(
                                                        "[core] image resolved but rejected due to size: {} ({} bytes; allowed {} bytes)",
                                                        name,
                                                        size,
                                                        MAX_IMAGE_BYTES
                                                    ));
                                                }
                                            }
                                            break;
                                        }
                                    }
                                }
                                if image_data.is_none() {
                                    emit(&format!("[core] image path not found in JAR: base='{}' rel='{}' full='{}'", core_base.unwrap_or_default(), img_path, full));
                                }
                            }
                        }
                    } else {
                        emit(&format!("[core] image is URL: {} (size cannot be pre-validated; will use imageUrl)", img_path));
                    }
                }
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
                    image: image_data,
                    image_url: raw
                        .image
                        .as_ref()
                        .filter(|s| is_url(s))
                        .cloned()
                        .or_else(|| {
                            if let Some(u) = raw.image_url.clone() {
                                emit(&format!("[core] using imageUrl from metadata: {}", u));
                                Some(u)
                            } else {
                                None
                            }
                        }),
                    internal: Some(false),
                    parent_id: None,
                });
            } else {
                warn(
                    "scan",
                    "Core metadata not found (missing or invalid metadata.yml)",
                );
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
                                            emit(&format!(
                                                "   -> Register internal plugin: {}",
                                                id
                                            ));
                                            // Try to resolve internal image (embedded reference or URL)
                                            let mut image: Option<String> = None;
                                            let mut image_url: Option<String> =
                                                raw_ip.image_url.clone();
                                            emit(&format!(
                                                "[internal:{}] image metadata: image={:?} imageUrl={:?}",
                                                id, raw_ip.image, raw_ip.image_url
                                            ));
                                            if let Some(img_ref) = raw_ip.image.as_deref() {
                                                if is_data_url(img_ref) {
                                                    match data_url_payload_len_bytes(img_ref) {
                                                        Some(sz) if is_size_allowed(sz) => {
                                                            emit(&format!("[internal:{}] image is data URL ({} bytes) — allowed ({} bytes)", id, sz, MAX_IMAGE_BYTES));
                                                            image = Some(img_ref.to_string());
                                                        }
                                                        Some(sz) => {
                                                            emit(&format!("[internal:{}] image is data URL ({} bytes) — rejected (allowed {} bytes)", id, sz, MAX_IMAGE_BYTES));
                                                        }
                                                        None => {
                                                            emit(&format!("[internal:{}] image is data URL but not base64/malformed — rejecting", id));
                                                        }
                                                    }
                                                } else if is_url(img_ref) {
                                                    emit(&format!(
                                                        "[internal:{}] image is URL: {}",
                                                        id, img_ref
                                                    ));
                                                    image_url = Some(img_ref.to_string());
                                                } else {
                                                    // Compute base from YAML path where it was found
                                                    let base = name
                                                        .rfind('/')
                                                        .map(|idx| &name[..idx])
                                                        .unwrap_or("");
                                                    let rel = img_ref.trim_start_matches('/');
                                                    let full = if base.is_empty() {
                                                        rel.to_string()
                                                    } else {
                                                        format!(
                                                            "{}/{}",
                                                            base.trim_end_matches('/'),
                                                            rel
                                                        )
                                                    };
                                                    emit(&format!(
                                                        "[internal:{}] image is a JAR path: {} — baseDir='{}' fullCandidate='{}' — scanning core zip",
                                                        id, img_ref, base, full
                                                    ));
                                                    if let Ok(file2) =
                                                        std::fs::File::open(&core_jar)
                                                    {
                                                        if let Ok(mut zip2) = ZipArchive::new(file2)
                                                        {
                                                            for zj in 0..zip2.len() {
                                                                if let Ok(mut zimg) =
                                                                    zip2.by_index(zj)
                                                                {
                                                                    let zname =
                                                                        zimg.name().to_string();
                                                                    if zname.ends_with('/') {
                                                                        continue;
                                                                    }
                                                                    if zname
                                                                        .eq_ignore_ascii_case(&full)
                                                                        || zname
                                                                            .eq_ignore_ascii_case(
                                                                                rel,
                                                                            )
                                                                        || zname
                                                                            .to_lowercase()
                                                                            .ends_with(
                                                                                &rel.to_lowercase(),
                                                                            )
                                                                    {
                                                                        let mut buf = Vec::new();
                                                                        if std::io::Read::read_to_end(&mut zimg, &mut buf).is_ok() {
                                                                            let size = buf.len();
                                                                            if is_size_allowed(size) {
                                                                                let mime = guess_mime_from_ext(&zname);
                                                                                let data_url = data_url_from_bytes(&zname, &buf);
                                                                                emit(&format!(
                                                                                    "[internal:{}] image resolved: {} ({} bytes, mime {}, dataUrlLen {}) — allowed ({} bytes)",
                                                                                    id,
                                                                                    zname,
                                                                                    size,
                                                                                    mime,
                                                                                    data_url.len(),
                                                                                    MAX_IMAGE_BYTES
                                                                                ));
                                                                                image = Some(data_url);
                                                                            } else {
                                                                                emit(&format!(
                                                                                    "[internal:{}] image resolved but rejected due to size: {} ({} bytes; allowed {} bytes)",
                                                                                    id,
                                                                                    zname,
                                                                                    size,
                                                                                    MAX_IMAGE_BYTES
                                                                                ));
                                                                            }
                                                                        }
                                                                        break;
                                                                    }
                                                                }
                                                            }
                                                            if image.is_none() {
                                                                emit(&format!(
                                                                    "[internal:{}] image path not found in core JAR: base='{}' rel='{}' full='{}'",
                                                                    id, base, img_ref, full
                                                                ));
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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
                                                image,
                                                image_url,
                                                internal: Some(true),
                                                parent_id: raw_ip
                                                    .parent
                                                    .clone()
                                                    .or(Some("avrix-core".into())),
                                            });
                                            emit(&format!(
                                                "   -> Internal plugin registered: {}",
                                                id
                                            ));
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
                    if let Ok((m, base)) = metadata::extract_metadata_with_base_from_jar(&p) {
                        emit(&format!("   -> Extracted metadata for {}", name));
                        entry.display_name = m.name.clone();
                        entry.version = m.version.clone();
                        entry.environment = m.environment.clone();
                        entry.author = m.author.clone();
                        entry.license = m.license.clone();
                        entry.id = m.id.clone();
                        entry.description = m.description.clone();
                        entry.dependencies = m.dependencies.clone();
                        emit(&format!(
                            "   -> [{}] image metadata: image={:?} imageUrl={:?} baseDir={:?}",
                            name, m.image, m.image_url, base
                        ));
                        // If image is a path inside the JAR, extract it as data URL or pass URL/data URL
                        if let Some(img_ref) = &m.image {
                            if is_data_url(img_ref) {
                                match data_url_payload_len_bytes(img_ref) {
                                    Some(sz) if is_size_allowed(sz) => {
                                        emit(&format!("   -> [{}] image is data URL ({} bytes) — allowed ({} bytes)", name, sz, MAX_IMAGE_BYTES));
                                        entry.image = Some(img_ref.clone());
                                    }
                                    Some(sz) => {
                                        emit(&format!("   -> [{}] image is data URL ({} bytes) — rejected (allowed {} bytes)", name, sz, MAX_IMAGE_BYTES));
                                    }
                                    None => {
                                        emit(&format!("   -> [{}] image is data URL but not base64/malformed — rejecting", name));
                                    }
                                }
                            } else if is_url(img_ref) {
                                emit(&format!("   -> [{}] image is URL: {}", name, img_ref));
                                entry.image_url = Some(img_ref.clone());
                            } else {
                                if let Ok(file) = std::fs::File::open(&p) {
                                    if let Ok(mut zip) = ZipArchive::new(file) {
                                        let base_s = base.unwrap_or_default();
                                        let rel = img_ref.trim_start_matches('/');
                                        let full = if base_s.is_empty() {
                                            rel.to_string()
                                        } else {
                                            format!("{}/{}", base_s.trim_end_matches('/'), rel)
                                        };
                                        emit(&format!(
                                            "   -> [{}] image is a JAR path: {} — baseDir='{}' fullCandidate='{}' — scanning entries",
                                            name, img_ref, base_s, full
                                        ));
                                        for zi in 0..zip.len() {
                                            if let Ok(mut zf) = zip.by_index(zi) {
                                                let zname = zf.name().to_string();
                                                if zname.ends_with('/') {
                                                    continue;
                                                }
                                                if zname.eq_ignore_ascii_case(&full)
                                                    || zname.eq_ignore_ascii_case(rel)
                                                    || zname
                                                        .to_lowercase()
                                                        .ends_with(&rel.to_lowercase())
                                                {
                                                    let mut buf = Vec::new();
                                                    if std::io::Read::read_to_end(&mut zf, &mut buf)
                                                        .is_ok()
                                                    {
                                                        let size = buf.len();
                                                        if is_size_allowed(size) {
                                                            let mime = guess_mime_from_ext(&zname);
                                                            let data_url =
                                                                data_url_from_bytes(&zname, &buf);
                                                            emit(&format!(
                                                                "   -> [{}] image resolved: {} ({} bytes, mime {}, dataUrlLen {}) — allowed ({} bytes)",
                                                                name,
                                                                zname,
                                                                size,
                                                                mime,
                                                                data_url.len(),
                                                                MAX_IMAGE_BYTES
                                                            ));
                                                            entry.image = Some(data_url);
                                                        } else {
                                                            emit(&format!(
                                                                "   -> [{}] image resolved but rejected due to size: {} ({} bytes; allowed {} bytes)",
                                                                name,
                                                                zname,
                                                                size,
                                                                MAX_IMAGE_BYTES
                                                            ));
                                                        }
                                                    }
                                                    break;
                                                }
                                            }
                                        }
                                        if entry.image.is_none() {
                                            emit(&format!(
                                                "   -> [{}] image path not found in JAR: base='{}' rel='{}' full='{}'",
                                                name, base_s, img_ref, full
                                            ));
                                        }
                                    }
                                }
                            }
                        } else {
                            emit(&format!("   -> [{}] no image in metadata", name));
                            entry.image = None;
                        }
                        if entry.image_url.is_none() {
                            if let Some(u) = m.image_url.clone() {
                                emit(&format!(
                                    "   -> [{}] using imageUrl from metadata: {}",
                                    name, u
                                ));
                                entry.image_url = Some(u);
                            }
                        }
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
        warn(
            "scan",
            "No plugins folder found. You may create it manually later.",
        );
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
