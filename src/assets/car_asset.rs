use bevy::asset::{AssetLoader, LoadRequest};

// Use support/car to load car asset with mesh and textures...
#[derive(AssetCollection, Resource)]
struct CarAsset {
    #[asset(key = "body")]
    body: Handle<Mesh>,
    // #[asset(key = "combined_image")]
    // combined_image: Handle<Image>,
    // #[asset(key = "tree_standard_material")]
    // tree_standard_material: Handle<StandardMaterial>,
    // #[asset(key = "player_standard_material")]
    // player_standard_material: Handle<StandardMaterial>,
}

#[derive(Default)]
pub struct CarAssetLoader;

impl AssetLoader for CarAssetLoader {
    // obsolete, move this to asset_io somehow?
    // fn from_bytes(&self, asset_path: &Path, _bytes: Vec<u8>) -> Result<Car> {
    //     info!("### Loading car {:?} via AssetLoader", asset_path);
    //     Car::load_from(asset_path)
    // }

    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), AnyError>> {
        todo!()
    }

    fn extensions(&self) -> &[&str] {
        &["ENC"]
    }
}
