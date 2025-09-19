use crate::logger::info;
use crate::models::{AvailableVersion, VersionEntry, VersionsResult};
use crate::util::find_game_root;
use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter, Manager, Window};
use tauri_plugin_store::StoreExt;
use zip::ZipArchive;

const STORE_SELECTED_KEY: &str = "selectedVersionId";

fn versions_root() -> Result<PathBuf> {
    // Prefer resolving from the launcher executable directory
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or(std::env::current_dir()?);

    // Try to find the game root starting from the exe dir
    if let Some(gr) = find_game_root(&exe_dir) {
        return Ok(gr.join("avrix").join("versions"));
    }

    // Allow an explicit absolute override
    if let Ok(custom) = std::env::var("AVRIX_VERSIONS_DIR") {
        let p = PathBuf::from(custom);
        if p.is_absolute() {
            return Ok(p);
        }
    }

    // Fallback: keep versions next to the launcher (within the exe directory)
    Ok(exe_dir.join("versions"))
}

fn read_dir_size_kb(path: &Path) -> u64 {
    fn walk(p: &Path, sum: &mut u64) {
        if let Ok(meta) = fs::metadata(p) {
            if meta.is_file() {
                *sum += meta.len() / 1024;
            }
        }
        if let Ok(rd) = fs::read_dir(p) {
            for e in rd.flatten() {
                let cp = e.path();
                if cp.is_dir() {
                    walk(&cp, sum);
                } else if let Ok(m) = fs::metadata(&cp) {
                    *sum += m.len() / 1024;
                }
            }
        }
    }
    let mut sum = 0u64;
    walk(path, &mut sum);
    sum.max(1)
}

fn core_jar_in(dir: &Path) -> Option<PathBuf> {
    let p = dir.join("Avrix-Core.jar");
    if p.exists() {
        Some(p)
    } else {
        None
    }
}

fn has_local_jre(dir: &Path) -> bool {
    let j = dir.join("jre");
    j.join(if cfg!(windows) {
        "bin/javaw.exe"
    } else {
        "bin/java"
    })
    .exists()
}

