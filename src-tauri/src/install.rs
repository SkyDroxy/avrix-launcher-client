use crate::logger::{emit_app_log, error, info};
use crate::{
    metadata,
    models::{InstallFromUrlResult, ValidationMetadata},
    util::find_game_root,
};
use anyhow::{anyhow, Result};
use std::path::PathBuf;
use tauri::Emitter;

pub fn install_plugin_local(path: String, window: tauri::Window) -> Result<String> {
    let emit = |m: &str| {
        info("install", m);
        let _ = window.emit("plugin-install-log", m.to_string());
        let _ = emit_app_log(&window, crate::logger::Level::Info, "install", m);
    };
    emit("[LOCAL] Installation started");
    let src = PathBuf::from(&path);
    if !src.exists() {
        let msg = "Source file not found";
        error("install", msg);
        return Err(anyhow!(msg));
    }
    emit("Checking metadata.yml ...");
    if !metadata::is_valid_avrix_plugin(&src) {
        let msg = "Invalid or missing metadata.yml";
        error("install", msg);
        return Err(anyhow!(msg));
    }
    let base = std::env::current_dir()?;
    let game_root = find_game_root(&base).unwrap_or(base.clone());
    emit(&format!("Game root: {}", game_root.to_string_lossy()));
    let plugins_dir = crate::util::resolve_plugins_dir();
    std::fs::create_dir_all(&plugins_dir)?;
    emit(&format!("Plugins folder: {}", plugins_dir.to_string_lossy()));
    let file_name = src
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| anyhow!("Nom de fichier invalide"))?;
    let dest = plugins_dir.join(file_name);
    emit(&format!("Copy to {}", dest.to_string_lossy()));
    std::fs::copy(&src, &dest).map_err(|e| {
        error("install", &format!("Copy error: {}", e));
        anyhow!(e)
    })?;
    emit("Done");
    Ok(format!("Plugin installed: {}", dest.to_string_lossy()))
}

pub fn install_plugin_from_url(url: String, window: tauri::Window) -> Result<InstallFromUrlResult> {
    let emit = |m: &str| {
        info("install", m);
        let _ = window.emit("plugin-install-log", m.to_string());
        let _ = emit_app_log(&window, crate::logger::Level::Info, "install", m);
    };
    emit(&format!("[URL] Download: {}", url));
    const MAX_SIZE: u64 = 25 * 1024 * 1024;
    if let Ok(head) = minreq::head(&url).send() {
        if let Some(len) = head
            .headers
            .get("content-length")
            .and_then(|v| v.parse::<u64>().ok())
        {
            emit(&format!("HEAD size={} bytes", len));
            if len > MAX_SIZE {
                let msg = "Downloaded file exceeds maximum allowed size";
                error("install", msg);
                return Err(anyhow!(msg));
            }
        }
    }
    let resp = minreq::get(&url).send().map_err(|e| { error("install", &format!("Request error: {}", e)); anyhow!(e) })?;
    let code = resp.status_code;
    if code < 200 || code >= 300 {
        error("install", &format!("HTTP {}", code));
        return Err(anyhow!(format!("Download failed (status {})", code)));
    }
    let bytes = resp.as_bytes();
    if bytes.len() as u64 > MAX_SIZE {
        return Err(anyhow!("Fichier dépasse la taille maximale autorisée"));
    }
    emit(&format!("Received {} bytes", bytes.len()));
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let sha256 = hex::encode(hasher.finalize());
    emit(&format!("SHA-256: {}…", &sha256[..16]));
    let mut tmp_path = std::env::temp_dir();
    let file_name = url.split('/').last().unwrap_or("plugin.jar");
    let file_name = if file_name.ends_with(".jar") {
        file_name
    } else {
        "downloaded-plugin.jar"
    };
    tmp_path.push(file_name);
    emit(&format!("Writing temp file: {}", tmp_path.to_string_lossy()));
    std::fs::write(&tmp_path, bytes).map_err(|e| {
        error("install", &format!("Write error: {}", e));
        anyhow!(e)
    })?;
    let meta = metadata::extract_metadata_from_jar(&tmp_path).ok();
    if meta.is_none() {
        let msg = "metadata.yml not found in archive";
        error("install", msg);
        return Err(anyhow!(msg));
    }
    let base = std::env::current_dir()?;
    let game_root = find_game_root(&base).unwrap_or(base.clone());
    emit(&format!("Game root: {}", game_root.to_string_lossy()));
    let plugins_dir = crate::util::resolve_plugins_dir();
    std::fs::create_dir_all(&plugins_dir)?;
    let dest = plugins_dir.join(file_name);
    emit(&format!("Final copy to {}", dest.to_string_lossy()));
    std::fs::copy(&tmp_path, &dest).map_err(|e| {
        emit(&format!("Erreur copie: {}", e));
        anyhow!(e)
    })?;
    let size = bytes.len() as u64;
    emit("Done");
    Ok(InstallFromUrlResult {
        message: format!("Plugin downloaded and installed: {}", dest.to_string_lossy()),
        size,
        sha256,
        name: meta.as_ref().and_then(|m| m.name.clone()),
        version: meta.as_ref().and_then(|m| m.version.clone()),
        environment: meta.as_ref().and_then(|m| m.environment.clone()),
    })
}

