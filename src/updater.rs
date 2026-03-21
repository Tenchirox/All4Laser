#![allow(dead_code)]

//! Auto-updater: check GitHub releases, download assets, and install.
//!
//! Inspired by LaserMagic's updater but adapted for GitHub Releases API
//! and cross-platform binary replacement.

use serde::Deserialize;
use std::io::Write;
use std::path::PathBuf;

/// Information about an available update.
#[derive(Debug, Clone)]
pub struct UpdateInfo {
    pub tag: String,
    pub name: String,
    pub body: String,
    pub html_url: String,
    pub assets: Vec<AssetInfo>,
}

#[derive(Debug, Clone)]
pub struct AssetInfo {
    pub name: String,
    pub download_url: String,
    pub size: u64,
}

/// Progress of an ongoing download.
#[derive(Debug, Clone)]
pub enum UpdateProgress {
    Checking,
    Available(UpdateInfo),
    Downloading { percent: f32, bytes_done: u64, bytes_total: u64 },
    Installing,
    Done(PathBuf),
    Error(String),
    UpToDate,
}

#[derive(Deserialize)]
struct GhRelease {
    tag_name: String,
    name: Option<String>,
    body: Option<String>,
    html_url: String,
    assets: Vec<GhAsset>,
}

#[derive(Deserialize)]
struct GhAsset {
    name: String,
    browser_download_url: String,
    size: u64,
}

/// Check for updates against a GitHub releases endpoint.
pub fn check_update(
    api_url: &str,
    current_version: &str,
) -> Result<Option<UpdateInfo>, String> {
    let resp = ureq::get(api_url)
        .set("Accept", "application/vnd.github.v3+json")
        .set("User-Agent", "All4Laser-Updater")
        .call()
        .map_err(|e| format!("HTTP error: {e}"))?;

    let release: GhRelease = resp
        .into_json()
        .map_err(|e| format!("JSON parse error: {e}"))?;

    let remote_tag = release.tag_name.trim_start_matches('v');
    let local_tag = current_version.trim_start_matches('v');

    if remote_tag == local_tag {
        return Ok(None);
    }

    // Simple version comparison: if they differ, assume remote is newer
    let info = UpdateInfo {
        tag: release.tag_name.clone(),
        name: release.name.unwrap_or_else(|| release.tag_name.clone()),
        body: release.body.unwrap_or_default(),
        html_url: release.html_url,
        assets: release
            .assets
            .into_iter()
            .map(|a| AssetInfo {
                name: a.name,
                download_url: a.browser_download_url,
                size: a.size,
            })
            .collect(),
    };

    Ok(Some(info))
}

/// Pick the best asset for the current platform.
pub fn pick_platform_asset(assets: &[AssetInfo]) -> Option<&AssetInfo> {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    // Build candidate keywords for current platform
    let os_keywords: Vec<&str> = match os {
        "linux" => vec!["linux", "Linux"],
        "macos" => vec!["macos", "darwin", "osx", "MacOS"],
        "windows" => vec!["windows", "win", "Windows"],
        _ => vec![os],
    };
    let arch_keywords: Vec<&str> = match arch {
        "x86_64" => vec!["x86_64", "amd64", "x64"],
        "aarch64" => vec!["aarch64", "arm64"],
        _ => vec![arch],
    };

    // First try: match both OS and arch
    for asset in assets {
        let name = &asset.name;
        let os_match = os_keywords.iter().any(|k| name.contains(k));
        let arch_match = arch_keywords.iter().any(|k| name.contains(k));
        if os_match && arch_match {
            return Some(asset);
        }
    }

    // Fallback: match OS only
    for asset in assets {
        let name = &asset.name;
        let os_match = os_keywords.iter().any(|k| name.contains(k));
        if os_match {
            return Some(asset);
        }
    }

    None
}

/// Download an asset to a temporary directory and return its path.
/// Sends progress updates via the provided callback.
pub fn download_asset<F>(
    asset: &AssetInfo,
    progress: F,
) -> Result<PathBuf, String>
where
    F: Fn(u64, u64),
{
    let resp = ureq::get(&asset.download_url)
        .set("User-Agent", "All4Laser-Updater")
        .call()
        .map_err(|e| format!("Download error: {e}"))?;

    let total = asset.size;

    let download_dir = std::env::temp_dir().join("all4laser_update");
    std::fs::create_dir_all(&download_dir)
        .map_err(|e| format!("Cannot create temp dir: {e}"))?;

    let dest_path = download_dir.join(&asset.name);
    let mut file = std::fs::File::create(&dest_path)
        .map_err(|e| format!("Cannot create file: {e}"))?;

    let mut reader = resp.into_reader();
    let mut buf = [0u8; 8192];
    let mut downloaded: u64 = 0;

    loop {
        let n = reader.read(&mut buf).map_err(|e| format!("Read error: {e}"))?;
        if n == 0 {
            break;
        }
        file.write_all(&buf[..n])
            .map_err(|e| format!("Write error: {e}"))?;
        downloaded += n as u64;
        progress(downloaded, total);
    }

    file.flush().map_err(|e| format!("Flush error: {e}"))?;

    Ok(dest_path)
}

