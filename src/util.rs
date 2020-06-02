use std::path::PathBuf;

/// Extract the crate to the cache directory
pub fn extract_crate(
    data: &[u8],
    crate_name: &str,
    crate_version: &str,
) -> anyhow::Result<PathBuf> {
    use {flate2::bufread::GzDecoder, std::io::BufReader, tar::Archive};

    let base = cache_dir()?;
    let target = base.join(format!("{}-{}", crate_name, crate_version));
    // avoid unpacking if it already exists
    if !target.is_dir() {
        Archive::new(GzDecoder::new(BufReader::new(data))).unpack(&base)?;
    }
    Ok(target)
}

/// Get the cache directory
pub fn cache_dir() -> anyhow::Result<PathBuf> {
    directories_next::ProjectDirs::from("com.github", "museun", "whatfeatures")
        .ok_or_else(|| anyhow::anyhow!("cannot open projects directory"))
        .map(|dir| dir.cache_dir().into())
}

/// This is the name chunking schemed used by crates.io
#[allow(dead_code)]
pub fn chunk_name(name: &str) -> String {
    assert!(!name.is_empty());
    match name.len() {
        1 => format!("1/{}", name),
        2 => format!("2/{}", name),
        3 => format!("3/{}/{}", &name[..1], name),
        _ => format!("{}/{}/{}", &name[..2], &name[2..4], name),
    }
}