pub fn validate_plugin_local(path: String) -> Result<ValidationMetadata> {
    let p = PathBuf::from(&path);
    if !p.exists() {
        return Ok(ValidationMetadata {
            valid: false,
            name: None,
            version: None,
            environment: None,
            size: 0,
            sha256: None,
            message: "Fichier introuvable".into(),
        });
    }
    if p.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.eq_ignore_ascii_case("jar"))
        .unwrap_or(false)
        == false
    {
        return Ok(ValidationMetadata {
            valid: false,
            name: None,
            version: None,
            environment: None,
            size: 0,
            sha256: None,
            message: "Extension non .jar".into(),
        });
    }
    let data = std::fs::read(&p)?;
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let sha256 = hex::encode(hasher.finalize());
    let size = data.len() as u64;
    let meta = metadata::extract_metadata_from_jar(&p).ok();
    if let Some(m) = &meta {
        return Ok(ValidationMetadata {
            valid: true,
            name: m.name.clone(),
            version: m.version.clone(),
            environment: m.environment.clone(),
            size,
            sha256: Some(sha256),
            message: "Valide".into(),
        });
    }
    Ok(ValidationMetadata {
        valid: false,
        name: None,
        version: None,
        environment: None,
        size,
        sha256: Some(sha256),
        message: "metadata.yml invalide".into(),
    })
}

pub fn validate_plugin_from_url(url: String) -> Result<ValidationMetadata> {
    const MAX_SIZE: u64 = 25 * 1024 * 1024;
    if let Ok(head) = minreq::head(&url).send() {
        if let Some(len) = head
            .headers
            .get("content-length")
            .and_then(|v| v.parse::<u64>().ok())
        {
            if len > MAX_SIZE {
                return Ok(ValidationMetadata {
                    valid: false,
                    name: None,
                    version: None,
                    environment: None,
                    size: len,
                    sha256: None,
                    message: "File too large".into(),
                });
            }
        }
    }
    let resp = minreq::get(&url).send()?;
    let code = resp.status_code;
    if code < 200 || code >= 300 {
        return Err(anyhow!(format!("HTTP status {}", code)));
    }
    let bytes = resp.as_bytes();
    if bytes.len() as u64 > MAX_SIZE {
        return Ok(ValidationMetadata {
            valid: false,
            name: None,
            version: None,
            environment: None,
            size: bytes.len() as u64,
            sha256: None,
            message: "File size exceeds limit".into(),
        });
    }
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let sha256 = hex::encode(hasher.finalize());
    let mut tmp = std::env::temp_dir();
    tmp.push("_validate_url_plugin.jar");
    std::fs::write(&tmp, bytes)?;
    let meta = metadata::extract_metadata_from_jar(&tmp).ok();
    if let Some(m) = &meta {
        return Ok(ValidationMetadata {
            valid: true,
            name: m.name.clone(),
            version: m.version.clone(),
            environment: m.environment.clone(),
            size: bytes.len() as u64,
            sha256: Some(sha256),
            message: "Valid".into(),
        });
    }
    Ok(ValidationMetadata {
        valid: false,
        name: None,
        version: None,
        environment: None,
        size: bytes.len() as u64,
        sha256: Some(sha256),
        message: "Invalid metadata.yml".into(),
    })
}
