use std::io::Cursor;

use crate::{
    state::AppState,
    thunderstore::{
        models::{Package, PackageVersion},
        version::{PackageIdent, VersionIdent},
    },
    utils::{fs::PluginZip, net},
};
use eyre::Result;
use futures::StreamExt;
use zip::ZipArchive;

pub mod models;
pub mod version;

pub async fn query_latest_package(ident: &PackageIdent) -> Result<Package> {
    let state = AppState::get().await?;
    let url = latest_package_url(&ident);

    Ok(net::fetch_json(&url, state.http()).await?)
}

pub async fn download_latest_package(ident: &PackageIdent) -> Result<(PluginZip, VersionIdent)> {
    let package = query_latest_package(ident).await?;
    let zip = download_specific_package(&package.latest.ident).await?;

    Ok((zip, package.latest.ident))
}

pub async fn download_specific_package(ident: &VersionIdent) -> Result<PluginZip> {
    let state = AppState::get().await?;
    let json_url = specific_package_url(ident);

    let package_version: PackageVersion = net::fetch_json(&json_url, state.http()).await?;
    let mut bytes_stream = net::fetch_stream(&package_version.download_url, state.http()).await?;

    let mut bytes: Vec<u8> = Vec::new();

    while let Some(chunk) = bytes_stream.next().await {
        // TODO: Report progress here...
        bytes.extend(chunk?);
    }

    Ok(ZipArchive::new(Cursor::new(bytes))?)
}

fn latest_package_url(ident: &PackageIdent) -> String {
    format!(
        "https://thunderstore.io/api/experimental/package/{}/{}",
        ident.namespace(),
        ident.name()
    )
}

fn specific_package_url(ident: &VersionIdent) -> String {
    format!(
        "https://thunderstore.io/api/experimental/package/{}/{}/{}",
        ident.namespace(),
        ident.name(),
        ident.version()
    )
}
