use bevy::prelude::*;
use bevy_2d_animation::components::{AnimatorBuilder, Frame};

use super::{
	build_animator,
	movement::{duration_from_frame_count, Direction, Movement, MovementRecord},
	EventObject,
};
use crate::{battle::start_battle, lerp::Lerp};

#[derive(Component)]
pub struct Player {
	facing_timer: Timer,
	last_direction: Direction,
}

pub fn spawn_player(
	mut commands: Commands,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	asset_server: Res<AssetServer>,
) {
	let walk_handle = asset_server.load("people/may/walk.png");
	let walk_atlas = TextureAtlas::from_grid(walk_handle, Vec2::new(16.0, 32.0), 9, 1, None, None);
	let walk_atlas_handle = texture_atlases.add(walk_atlas);

	let run_handle = asset_server.load("people/may/running.png");
	let run_atlas = TextureAtlas::from_grid(run_handle, Vec2::new(16.0, 32.0), 9, 1, None, None);
	let run_atlas_handle = texture_atlases.add(run_atlas);

	commands.spawn((
		SpriteSheetBundle {
			texture_atlas: walk_atlas_handle.clone(),
			..default()
		},
		Player {
			last_direction: Direction::South,
			facing_timer: Timer::new(duration_from_frame_count(8), TimerMode::Once),
		},
		EventObject,
		MovementRecord::default(),
		Lerp::default(),
		build_animator(walk_atlas_handle, Some(run_atlas_handle)),
	));
}

pub fn move_player(
	mut movement_query: Query<(&mut MovementRecord, &mut Player)>,
	commands: Commands,
	keys: Res<Input<KeyCode>>,
	time: Res<Time>,
) {
	if keys.just_pressed(KeyCode::Space) {
		start_battle(
			commands,
			crate::pokemon::Pokemon::Flaaffy,
			crate::pokemon::Pokemon::PorygonZ,
		);

		return;
	}

	let (mut movement_record, mut player) = movement_query.get_single_mut().unwrap();

	let mut next_direction = None;

	for keycode in keys.get_pressed() {
		let next = match keycode {
			KeyCode::Up => Some(Direction::North),
			KeyCode::Right => Some(Direction::East),
			KeyCode::Left => Some(Direction::West),
			KeyCode::Down => Some(Direction::South),
			_ => None,
		};

		if next.is_some() {
			next_direction = next;
		}
	}

	if let Some(direction) = next_direction {
		let direction_matches = match movement_record.next {
			Movement::Walk(dir) => dir == direction,
			Movement::Run(dir) => dir == direction,
			Movement::Face(dir) => dir == direction,
			_ => false,
		} || player.last_direction == direction;

		if direction_matches {
			player.facing_timer.tick(time.delta());
		} else {
			player.facing_timer.reset();
		}

		movement_record.next = if keys.pressed(KeyCode::X) {
			Movement::Run(direction)
		} else if player.facing_timer.finished() {
			Movement::Walk(direction)
		} else {
			Movement::Face(direction)
		};

		player.last_direction = direction
	} else {
		movement_record.next = Movement::None;
	}
}
