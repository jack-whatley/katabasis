use std::path::PathBuf;

#[cfg(windows)]
pub async fn create_symlink<P: Into<PathBuf>, T: Into<PathBuf>>(
    target: P,
    symlink: T
) -> crate::Result<()> {
    let target = target.into();
    let symlink = symlink.into();

    if target.is_dir() {
        tokio::task::spawn_blocking(move || {
            std::os::windows::fs::symlink_dir(target, symlink)
        }).await??;
    }
    else if target.is_file() {
        tokio::task::spawn_blocking(move || {
            std::os::windows::fs::symlink_file(target, symlink)
        }).await??;
    }
    else {
        return Err(crate::Error::FileSystemError(
            format!("Failed to create symlink for: '{:#?}'\nIs not a directory or a file...", target)
        ))
    }

    Ok(())
}
