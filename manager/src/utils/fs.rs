use std::borrow::Cow;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use zip::ZipArchive;
use eyre::Result;

pub type PluginZip = ZipArchive<Cursor<Vec<u8>>>;

pub fn extract_archive<M>(
    mut archive: PluginZip,
    dir: PathBuf,
    mut is_valid: M,
) -> Result<()> 
where
    M: FnMut(&Path) -> Result<Option<Cow<Path>>>
{
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

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

        std::fs::create_dir_all(&target_path.parent().unwrap())?;

        let mut target_file = std::fs::File::create(&target_path)?;
        std::io::copy(&mut file, &mut target_file)?;
    }

    Ok(())
}
