use crate::logger::{error, info};
use crate::models::RawMetadata;
use anyhow::{anyhow, Result};
use std::path::PathBuf;

pub fn extract_metadata_with_base_from_jar(
    path: &PathBuf,
) -> Result<(RawMetadata, Option<String>)> {
    use std::io::Read;
    info(
        "metadata",
        &format!("Extracting metadata from {}", path.display()),
    );
    let file = std::fs::File::open(path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    let mut found_raw: Option<(String, RawMetadata, Option<String>)> = None;
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
                    // base dir is the path prefix up to the last '/'
                    let base = name.rfind('/').map(|idx| name[..idx].to_string());
                    found_raw = Some((contents, raw, base));
                    break;
                }
                Err(e) => {
                    last_error = Some(anyhow!(e));
                }
            }
        }
    }
    if let Some((raw_text, meta, base)) = found_raw {
        let preview = raw_text.lines().take(6).collect::<Vec<_>>().join(" | ");
        info("metadata", &format!("metadata.yml loaded: {} ...", preview));
        return Ok((meta, base));
    }
    if let Some(err) = last_error {
        error("metadata", &format!("metadata.yml invalid: {}", err));
        return Err(anyhow!("metadata.yml invalid: {}", err));
    }
    error("metadata", "metadata.yml not found");
    Err(anyhow!("metadata.yml not found"))
}

// Back-compat convenience wrapper
pub fn extract_metadata_from_jar(path: &PathBuf) -> Result<RawMetadata> {
    extract_metadata_with_base_from_jar(path).map(|(m, _)| m)
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