fn dir_modified_secs(dir: &Path) -> u64 {
    fs::metadata(dir)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn load_selected_id(app: &AppHandle) -> Option<String> {
    let exe_dir = std::env::current_exe().ok()?.parent()?.to_path_buf();
    let path = exe_dir.join("avrix-settings.json");
    let store = app.store(&path).ok()?;
    store
        .get(STORE_SELECTED_KEY)
        .and_then(|v| v.as_str().map(|s| s.to_string()))
}

fn save_selected_id(app: &AppHandle, id: Option<String>) -> Result<()> {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .context("Cannot determine executable directory")?;
    let path = exe_dir.join("avrix-settings.json");
    let store = app.store(&path).map_err(|e| anyhow!(e.to_string()))?;
    if let Some(id) = id {
        store.set(STORE_SELECTED_KEY, id.as_str());
    } else {
        store.delete(STORE_SELECTED_KEY);
    }
    store.save().map_err(|e| anyhow!(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn list_versions(window: Window) -> Result<VersionsResult, String> {
    info("versions", "list_versions invoked");
    let root = versions_root().map_err(|e| e.to_string())?;
    let _ = fs::create_dir_all(&root);
    let mut entries: Vec<VersionEntry> = Vec::new();
    if let Ok(rd) = fs::read_dir(&root) {
        for e in rd.flatten() {
            let dir = e.path();
            if !dir.is_dir() {
                continue;
            }
            let id = e.file_name().to_string_lossy().to_string();
            let has_jre = has_local_jre(&dir);
            let modified = dir_modified_secs(&dir);
            let size_kb = read_dir_size_kb(&dir);
            // Try to detect version from Avrix-Core.jar metadata
            let mut version: Option<String> = None;
            let mut display_name: Option<String> = None;
            if let Some(core) = core_jar_in(&dir) {
                if let Ok((raw, _)) = crate::metadata::extract_metadata_with_base_from_jar(&core) {
                    version = raw.version.clone();
                    display_name = raw.name.clone();
                }
            }
            entries.push(VersionEntry {
                id,
                version,
                display_name,
                dir: dir.to_string_lossy().to_string(),
                has_jre,
                modified,
                size_kb,
            });
        }
    }
    // Sort by id asc
    entries.sort_by(|a, b| a.id.cmp(&b.id));
    let app = window.app_handle();
    let selected_id = load_selected_id(&app);
    let res = VersionsResult {
        versions: entries,
        root: root.to_string_lossy().to_string(),
        selected_id,
    };
    let _ = window.emit("versions-log", "Listed versions".to_string());
    Ok(res).map_err(|e: String| e)
}

fn unzip_to(bytes: &[u8], dest: &Path) -> Result<()> {
    let reader = std::io::Cursor::new(bytes);
    let mut zip = ZipArchive::new(reader)?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let out_path = dest.join(file.mangled_name());
        if file.name().ends_with('/') {
            fs::create_dir_all(&out_path)?;
        } else {
            if let Some(p) = out_path.parent() {
                fs::create_dir_all(p)?;
            }
            let mut out = fs::File::create(&out_path)?;
            std::io::copy(&mut file, &mut out)?;
        }
    }
    Ok(())
}

fn normalize_version_id<S: AsRef<str>>(v: S) -> String {
    let raw = v.as_ref().trim();
    let ver = raw.trim_start_matches(|c: char| c == 'v' || c == 'V');
    format!("v{}", ver)
}

fn detect_version_from_jar(path: &Path) -> Option<String> {
    // Try metadata.yml first
    if let Ok(meta) = crate::metadata::extract_metadata_from_jar(&path.to_path_buf()) {
        if let Some(ver) = meta.version {
            return Some(ver);
        }
    }
    // Fallback: parse from filename
    let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
    if let Some((_, ver)) = crate::util::parse_name_version_simple(name) {
        return Some(ver);
    }
    None
}

fn detect_version_from_dir(dir: &Path) -> Option<String> {
    // Try Avrix-Core.jar at root
    if let Some(core) = core_jar_in(dir) {
        if let Some(ver) = detect_version_from_jar(&core) {
            return Some(ver);
        }
    }
    // Else scan for any *.jar that looks like Avrix-Core*
    if let Ok(rd) = fs::read_dir(dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_file() {
                if let Some(name) = p.file_name().and_then(|s| s.to_str()) {
                    if name.ends_with(".jar") && name.to_ascii_lowercase().starts_with("avrix-core")
                    {
                        if let Some(ver) = detect_version_from_jar(&p) {
                            return Some(ver);
                        }
                    }
                }
            } else if p.is_dir() {
                if let Some(ver) = detect_version_from_dir(&p) {
                    return Some(ver);
                }
            }
        }
    }
    None
}

// Manifest support (MinIO or any HTTP URL)
#[derive(serde::Deserialize)]
struct ManifestVersion {
    #[allow(dead_code)]
    tag: Option<String>,
    version: String,
    #[serde(rename = "coreUrl")]
    core_url: String,
    #[serde(rename = "jreUrl")]
    jre_url: Option<String>,
    #[serde(rename = "publishedAt")]
    published_at: Option<String>,
}
#[derive(serde::Deserialize)]
struct Manifest {
    #[allow(dead_code)]
    latest: Option<String>,
    versions: Vec<ManifestVersion>,
}

fn http_get(url: &str) -> Result<Vec<u8>> {
    let mut req = minreq::get(url);
    req = req.with_header("User-Agent", "AvrixLauncher/1.0");
    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        if !token.is_empty() {
            req = req.with_header("Authorization", format!("Bearer {}", token));
        }
    }
    let resp = req.send()?;
    if resp.status_code < 200 || resp.status_code >= 300 {
        return Err(anyhow!(format!("HTTP {}", resp.status_code)));
    }
    Ok(resp.as_bytes().to_vec())
}

fn normalize_tag_to_version(tag: &str) -> String {
    tag.trim_start_matches(|c: char| c == 'v' || c == 'V')
        .to_string()
}

// Default public manifest URL published by CI (override with AVRIX_MANIFEST_URL)
const DEFAULT_MANIFEST_URL: &str = "https://s3.storage.skymunt.com/avrix-loader/manifest.json";

fn get_manifest_url() -> String {
    if let Ok(s) = std::env::var("AVRIX_MANIFEST_URL") {
        let s = s.trim().to_string();
        if !s.is_empty() {
            return s;
        }
    }
    DEFAULT_MANIFEST_URL.to_string()
}

fn try_fetch_manifest() -> Result<Manifest> {
    let url = get_manifest_url();
    let bytes = http_get(&url)?;
    let mani: Manifest = serde_json::from_slice(&bytes)?;
    Ok(mani)
}

fn find_in_manifest<'a>(m: &'a Manifest, version: &str) -> Option<&'a ManifestVersion> {
    let needle = normalize_tag_to_version(version);
    m.versions
        .iter()
        .find(|v| normalize_tag_to_version(&v.version) == needle)
}

