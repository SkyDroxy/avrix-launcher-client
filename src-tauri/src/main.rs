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

fn main() {
    info("main", "Avrix Launcher starting up");
    tauri::Builder::default()
        .plugin(tauri_plugin_stronghold::Builder::new(|pass| todo!()).build())
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
