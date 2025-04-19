use async_trait::async_trait;
use manager_core::data::Plugin;
use manager_core::error::KatabasisResult;
use crate::PluginHandler;

pub struct ThunderstorePluginHandler;

#[async_trait]
impl PluginHandler for ThunderstorePluginHandler {
    fn get_api_url(&self) -> String {
        todo!()
    }

    async fn download_latest(&self, plugin: &Plugin) -> KatabasisResult<()> {
        todo!()
    }

    async fn check_for_updates(&self, plugin: &Plugin) -> KatabasisResult<bool> {
        todo!()
    }
}