#[tauri::command]
pub fn list_available_versions(window: Window) -> Result<Vec<AvailableVersion>, String> {
    info("versions", "list_available_versions invoked");
    // Manifest-only listing
    let mani = match try_fetch_manifest() {
        Ok(m) => m,
        Err(e) => {
            let msg = format!("Manifest introuvable ou inaccessible: {}", e);
            let _ = window.emit("versions-log", msg.clone());
            return Err(msg);
        }
    };
    let out: Vec<AvailableVersion> = mani
        .versions
        .into_iter()
        .map(|v| AvailableVersion {
            tag: v.tag.unwrap_or_else(|| format!("v{}", v.version)),
            version: normalize_tag_to_version(&v.version),
            core_url: v.core_url,
            jre_url: v.jre_url,
            published_at: v.published_at,
        })
        .collect();
    let _ = window.emit(
        "versions-log",
        format!("{} versions depuis le manifest", out.len()),
    );
    Ok(out)
}

#[tauri::command]
pub fn install_version_from_release(version: String, window: Window) -> Result<String, String> {
    info(
        "versions",
        &format!("install_version_from_release invoked (version={})", version),
    );
    // Manifest-only resolution
    let needle_ver = normalize_tag_to_version(&version);
    let mani = try_fetch_manifest().map_err(|e| e.to_string())?;
    let mver = find_in_manifest(&mani, &needle_ver)
        .ok_or_else(|| "Version introuvable dans le manifest".to_string())?;
    let ver = normalize_tag_to_version(&mver.version);
    let core_url = mver.core_url.clone();
    let jre_url = mver.jre_url.clone(); // JRE peut être optionnel

    let root = versions_root().map_err(|e| e.to_string())?;
    fs::create_dir_all(&root).map_err(|e| e.to_string())?;
    let id = normalize_version_id(&ver);
    let dest = root.join(&id);
    if dest.exists() {
        return Err(format!("La version {} est déjà installée", id));
    }
    fs::create_dir_all(&dest).map_err(|e| e.to_string())?;

    // Download core jar
    let _ = window.emit(
        "versions-log",
        format!("Téléchargement Avrix-Core-{}.jar…", ver),
    );
    let core_bytes = http_get(&core_url).map_err(|e| e.to_string())?;
    fs::write(dest.join("Avrix-Core.jar"), &core_bytes).map_err(|e| e.to_string())?;

    // Download and unzip jre.zip if present
    if let Some(jre) = jre_url {
        let _ = window.emit("versions-log", "Téléchargement jre.zip…".to_string());
        let jre_bytes = http_get(&jre).map_err(|e| e.to_string())?;
        let tmp = dest.join(format!("_jre_tmp_{}", rand_suffix()));
        fs::create_dir_all(&tmp).ok();
        unzip_to(&jre_bytes, &tmp).map_err(|e| e.to_string())?;
        // If zip contains jre/ at root (as produced by CI), move that into dest/jre, else if contents is already jre, handle gracefully
        let tmp_jre = tmp.join("jre");
        let final_jre = dest.join("jre");
        if tmp_jre.exists() {
            let _ = fs::rename(&tmp_jre, &final_jre);
        } else {
            let _ = fs::rename(&tmp, &final_jre);
        }
        let _ = fs::remove_dir_all(&tmp);
    }

    let msg = format!("Version {} installée dans {}", id, dest.to_string_lossy());
    let _ = window.emit("versions-log", msg.clone());
    Ok(msg)
}

