#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod install;
mod launch;
mod logger;
mod metadata;
mod models;
mod store;
mod util;
mod versions;
mod workshop;

use crate::logger::info;
#[tauri::command]
fn scan_plugins(window: tauri::Window) -> Result<models::PluginsResult, String> {
    info("main", "scan_plugins invoked");
    util::scan_plugins(&window).map_err(|e| e.to_string())
}

#[tauri::command]
fn launch_game(window: tauri::Window, steam: bool, mem_mb: Option<u64>) -> Result<String, String> {
    info(
        "main",
        &format!("launch_game invoked (steam={}, mem_mb={:?})", steam, mem_mb),
    );
    launch::launch_game(window, steam, mem_mb).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_memory_info() -> Result<models::MemoryInfo, String> {
    info("main", "get_memory_info invoked");
    use sysinfo::{MemoryRefreshKind, RefreshKind, System};
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing().with_memory(MemoryRefreshKind::everything()),
    );
    sys.refresh_memory();
    let total_mb = (sys.total_memory() / (1024 * 1024)) as u64;
    let available_mb = (sys.available_memory() / (1024 * 1024)) as u64;
    Ok(models::MemoryInfo {
        total_mb,
        available_mb,
    })
}

#[tauri::command]
fn install_plugin_local(path: String, window: tauri::Window) -> Result<String, String> {
    info(
        "main",
        &format!("install_plugin_local invoked (path={})", path),
    );
    install::install_plugin_local(path, window).map_err(|e| e.to_string())
}

#[tauri::command]
fn install_plugin_from_url(
    url: String,
    window: tauri::Window,
) -> Result<models::InstallFromUrlResult, String> {
    info(
        "main",
        &format!("install_plugin_from_url invoked (url={})", url),
    );
    install::install_plugin_from_url(url, window).map_err(|e| e.to_string())
}

#[tauri::command]
fn validate_plugin_local(path: String) -> Result<models::ValidationMetadata, String> {
    info(
        "main",
        &format!("validate_plugin_local invoked (path={})", path),
    );
    install::validate_plugin_local(path).map_err(|e| e.to_string())
}

#[tauri::command]
fn validate_plugin_from_url(url: String) -> Result<models::ValidationMetadata, String> {
    info(
        "main",
        &format!("validate_plugin_from_url invoked (url={})", url),
    );
    install::validate_plugin_from_url(url).map_err(|e| e.to_string())
}

#[tauri::command]
fn scan_workshop(window: tauri::Window) -> Result<models::WorkshopScanResult, String> {
    info("main", "scan_workshop invoked");
    workshop::scan_workshop(window).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_plugin(name: String) -> Result<String, String> {
    info("main", &format!("delete_plugin invoked (name={})", name));
    util::delete_plugin(name).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_settings_path() -> Result<String, String> {
    info("main", "get_settings_path invoked");
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .ok_or_else(|| "Cannot determine executable directory".to_string())?;
    let path = exe_dir.join("avrix-settings.json");
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
fn open_external(window: tauri::Window, url: String) -> Result<(), String> {
    info("main", &format!("open_external invoked (url={})", url));
    let _ = window;
    tauri_plugin_opener::open_url(url, None::<&str>).map_err(|e| e.to_string())
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct GameRootInfo {
    effective: String,
    detected: String,
    override_path: Option<String>,
    is_override_valid: bool,
}

#[tauri::command]
fn get_game_root_info(app: tauri::AppHandle) -> Result<GameRootInfo, String> {
    use tauri_plugin_store::StoreExt;
    let effective = crate::util::get_effective_game_root(&app);
    let base = std::env::current_dir().unwrap_or_else(|_| std::env::temp_dir());
    let detected = match crate::util::find_game_root(&base) {
        Some(p) => p.to_string_lossy().to_string(),
        None => String::new(),
    };

    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .ok_or_else(|| "Cannot determine executable directory".to_string())?;
    let store_path = exe_dir.join("avrix-settings.json");
    let store = app.store(&store_path).map_err(|e| e.to_string())?;
    let override_path = store
        .get("gameRoot")
        .and_then(|v| v.as_str().map(|s| s.to_string()));
    let is_override_valid = override_path
        .as_ref()
        .map(|s| crate::util::is_valid_game_root_dir(std::path::Path::new(s)))
        .unwrap_or(false);

    Ok(GameRootInfo {
        effective: effective.to_string_lossy().to_string(),
        detected,
        override_path,
        is_override_valid,
    })
}

#[tauri::command]
fn set_game_root(app: tauri::AppHandle, path: String) -> Result<(), String> {
    use tauri_plugin_store::StoreExt;
    let p = std::path::PathBuf::from(&path);
    if !crate::util::is_valid_game_root_dir(&p) {
        return Err("Dossier Project Zomboid invalide: les fichiers .exe et .bat requis sont introuvables.".into());
    }
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .ok_or_else(|| "Cannot determine executable directory".to_string())?;
    let store_path = exe_dir.join("avrix-settings.json");
    let store = app.store(&store_path).map_err(|e| e.to_string())?;
    store.set("gameRoot", serde_json::Value::String(path));
    store.save().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn clear_game_root_override(app: tauri::AppHandle) -> Result<(), String> {
    use tauri_plugin_store::StoreExt;
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .ok_or_else(|| "Cannot determine executable directory".to_string())?;
    let store_path = exe_dir.join("avrix-settings.json");
    let store = app.store(&store_path).map_err(|e| e.to_string())?;
    if store.get("gameRoot").is_some() {
        store.delete("gameRoot");
        store.save().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn validate_game_root(path: String) -> Result<(), String> {
    let p = std::path::PathBuf::from(path);
    if crate::util::is_valid_game_root_dir(&p) {
        Ok(())
    } else {
        Err("Dossier Project Zomboid invalide.".into())
    }
}

fn main() {
    info("main", "Avrix Launcher starting up");
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(
            tauri_plugin_stronghold::Builder::new(|_| {
                let pass = std::env::var("STRONGHOLD_PASSWORD")
                    .unwrap_or_else(|_| "avrix-stronghold".to_string());
                pass.into_bytes()
            })
            .build(),
        )
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle();
            logger::setup_global_handlers(&handle);
            store::setup_stores(&handle).map_err(tauri::Error::from)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            scan_plugins,
            launch_game,
            get_memory_info,
            install_plugin_local,
            install_plugin_from_url,
            scan_workshop,
            validate_plugin_local,
            validate_plugin_from_url,
            delete_plugin,
            get_settings_path,
            open_external,
            get_game_root_info,
            set_game_root,
            clear_game_root_override,
            validate_game_root,
            versions::list_versions,
            versions::list_available_versions,
            versions::install_version_local,
            versions::install_version_from_url,
            versions::install_version_from_release,
            versions::repair_version_from_release,
            versions::select_version,
            versions::get_selected_version,
            versions::delete_version
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
