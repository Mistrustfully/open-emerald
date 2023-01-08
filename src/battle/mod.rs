use crate::{
	camera::{Canvas, RenderTargets, BATTLE_LAYER},
	pokemon::Pokemon,
	GameState, HEIGHT,
};
use bevy::{prelude::*, sprite::Anchor};
use iyes_loopless::prelude::*;

pub struct BattlePlugin;
impl Plugin for BattlePlugin {
	fn build(&self, app: &mut App) {
		app.add_enter_system(GameState::Battle, setup_battle);
	}
}

pub fn setup_battle(
	// battle_data: Res<BattleData>,
	mut native_camera: Query<&mut Handle<Image>, With<Canvas>>,
	render_targets: Res<RenderTargets>,
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	*native_camera.get_single_mut().unwrap() = render_targets.battle.clone_weak();

	commands
		.spawn(SpriteBundle {
			texture: asset_server.load("backgrounds/grass.png"),
			transform: Transform::from_xyz(0.0, 24.0, 0.0),
			..default()
		})
		.insert(BATTLE_LAYER);
}

#[derive(Resource, Debug)]
pub struct BattleData {
	player_mon: Pokemon,
	enemy_mon: Pokemon,
}

pub fn start_battle(mut commands: Commands, player_mon: Pokemon, enemy_mon: Pokemon) {
	commands.insert_resource(BattleData {
		player_mon,
		enemy_mon,
	});

	commands.insert_resource(NextState(GameState::Battle));
}