#[tauri::command]
pub fn repair_version_from_release(version: String, window: Window) -> Result<String, String> {
    info(
        "versions",
        &format!("repair_version_from_release invoked (version={})", version),
    );
    let needle_ver = normalize_tag_to_version(&version);
    let mani = try_fetch_manifest().map_err(|e| e.to_string())?;
    let mver = find_in_manifest(&mani, &needle_ver)
        .ok_or_else(|| "Version introuvable dans le manifest".to_string())?;
    let core_url = mver.core_url.clone();
    let target_jre = mver.jre_url.clone();
    let root = versions_root().map_err(|e| e.to_string())?;
    fs::create_dir_all(&root).map_err(|e| e.to_string())?;
    let id = normalize_version_id(&needle_ver);
    let dest = root.join(&id);
    fs::create_dir_all(&dest).map_err(|e| e.to_string())?;

    let _ = window.emit(
        "versions-log",
        format!("Réparation Avrix-Core-{}.jar…", needle_ver),
    );
    let core_bytes = http_get(&core_url).map_err(|e| e.to_string())?;
    fs::write(dest.join("Avrix-Core.jar"), &core_bytes).map_err(|e| e.to_string())?;
    if let Some(jre) = target_jre {
        let _ = window.emit("versions-log", "Téléchargement jre.zip…".to_string());
        let jre_bytes = http_get(&jre).map_err(|e| e.to_string())?;
        let final_jre = dest.join("jre");
        if final_jre.exists() {
            let _ = fs::remove_dir_all(&final_jre);
        }
        let tmp = dest.join(format!("_jre_tmp_{}", rand_suffix()));
        fs::create_dir_all(&tmp).ok();
        unzip_to(&jre_bytes, &tmp).map_err(|e| e.to_string())?;
        let tmp_jre = tmp.join("jre");
        if tmp_jre.exists() {
            let _ = fs::rename(&tmp_jre, &final_jre);
        } else {
            let _ = fs::rename(&tmp, &final_jre);
        }
        let _ = fs::remove_dir_all(&tmp);
    }
    let msg = format!("Version {} réparée dans {}", id, dest.to_string_lossy());
    let _ = window.emit("versions-log", msg.clone());
    Ok(msg)
}

