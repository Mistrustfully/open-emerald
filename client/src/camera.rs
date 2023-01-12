// SPDX-FileCopyrightText: 2023 Christian Fletcher <mistrustfully@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{widgets::UICamera, HEIGHT, WIDTH};
use bevy::{
	core_pipeline::clear_color::ClearColorConfig,
	prelude::*,
	render::{
		camera::{RenderTarget, ScalingMode},
		render_resource::{
			Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
		},
		view::RenderLayers,
	},
};
use bevy_editor_pls::default_windows::hierarchy::picking::EditorRayCastSource;

pub const NATIVE_LAYER: RenderLayers = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);
pub const BATTLE_LAYER: RenderLayers = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 2) as u8);
pub const TRANSPARENT: Color = Color::Rgba {
	red: 0.0,
	green: 0.0,
	blue: 0.0,
	alpha: 0.0,
};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(setup_cameras);
	}
}

#[derive(Resource, Default)]
pub struct RenderTargets {
	pub overworld: Handle<Image>,
	pub battle: Handle<Image>,
	pub ui: Handle<Image>,
}

#[derive(Component)]
pub struct Canvas;

fn create_image(width: u32, height: u32) -> Image {
	let size = Extent3d {
		width,
		height,
		..default()
	};

	let mut image = Image {
		texture_descriptor: TextureDescriptor {
			label: None,
			size,
			dimension: TextureDimension::D2,
			format: TextureFormat::Bgra8UnormSrgb,
			mip_level_count: 1,
			sample_count: 1,
			usage: TextureUsages::TEXTURE_BINDING
				| TextureUsages::COPY_DST
				| TextureUsages::RENDER_ATTACHMENT,
		},
		..default()
	};

	image.resize(size);

	return image;
}

fn setup_cameras(
	mut commands: Commands,
	mut images: ResMut<Assets<Image>>,
	mut ui_camera_query: Query<&mut Camera, With<UICamera>>,
) {
	// We render at native GBA resolution, and scale that to the window

	// The overworld layer is defaulted to 1.
	let overworld = create_image(WIDTH, HEIGHT);
	let overworld_handle = images.add(overworld);

	let battle = create_image(WIDTH, HEIGHT);
	let battle_handle = images.add(battle);

	let ui = create_image(WIDTH, HEIGHT);
	let ui_handle = images.add(ui);

	let mut overworld_camera = Camera2dBundle::default();
	overworld_camera.camera.target = RenderTarget::Image(overworld_handle.clone_weak());
	overworld_camera.camera_2d.clear_color = ClearColorConfig::Custom(TRANSPARENT);

	let mut battle_camera = Camera2dBundle::default();
	battle_camera.camera.target = RenderTarget::Image(battle_handle.clone_weak());
	battle_camera.camera_2d.clear_color = ClearColorConfig::Custom(TRANSPARENT);

	let mut ui_camera = ui_camera_query
		.get_single_mut()
		.expect("UI Camera wasn't created!");

	ui_camera.target = RenderTarget::Image(ui_handle.clone_weak());

	let mut native_camera = Camera2dBundle::default();
	native_camera.camera_2d.clear_color = ClearColorConfig::Custom(Color::hex("8C96FF").unwrap());
	native_camera.projection.scaling_mode = ScalingMode::FixedVertical(HEIGHT as f32);
	native_camera.camera.priority = 1;

	commands.spawn((overworld_camera, EditorRayCastSource::new()));
	commands.spawn((battle_camera, BATTLE_LAYER));
	commands.spawn((native_camera, NATIVE_LAYER));

	commands.spawn((
		SpriteBundle {
			texture: overworld_handle.clone_weak(),
			..default()
		},
		Canvas,
		NATIVE_LAYER,
	));

	commands.spawn(SpriteBundle {
		texture: ui_handle.clone_weak(),
		transform: Transform::from_xyz(0.0, 0.0, 1.0),
		..default()
	});

	commands.insert_resource(RenderTargets {
		overworld: overworld_handle,
		battle: battle_handle,
		ui: ui_handle,
	});
}
