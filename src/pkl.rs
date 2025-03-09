use bevy_app::{App, Plugin};
use bevy_asset::io::Reader;
use bevy_asset::{Asset, AssetApp, AssetLoader, LoadContext};
use std::marker::PhantomData;
use thiserror::Error;

/// Plugin to load your asset type `A` from pkl files.
pub struct PklAssetPlugin<A> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<A>,
}

impl<A> Plugin for PklAssetPlugin<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    fn build(&self, app: &mut App) {
        app.init_asset::<A>()
            .register_asset_loader(PklAssetLoader::<A> {
                extensions: self.extensions.clone(),
                _marker: PhantomData,
            });
    }
}

impl<A> PklAssetPlugin<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    /// Create a new plugin that will load assets from files with the given extensions.
    pub fn new(extensions: &[&'static str]) -> Self {
        Self {
            extensions: extensions.to_owned(),
            _marker: PhantomData,
        }
    }
}

/// Loads your asset type `A` from pkl files
pub struct PklAssetLoader<A> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<A>,
}

/// Possible errors that can be produced by [`PklAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum PklLoaderError {
    /// An [IO Error](std::io::Error)
    #[error("Could not read the file: {0}")]
    Io(#[from] std::io::Error),
    /// A [PKL Error](rpkl::error::Error)
    #[error("Could not parse the PKL: {0}")]
    PklError(#[from] rpkl::error::Error),
}

impl<A> AssetLoader for PklAssetLoader<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    type Asset = A;
    type Settings = ();
    type Error = PklLoaderError;

    async fn load(
        &self,
        _reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        // Get the path from the load context
        let path = load_context.path().to_path_buf();

        // Use rpkl to load and deserialize the PKL file
        let asset = rpkl::from_config::<A>(path)?;

        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }
}