#[tauri::command]
pub fn install_version_local(path: String, window: Window) -> Result<String, String> {
    info(
        "versions",
        &format!("install_version_local invoked (path={})", path),
    );
    let src = PathBuf::from(&path);
    if !src.exists() {
        return Err("Fichier introuvable".into());
    }
    let root = versions_root().map_err(|e| e.to_string())?;
    fs::create_dir_all(&root).map_err(|e| e.to_string())?;
    // Accept either a directory, a zip, or a single Avrix-Core.jar
    if src.is_dir() {
        // Detect version before copying
        let ver = detect_version_from_dir(&src).ok_or_else(|| {
            "Impossible de détecter la version (Avrix-Core.jar manquant)".to_string()
        })?;
        let id = normalize_version_id(ver);
        let dest = root.join(&id);
        if dest.exists() {
            return Err(format!("La version {} est déjà installée", id));
        }
        fs::create_dir_all(&dest).map_err(|e| e.to_string())?;
        fs_extra::dir::copy(
            &src,
            &dest,
            &fs_extra::dir::CopyOptions::new().copy_inside(true),
        )
        .map_err(|e| e.to_string())?;
        let msg = format!("Version {} installée dans {}", id, dest.to_string_lossy());
        let _ = window.emit("versions-log", msg.clone());
        return Ok(msg);
    }
    // If it's a zip: extract to a subfolder named by stem
    let ext = src
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if ext == "zip" {
        let data = fs::read(&src).map_err(|e| e.to_string())?;
        // Extract to temp, detect version, move to root/v<ver>
        let mut tmp = std::env::temp_dir();
        let rand = format!("avrix-version-{}", rand_suffix());
        tmp.push(rand);
        fs::create_dir_all(&tmp).map_err(|e| e.to_string())?;
        unzip_to(&data, &tmp).map_err(|e| e.to_string())?;
        let ver = detect_version_from_dir(&tmp)
            .ok_or_else(|| "Impossible de détecter la version dans l'archive".to_string())?;
        let id = normalize_version_id(ver);
        let dest = root.join(&id);
        if dest.exists() {
            fs::remove_dir_all(&tmp).ok();
            return Err(format!("La version {} est déjà installée", id));
        }
        match fs::rename(&tmp, &dest) {
            Ok(_) => {}
            Err(_) => {
                // Cross-device fallback: copy then remove temp
                fs::create_dir_all(&dest).map_err(|e| e.to_string())?;
                fs_extra::dir::copy(
                    &tmp,
                    &dest,
                    &fs_extra::dir::CopyOptions::new().copy_inside(true),
                )
                .map_err(|e| e.to_string())?;
                let _ = fs::remove_dir_all(&tmp);
            }
        }
        let msg = format!("Version {} extraite dans {}", id, dest.to_string_lossy());
        let _ = window.emit("versions-log", msg.clone());
        return Ok(msg);
    }
    // If it's a jar: copy as Avrix-Core.jar into a new folder named by stem
    if ext == "jar" {
        let ver = detect_version_from_jar(&src)
            .ok_or_else(|| "Impossible de détecter la version du .jar".to_string())?;
        let id = normalize_version_id(ver);
        let dest = root.join(&id);
        if dest.exists() {
            return Err(format!("La version {} est déjà installée", id));
        }
        fs::create_dir_all(&dest).map_err(|e| e.to_string())?;
        fs::copy(&src, dest.join("Avrix-Core.jar")).map_err(|e| e.to_string())?;
        let msg = format!("Version {} créée: {}", id, dest.to_string_lossy());
        let _ = window.emit("versions-log", msg.clone());
        return Ok(msg);
    }
    Err("Format non supporté. Utilisez un dossier, un zip, ou un Avrix-Core.jar".into())
}

