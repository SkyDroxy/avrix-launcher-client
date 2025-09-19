use crate::logger::{error, info};
use crate::models::RawMetadata;
use anyhow::{anyhow, Result};
use std::path::PathBuf;

pub fn extract_metadata_from_jar(path: &PathBuf) -> Result<RawMetadata> {
    use std::io::Read;
    info(
        "metadata",
        &format!("Extracting metadata from {}", path.display()),
    );
    let file = std::fs::File::open(path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    let mut found_raw: Option<(String, RawMetadata)> = None;
    let mut last_error: Option<anyhow::Error> = None;
    for i in 0..archive.len() {
        let mut f = archive.by_index(i)?;
        let name = f.name().to_string();
        if name.eq_ignore_ascii_case("metadata.yml") || name.ends_with("/metadata.yml") {
            let mut contents = String::new();
            if let Err(e) = f.read_to_string(&mut contents) {
                last_error = Some(anyhow!(e));
                continue;
            }
            match serde_yaml::from_str::<RawMetadata>(&contents) {
                Ok(raw) => {
                    found_raw = Some((contents, raw));
                    break;
                }
                Err(e) => {
                    last_error = Some(anyhow!(e));
                }
            }
        }
    }
    if let Some((raw_text, meta)) = found_raw {
        let preview = raw_text.lines().take(6).collect::<Vec<_>>().join(" | ");
        info("metadata", &format!("metadata.yml loaded: {} ...", preview));
        return Ok(meta);
    }
    if let Some(err) = last_error {
        error("metadata", &format!("metadata.yml invalid: {}", err));
        return Err(anyhow!("metadata.yml invalid: {}", err));
    }
    error("metadata", "metadata.yml not found");
    Err(anyhow!("metadata.yml not found"))
}

pub fn is_valid_avrix_plugin(path: &PathBuf) -> bool {
    if !path.exists() {
        return false;
    }
    if path
        .extension()
        .and_then(|s| s.to_str())
        .map(|e| e.eq_ignore_ascii_case("jar"))
        .unwrap_or(false)
        == false
    {
        return false;
    }
    extract_metadata_from_jar(path).is_ok()
}
