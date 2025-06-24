use std::borrow::Cow;
use std::ffi::OsStr;
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use eyre::{Context, Result};
use tokio::fs::DirEntry;
use zip::ZipArchive;

pub type PluginZip = ZipArchive<Cursor<Vec<u8>>>;

pub async fn extract_archive<M>(
    mut archive: PluginZip,
    dir: PathBuf,
    mut is_valid: M,
) -> Result<()> 
where
    M: FnMut(&Path) -> Result<Option<Cow<Path>>>,
{
    for i in 0..archive.len() {
        let file = archive.by_index(i)?;

        if file.is_dir() {
            continue;
        }

        let file_name = file.name();

        let relative_path: Cow<'_, Path> = if cfg!(unix) && file_name.contains('\\') {
            PathBuf::from(file_name.replace('\\', "/")).into()
        }
        else {
            Path::new(file_name).into()
        };

        let Some(relative_target) = is_valid(&relative_path)? else { continue; };

        let target_path = dir.join(relative_target);

        tokio::fs::create_dir_all(&target_path.parent().unwrap()).await?;

        let mut target_file = tokio::fs::File::create(&target_path).await?;
        let mut file_bytes = Vec::new();

        for byte in file.bytes() {
            file_bytes.push(byte?);
        }

        tokio::io::copy(&mut file_bytes.as_slice(), &mut target_file).await?;
    }

    Ok(())
}

pub async fn iterate_directory(path: impl Into<PathBuf>) -> Result<Vec<DirEntry>> {
    let mut paths: Vec<DirEntry> = vec![];
    let mut items = tokio::fs::read_dir(path.into()).await?;

    while let Some(entry) = items.next_entry().await? {
        let path = entry.path();

        paths.push(entry);

        if path.is_dir() {
            paths.append(&mut Box::pin(iterate_directory(path)).await?)
        }
    }

    Ok(paths)
}

#[derive(Debug, Clone, Copy)]
pub struct CopyFileOpts {
    pub should_copy_file: bool,
    pub should_overwrite_file: bool,
}

pub async fn copy_dir_contents_to<F>(
    src_dir: impl AsRef<Path>,
    dest_dir: PathBuf,
    mut pre_install: F
) -> Result<()>
where
    F: FnMut(&Path, bool) -> Result<CopyFileOpts>,
{
    for entry in iterate_directory(src_dir.as_ref()).await? {
        let entry_path = entry.path();

        let rel_path = entry_path
            .strip_prefix(&src_dir)
            .context("failed to determine relative path from dir")?;

        let full_path = dest_dir.join(rel_path);

        if entry_path.is_dir() {
            // Skip dir creation if exists
            if full_path.exists() {
                continue;
            }

            tokio::fs::create_dir(&full_path).await?;
        }
        else {
            let pre_existing = full_path.exists();
            let copy_opts: CopyFileOpts = pre_install(rel_path, pre_existing)?;

            if pre_existing {
                if !copy_opts.should_overwrite_file {
                    tracing::warn!(
                        "file {} is being skipped during copy_dir_contents_to",
                        rel_path.display()
                    );

                    continue;
                }

                if copy_opts.should_overwrite_file && !copy_opts.should_copy_file {
                    tokio::fs::remove_file(&full_path)
                        .await
                        .with_context(
                            || format!("failed to remove existing file {}", rel_path.display())
                        )?;
                }
            }

            if copy_opts.should_copy_file {
                tokio::fs::copy(entry.path(), full_path).await.with_context(
                    || format!("copying file failed at {}", rel_path.display())
                )?;
            }
            else {
                tokio::fs::hard_link(entry.path(), full_path).await.with_context(
                    || format!("hard linking file failed at {}", rel_path.display())
                )?;
            }
        }
    }

    Ok(())
}

/// Function for switching a file between enabled and disabled, this involves changing
/// the extension so mod loaders like `BepInEx` will ignore the file.
///
/// For example:
/// - `if state == true file_name = file_name`
/// - `if state != true file_name = file_name.DISABLED`
pub async fn switch_file(path: impl AsRef<Path>, state: bool) -> Result<()> {
    let path = path.as_ref();
    let mut switched_path = path.to_path_buf();

    if state {
        add_extension(&mut switched_path, "DISABLED");
    }
    else {
        while let Some("DISABLED") = switched_path.extension().and_then(OsStr::to_str) {
            switched_path.set_extension("");
        }
    }

    tokio::fs::rename(path, &switched_path).await?;

    Ok(())
}

fn add_extension(path: &mut PathBuf, extension: impl AsRef<OsStr>) {
    match path.extension() {
        Some(ext) => {
            let mut ext = ext.to_os_string();

            ext.push(".");
            ext.push(extension);

            path.set_extension(ext);
        }
        None => {
            path.set_extension(extension);
        }
    }
}