#[tauri::command]
pub fn install_version_from_url(url: String, window: Window) -> Result<String, String> {
    info(
        "versions",
        &format!("install_version_from_url invoked (url={})", url),
    );
    const MAX_SIZE: u64 = 200 * 1024 * 1024; // 200 MiB
    if let Ok(head) = minreq::head(&url).send() {
        if let Some(len) = head
            .headers
            .get("content-length")
            .and_then(|v| v.parse::<u64>().ok())
        {
            if len > MAX_SIZE {
                return Err("Archive trop volumineuse".into());
            }
        }
    }
    let resp = minreq::get(&url).send().map_err(|e| e.to_string())?;
    if resp.status_code < 200 || resp.status_code >= 300 {
        return Err(format!("HTTP {}", resp.status_code));
    }
    let bytes = resp.as_bytes();
    if (bytes.len() as u64) > MAX_SIZE {
        return Err("Archive dépasse la taille maximale".into());
    }
    // Heuristic: if ends with .zip -> unzip, if .jar -> create folder, else try unzip first then fallback
    let root = versions_root().map_err(|e| e.to_string())?;
    fs::create_dir_all(&root).map_err(|e| e.to_string())?;
    let name = url.split('/').last().unwrap_or("").to_ascii_lowercase();
    if name.ends_with(".zip") {
        // Extract to temp then detect version and move
        let mut tmp = std::env::temp_dir();
        let rand = format!("avrix-version-{}", rand_suffix());
        tmp.push(rand);
        fs::create_dir_all(&tmp).map_err(|e| e.to_string())?;
        unzip_to(bytes, &tmp).map_err(|e| e.to_string())?;
        let ver = detect_version_from_dir(&tmp)
            .ok_or_else(|| "Impossible de détecter la version dans l'archive".to_string())?;
        let id = normalize_version_id(ver);
        let dest = root.join(&id);
        if dest.exists() {
            fs::remove_dir_all(&tmp).ok();
            return Err(format!("La version {} est déjà installée", id));
        }
        match fs::rename(&tmp, &dest) {
            Ok(_) => {}
            Err(_) => {
                fs::create_dir_all(&dest).map_err(|e| e.to_string())?;
                fs_extra::dir::copy(
                    &tmp,
                    &dest,
                    &fs_extra::dir::CopyOptions::new().copy_inside(true),
                )
                .map_err(|e| e.to_string())?;
                let _ = fs::remove_dir_all(&tmp);
            }
        }
        let msg = format!("Version {} installée: {}", id, dest.to_string_lossy());
        let _ = window.emit("versions-log", msg.clone());
        return Ok(msg);
    }
    // Treat otherwise as jar: write temp file, read metadata, then create version dir
    let mut tmp_file = std::env::temp_dir();
    tmp_file.push(format!("_avrix_core_{}.jar", rand_suffix()));
    fs::write(&tmp_file, bytes).map_err(|e| e.to_string())?;
    let ver = detect_version_from_jar(&tmp_file)
        .ok_or_else(|| "Impossible de détecter la version du .jar".to_string())?;
    let id = normalize_version_id(ver);
    let dest = root.join(&id);
    if dest.exists() {
        fs::remove_file(&tmp_file).ok();
        return Err(format!("La version {} est déjà installée", id));
    }
    fs::create_dir_all(&dest).map_err(|e| e.to_string())?;
    fs::rename(&tmp_file, dest.join("Avrix-Core.jar")).map_err(|e| e.to_string())?;
    let msg = format!("Version {} installée: {}", id, dest.to_string_lossy());
    let _ = window.emit("versions-log", msg.clone());
    Ok(msg)
}

#[tauri::command]
pub fn select_version(id: Option<String>, window: Window) -> Result<String, String> {
    info("versions", &format!("select_version invoked (id={:?})", id));
    let app = window.app_handle();
    save_selected_id(&app, id.clone()).map_err(|e| e.to_string())?;
    Ok(id.unwrap_or_else(|| "<none>".into()))
}

#[tauri::command]
pub fn get_selected_version(window: Window) -> Result<Option<String>, String> {
    let app = window.app_handle();
    Ok(load_selected_id(&app))
}

#[tauri::command]
pub fn delete_version(id: String, window: Window) -> Result<String, String> {
    info("versions", &format!("delete_version invoked (id={})", id));
    let root = versions_root().map_err(|e| e.to_string())?;
    let target = root.join(&id);
    if !target.exists() {
        return Err("Version introuvable".into());
    }
    // If selected, clear selection
    let app = window.app_handle();
    if let Some(sel) = load_selected_id(&app) {
        if sel == id {
            let _ = save_selected_id(&app, None);
        }
    }
    fs::remove_dir_all(&target).map_err(|e| e.to_string())?;
    Ok(format!("Supprimée: {}", id))
}

pub fn resolve_selected_version_dir(app: &AppHandle) -> Option<PathBuf> {
    let id = load_selected_id(app)?;
    let root = versions_root().ok()?;
    let dir = root.join(id);
    if dir.exists() {
        Some(dir)
    } else {
        None
    }
}

fn rand_suffix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{:x}", nanos)
}
