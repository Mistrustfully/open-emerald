use asset_loaders::AssetLoaders;
use bevy::{
	diagnostic::FrameTimeDiagnosticsPlugin,
	prelude::*,
	render::{render_resource::WgpuFeatures, settings::WgpuSettings},
	window::WindowResizeConstraints,
};

use kayak_ui::prelude::{widgets::*, *};

use iyes_loopless::prelude::*;
use main_menu::MenuPlugin;

mod asset_loaders;
mod battle;
mod camera;
mod lerp;
mod main_menu;
mod overworld;
mod pokemon;
mod widgets;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
	Menu,
	Overworld,
	Battle,
}

pub const WIDTH: u32 = 240;
pub const HEIGHT: u32 = 160;

fn main() {
	let width = WIDTH as f32;
	let height = HEIGHT as f32;

	let mut app = App::new();
	app.add_loopless_state(GameState::Menu)
		.insert_resource(Msaa { samples: 1 })
		.add_plugins(
			DefaultPlugins
				.set(WindowPlugin {
					window: WindowDescriptor {
						width: width * 5.0,
						height: height * 5.0,
						title: "test".to_string(),
						resize_constraints: WindowResizeConstraints {
							min_width: width,
							min_height: height,
							..default()
						},
						..default()
					},
					..default()
				})
				.set(ImagePlugin::default_nearest()),
		)
		.add_plugin(AssetLoaders)
		.add_plugin(KayakContextPlugin)
		.add_plugin(KayakWidgets)
		.add_plugin(widgets::WidgetPlugin)
		.add_plugin(camera::CameraPlugin)
		.add_plugin(MenuPlugin)
		.add_plugin(overworld::event_object::EventObjectPlugin)
		.add_plugin(battle::BattlePlugin)
		.add_system(lerp::update_lerp);

	// Use bevy_editor_pls if in debug mode.
	#[cfg(debug_assertions)]
	{
		let mut wgpu_settings = WgpuSettings::default();
		wgpu_settings.features |= WgpuFeatures::POLYGON_MODE_LINE;

		app.add_plugin(bevy_editor_pls::EditorPlugin)
			.add_plugin(FrameTimeDiagnosticsPlugin)
			.insert_resource(wgpu_settings);
	}

	app.run();
}