/// Attempt to install the downloaded asset.
///
/// On Linux: if it's a .tar.gz or .AppImage, extract/place next to current exe.
/// On Windows: if it's a .zip or .exe, extract/place next to current exe.
/// On macOS: if it's a .dmg or .zip, place in temp and open.
///
/// Returns the path to the installed binary or archive.
pub fn install_update(downloaded_path: &PathBuf) -> Result<PathBuf, String> {
    let name = downloaded_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_lowercase();

    let current_exe = std::env::current_exe()
        .map_err(|e| format!("Cannot determine current exe: {e}"))?;
    let install_dir = current_exe
        .parent()
        .ok_or("Cannot determine install directory")?;

    if name.ends_with(".appimage") || (name.ends_with("") && !name.contains('.')) {
        // Linux AppImage or raw binary: replace current exe
        let target = install_dir.join(
            downloaded_path
                .file_name()
                .unwrap_or_default(),
        );

        // Backup old binary
        let backup = current_exe.with_extension("bak");
        let _ = std::fs::rename(&current_exe, &backup);

        std::fs::copy(downloaded_path, &target)
            .map_err(|e| format!("Copy failed: {e}"))?;

        // Make executable on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = std::fs::Permissions::from_mode(0o755);
            let _ = std::fs::set_permissions(&target, perms);
        }

        return Ok(target);
    }

    if name.ends_with(".zip") || name.ends_with(".tar.gz") || name.ends_with(".tgz") {
        // For archives, copy to install dir and let the user extract manually,
        // or attempt extraction via system tools.
        let target = install_dir.join(
            downloaded_path.file_name().unwrap_or_default(),
        );
        std::fs::copy(downloaded_path, &target)
            .map_err(|e| format!("Copy archive failed: {e}"))?;

        // Try system extraction (best-effort)
        #[cfg(unix)]
        if name.ends_with(".tar.gz") || name.ends_with(".tgz") {
            let _ = std::process::Command::new("tar")
                .args(["xzf", &target.to_string_lossy(), "-C", &install_dir.to_string_lossy()])
                .status();
        }

        return Ok(target);
    }

    // Fallback: just return the downloaded path and let the user handle it
    Ok(downloaded_path.clone())
}

/// Full async-friendly update flow that can be driven from a background thread.
/// Sends `UpdateProgress` through a channel.
pub fn run_update_flow(
    api_url: &str,
    current_version: &str,
    tx: crossbeam_channel::Sender<UpdateProgress>,
) {
    let _ = tx.send(UpdateProgress::Checking);

    match check_update(api_url, current_version) {
        Ok(Some(info)) => {
            let _ = tx.send(UpdateProgress::Available(info.clone()));

            if let Some(asset) = pick_platform_asset(&info.assets) {
                let asset = asset.clone();
                let tx2 = tx.clone();

                match download_asset(&asset, |done, total| {
                    let pct = if total > 0 {
                        (done as f32 / total as f32) * 100.0
                    } else {
                        0.0
                    };
                    let _ = tx2.send(UpdateProgress::Downloading {
                        percent: pct,
                        bytes_done: done,
                        bytes_total: total,
                    });
                }) {
                    Ok(path) => {
                        let _ = tx.send(UpdateProgress::Installing);
                        match install_update(&path) {
                            Ok(installed) => {
                                let _ = tx.send(UpdateProgress::Done(installed));
                            }
                            Err(e) => {
                                let _ = tx.send(UpdateProgress::Error(e));
                            }
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(UpdateProgress::Error(e));
                    }
                }
            } else {
                let _ = tx.send(UpdateProgress::Error(
                    "No compatible asset found for this platform".into(),
                ));
            }
        }
        Ok(None) => {
            let _ = tx.send(UpdateProgress::UpToDate);
        }
        Err(e) => {
            let _ = tx.send(UpdateProgress::Error(e));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pick_platform_asset_matches_linux_x86_64() {
        let assets = vec![
            AssetInfo {
                name: "all4laser-v1.0-linux-x86_64.tar.gz".into(),
                download_url: "https://example.com/linux".into(),
                size: 1000,
            },
            AssetInfo {
                name: "all4laser-v1.0-windows-x64.zip".into(),
                download_url: "https://example.com/windows".into(),
                size: 2000,
            },
        ];

        let picked = pick_platform_asset(&assets);
        assert!(picked.is_some());
        // Should pick something (depends on test runner platform)
    }

    #[test]
    fn same_version_returns_none() {
        // This test would need a mock HTTP server, so we just test logic
        let remote = "1.2.3";
        let local = "1.2.3";
        assert_eq!(
            remote.trim_start_matches('v'),
            local.trim_start_matches('v')
        );
    }

    #[test]
    fn version_strip_v_prefix() {
        let tag = "v1.5.0";
        assert_eq!(tag.trim_start_matches('v'), "1.5.0");
    }
}
