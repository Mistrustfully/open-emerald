use bevy::prelude::*;

pub mod font_config;

pub struct AssetLoaders;
impl Plugin for AssetLoaders {
	fn build(&self, app: &mut App) {
		app.add_asset::<font_config::FontConfig>()
			.init_asset_loader::<font_config::FontConfigLoader>();
	}
}
