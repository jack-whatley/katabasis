use std::path::Path;
use crate::storage::NetSemaphore;
use crate::utils::download;
use regex::Regex;

pub const GITHUB_BEPINEX_URL: &'static str = "https://api.github.com/repos/BepInEx/BepInEx/releases/latest";

const BEPINEX_REGEX_STR: &'static str = r#"BepInEx_win_x64_([0-9]+(\.[0-9]+)+)\.zip"#;

/// Returns download link to the latest version of BepInEx mod loader (for windows)
pub async fn get_latest_bepinex(net_semaphore: &NetSemaphore) -> crate::Result<(String, String)> {
    let body = download::fetch_json::<serde_json::Value>(
        GITHUB_BEPINEX_URL,
        net_semaphore
    ).await?;

    let download_assets = body["assets"].as_array().ok_or(
        crate::error::Error::ParseError(
            "Failed to parse download assets".to_string()
        )
    )?;

    for asset in download_assets {
        let name = asset["name"].as_str().ok_or(
            crate::error::Error::ParseError(
                "Failed to parse download assets".to_string()
            )
        )?;

        if check_bepinex_file_validity(name) {
            let file_name = asset["name"].as_str().ok_or(
                crate::error::Error::ParseError(
                    "Failed to parse BepInEx download name".to_string()
                )
            )?;

            let download_url = asset["browser_download_url"].as_str().ok_or(
                crate::error::Error::ParseError(
                    "Failed to parse download assets".to_string()
                )
            )?;

            // Returns file name, download url
            return Ok((file_name.to_string(), download_url.to_string()));
        }
    }

    Err(
        crate::Error::ParseError(
            "Failed to fetch latest BepInEx version from github".to_string()
        )
    )
}

fn check_bepinex_file_validity(haystack: &str) -> bool {
    Regex::new(BEPINEX_REGEX_STR).unwrap().is_match(haystack)
}

// Windows workaround to requiring permission to use std::os::windows::fs::symlink_file/symlink_dir
// See here: https://stackoverflow.com/questions/64991523/why-are-administrator-privileges-required-to-create-a-symlink-on-windows
// This doesn't seem to work, not sure what the solution is, perhaps worth looking into how to do it with tauri
// Vortex also seems to request admin somehow, not sure how though (maybe an electron API)

#[cfg(windows)]
pub fn symlink_file<P: AsRef<Path>, U: AsRef<Path>>(src: P, dst: U) -> crate::Result<()> {
    std::os::windows::fs::symlink_file(&src, &dst).map_err(|err| {
        crate::Error::FileSystemError(
            format!("Failed to create Windows symlink from '{}' to '{}' with error: '{}'", src.as_ref().display(), dst.as_ref().display(), err)
        )
    })
}

#[cfg(windows)]
pub fn symlink_dir<P: AsRef<Path>, U: AsRef<Path>>(src: P, dst: U) -> crate::Result<()> {
    std::os::windows::fs::symlink_dir(&src, &dst).map_err(|err| {
        crate::Error::FileSystemError(
            format!("Failed to create Windows symlink from '{}' to '{}' with error: '{}'", src.as_ref().display(), dst.as_ref().display(), err)
        )
    })
}

#[cfg(unix)]
pub fn symlink_file<P: AsRef<Path>, U: AsRef<Path>>(src: P, dst: U) -> crate::Result<()> {
    std::os::unix::fs::symlink(src.as_ref(), dst.as_ref()).map_err(|err| {
        crate::Error::FileSystemError(
            format!("Failed to create Unix symlink from '{}' to '{}' with error: '{}'", src.as_ref().display(), dst.as_ref().display(), err)
        )
    })
}

#[cfg(unix)]
pub fn symlink_dir<P: AsRef<Path>, U: AsRef<Path>>(src: P, dst: U) -> crate::Result<()> {
    symlink_file(src, dst)
}

#[cfg(test)]
mod tests {
    use crate::setup::utils::check_bepinex_file_validity;

    #[test]
    fn test_check_bepinex_file_validity() {
        assert!(check_bepinex_file_validity("BepInEx_win_x64_5.4.23.2.zip"));
    }
}
