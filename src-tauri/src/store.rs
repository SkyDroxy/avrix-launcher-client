use crate::logger::{info, warn};
use anyhow::{anyhow, Context, Result};
use serde_json::json;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

pub fn setup_stores(app: &AppHandle) -> Result<()> {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .context("Cannot determine executable directory")?;
    let target_path = exe_dir.join("avrix-settings.json");
    let store = app
        .store(&target_path)
        .map_err(|e| anyhow!(e.to_string()))?;

    info(
        "store",
        &format!("Initialized at {}", target_path.display()),
    );
    info(
        "store",
        &format!("Using settings file at: {}", target_path.display()),
    );

    let has_mem = store.get("memoryMB").is_some();
    let has_preset = store.get("memPreset").is_some();

    if !has_mem || !has_preset {
        warn(
            "store",
            &format!(
                "Missing keys -> memoryMB: {}, memPreset: {} (starting migration/defaults)",
                has_mem, has_preset
            ),
        );

        let mut migrated_mem = false;
        let mut migrated_preset = false;
        let mut defaulted_mem = false;
        let mut defaulted_preset = false;

        if let Ok(legacy) = app.store("settings.json") {
            if !has_mem {
                if let Some(v) = legacy.get("memoryMB") {
                    store.set("memoryMB", v);
                    migrated_mem = true;
                }
            }
            if !has_preset {
                if let Some(v) = legacy.get("memPreset") {
                    store.set("memPreset", v);
                    migrated_preset = true;
                }
            }
        }
        if let Ok(legacy_avrix) = app.store("avrix-settings.json") {
            if !has_mem {
                if let Some(v) = legacy_avrix.get("memoryMB") {
                    store.set("memoryMB", v);
                    migrated_mem = true;
                }
            }
            if !has_preset {
                if let Some(v) = legacy_avrix.get("memPreset") {
                    store.set("memPreset", v);
                    migrated_preset = true;
                }
            }
        }

        if store.get("memoryMB").is_none() {
            store.set("memoryMB", json!(3072u64)); // ~3 GB default
            defaulted_mem = true;
        }
        if store.get("memPreset").is_none() {
            store.set("memPreset", json!("auto"));
            defaulted_preset = true;
        }

        store.save().map_err(|e| anyhow!(e.to_string()))?;
        info("store", &format!(
            "Migration summary -> migrated_mem: {}, migrated_preset: {}, defaulted_mem: {}, defaulted_preset: {}",
            migrated_mem, migrated_preset, defaulted_mem, defaulted_preset
        ));
    } else {
        info("store", "Keys already present. No migration needed.");
    }

    let final_mem = store
        .get("memoryMB")
        .map(|v| v.to_string())
        .unwrap_or_else(|| "<none>".into());
    let final_preset = store
        .get("memPreset")
        .map(|v| v.to_string())
        .unwrap_or_else(|| "<none>".into());

    info(
        "store",
        &format!(
            "Final values -> memoryMB: {}, memPreset: {}",
            final_mem, final_preset
        ),
    );

    Ok(())
}
