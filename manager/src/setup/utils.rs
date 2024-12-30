use crate::storage::NetSemaphore;
use crate::utils::download;
use regex::Regex;

pub const GITHUB_BEPINEX_URL: &'static str = "https://api.github.com/repos/BepInEx/BepInEx/releases/latest";

const BEPINEX_REGEX_STR: &'static str = r#"BepInEx_win_x64_([0-9]+(\.[0-9]+)+)\.zip"#;

/// Returns download link to the latest version of BepInEx mod loader (for windows)
pub async fn get_latest_bepinex(net_semaphore: &NetSemaphore) -> crate::Result<(String, String)> {
    let body = download::fetch_json(
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

#[cfg(test)]
mod tests {
    use crate::setup::utils::check_bepinex_file_validity;

    #[test]
    fn test_check_bepinex_file_validity() {
        assert!(check_bepinex_file_validity("BepInEx_win_x64_5.4.23.2.zip"));
    }
}
