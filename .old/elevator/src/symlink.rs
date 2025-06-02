use std::path::PathBuf;

#[cfg(windows)]
pub async fn create(target_file: PathBuf, symlink: PathBuf) -> anyhow::Result<()> {
    if !target_file.exists() {
        return Err(anyhow::anyhow!("Failed to find target file '{:?}'", target_file));
    }

    if target_file.is_dir() {
        tokio::task::spawn_blocking(move || {
            std::os::windows::fs::symlink_dir(target_file, symlink)
        }).await??;
    }
    else if target_file.is_file() {
        tokio::task::spawn_blocking(move || {
            std::os::windows::fs::symlink_file(target_file, symlink)
        }).await??;
    }
    else {
        return Err(anyhow::anyhow!("Provided Symlink Target is not a valid directory or file. '{:?}'", target_file));
    }

    Ok(())
}

#[cfg(unix)]
pub async fn create(target_file: PathBuf, symlink: PathBuf) -> anyhow::Result<()> {
    if !target_file.exists() {
        return Err(anyhow::anyhow!("Failed to find target file '{:?}'", target_file));
    }

    tokio::task::spawn_blocking(move || {
        std::os::unix::fs::symlink(target_file, symlink)
    }).await??;

    Ok(())
}
