use std::borrow::Cow;
use std::path::{Component, Path, PathBuf};
use async_trait::async_trait;
use eyre::{OptionExt, Result};
use crate::collection::{Collection, Plugin};
use crate::collection::install::handler::PluginHandler;
use crate::utils::{fs, paths};
use crate::utils::fs::{CopyFileOpts, PluginZip};

#[derive(Debug)]
pub struct MappedInstaller<'a> {
    dir_maps: &'a [DirectoryMap<'a>],
    default_map: usize,
}

#[derive(Debug)]
pub struct DirectoryMap<'a> {
    pub dir_name: &'a str,
    pub dir_path: &'a str,
    pub mode: MapMode,
    pub files_mutable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapMode {
    SeparateDir,
    None,
}

impl<'a> DirectoryMap<'a> {
    pub const fn new(dir_name: &'a str, dir_path: &'a str, mode: MapMode) -> Self {
        Self {
            dir_name,
            dir_path,
            mode,
            files_mutable: false,
        }
    }

    pub const fn flattened(dir_name: &'a str, dir_path: &'a str) -> Self {
        Self::new(dir_name, dir_path, MapMode::SeparateDir)
    }

    pub const fn none(dir_name: &'a str, dir_path: &'a str) -> Self {
        Self::new(dir_name, dir_path, MapMode::None)
    }

    pub const fn files_mutable(mut self) -> Self {
        self.files_mutable = true;

        self
    }
}

impl<'a> MappedInstaller<'a> {
    pub fn new(maps: &'a [DirectoryMap<'a>], default_map: usize) -> Self {
        Self {
            dir_maps: maps,
            default_map,
        }
    }

    fn maps(&self) -> impl Iterator<Item = &DirectoryMap> {
        self.dir_maps.iter()
    }

    fn match_map(&self, name: &str) -> Option<&DirectoryMap> {
        self.maps().find(|map| map.dir_name.to_lowercase() == name.to_lowercase())
    }

    fn map_relative_to_dir<'b>(
        &self,
        rel_path: &'b Path,
        plugin: &str
    ) -> Result<Option<Cow<'b, Path>>> {
        let mut alt_path = PathBuf::new();
        let mut components = rel_path.components();

        let matching_map = loop {
            match components.next() {
                Some(Component::Normal(name)) => {
                    alt_path.push(name);

                    if let Some(name) = name.to_str() {
                        if let Some(map) = self.match_map(name) {
                            break map;
                        }
                    }
                }
                Some(Component::ParentDir) => { alt_path.pop(); }
                Some(Component::RootDir | Component::Prefix(_) | Component::CurDir) => continue,
                None => break &self.dir_maps[self.default_map],
            }
        };

        let mut target_path = PathBuf::from(matching_map.dir_path);

        let should_separate = matching_map.mode == MapMode::SeparateDir;
        let is_top_level = components.clone().next().is_none();

        if should_separate {
            target_path.push(plugin);
        }

        if is_top_level {
            target_path.push(&alt_path);
        }
        else {
            target_path.push(components);
        }

        Ok(Some(Cow::Owned(target_path)))
    }

    async fn for_mod_file<F>(
        &self,
        plugin: &Plugin,
        collection: &Collection,
        mut on_file: F
    ) -> Result<()>
    where
        F: AsyncFnMut(&Path) -> Result<()>
    {
        let package_name = plugin.kind.full_name();

        for map in self.maps() {
            match map.mode {
                MapMode::SeparateDir => {
                    let path = paths::collection_dir(&collection.name)
                        .join(&map.dir_path)
                        .join(package_name);

                    on_file(&path).await?;
                }
                MapMode::None => {}
            }
        }

        Ok(())
    }
}

#[async_trait]
impl PluginHandler for MappedInstaller<'_> {
    async fn extract(&self, zip: PluginZip, dir: PathBuf, plugin_name: &str) -> Result<()> {
        let plugin_name = plugin_name.to_string();

        fs::extract_archive(zip, dir, |rel_path| {
            self.map_relative_to_dir(rel_path, &plugin_name)
        }).await?;

        Ok(())
    }

    async fn install(&self, src: &Path, _plugin_name: &str, collection: &Collection) -> Result<()> {
        let dest = paths::collection_dir(&collection.name);

        fs::copy_dir_contents_to(src, dest, |rel_path, _| -> Result<CopyFileOpts> {
            let mapped_dir = self
                .maps()
                .find(|map| rel_path.starts_with(map.dir_path))
                .ok_or_eyre("failed to find mapped dir, this should be impossible")?;

            let should_overwrite = match mapped_dir.mode {
                MapMode::SeparateDir => false,
                MapMode::None => true,
            };

            Ok(CopyFileOpts { should_copy_file: mapped_dir.files_mutable, should_overwrite_file: should_overwrite })
        }).await?;

        Ok(())
    }

    async fn uninstall(&self, plugin: &Plugin, collection: &Collection) -> Result<()> {
        self.for_mod_file(plugin, collection, async |path| -> Result<()> {
            if path.is_file() {
                tokio::fs::remove_file(path).await?;
            }
            else {
                tokio::fs::remove_dir_all(path).await?;
            }

            Ok(())
        }).await?;

        Ok(())
    }

    async fn switch(&self, enabled: bool, plugin: &Plugin, collection: &Collection) -> Result<()> {
        todo!()
    }
}
