// SPDX-FileCopyrightText: 2023 Christian Fletcher <mistrustfully@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub mod movement;
pub mod player;

use bevy_2d_animation::{
	components::{Animator, AnimatorBuilder, Frame, RepeatMode},
	AnimationPlayer,
};
use movement::*;
use player::*;

use crate::{battle::start_battle, GameState};

use self::movement::Direction;

pub struct EventObjectPlugin;
impl Plugin for EventObjectPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugin(AnimationPlayer::<Movement>::new())
			.add_enter_system(GameState::Overworld, spawn_player)
			.add_system_set(
				ConditionSet::new()
					.run_in_state(GameState::Overworld)
					.with_system(move_player)
					.with_system(movement::apply_movements)
					.with_system(animate_event_object)
					.into(),
			);
	}
}

#[derive(Component)]
pub struct EventObject;

pub fn build_animator(
	walk_sheet: Handle<TextureAtlas>,
	opt_run_sheet: Option<Handle<TextureAtlas>>,
) -> Animator<Movement> {
	let mut animator =
		AnimatorBuilder::new(walk_sheet, duration_from_frame_count(4), RepeatMode::Pause);

	animator
		.register_animation(Movement::Face(Direction::North), vec![5, 1])
		.register_animation(Movement::Face(Direction::South), vec![3, 0])
		.register_animation(Movement::Face(Direction::West), vec![7, 2])
		.register_animation(
			Movement::Face(Direction::East),
			vec![Frame::flip_x(7), Frame::flip_x(2)],
		)
		.set_priority(1)
		.set_repeat_mode(RepeatMode::Loop)
		.set_duration(duration_from_frame_count(8))
		.register_animation(Movement::Walk(Direction::North), vec![1, 6, 1, 5])
		.register_animation(Movement::Walk(Direction::South), vec![0, 4, 0, 3])
		.register_animation(Movement::Walk(Direction::West), vec![2, 7, 2, 8])
		.register_animation(
			Movement::Walk(Direction::East),
			vec![
				Frame::flip_x(2),
				Frame::flip_x(7),
				Frame::flip_x(2),
				Frame::flip_x(8),
			],
		);

	if let Some(run_sheet) = opt_run_sheet {
		animator
			.set_spritesheet(run_sheet)
			.set_priority(2)
			.set_duration(duration_from_frame_count(4))
			.register_animation(Movement::Run(Direction::North), vec![1, 5, 1, 6])
			.register_animation(Movement::Run(Direction::South), vec![0, 3, 0, 4])
			.register_animation(Movement::Run(Direction::West), vec![2, 7, 2, 8])
			.register_animation(
				Movement::Run(Direction::East),
				vec![
					Frame::flip_x(2),
					Frame::flip_x(7),
					Frame::flip_x(2),
					Frame::flip_x(8),
				],
			);
	}

	animator.build()
}

fn animate_event_object(
	mut query: Query<(&mut Animator<Movement>, &MovementRecord), Changed<MovementRecord>>,
) {
	for (mut animator, movement_record) in query.iter_mut() {
		if movement_record.consumed {
			let animation = movement_record.last;

			animator.stop_animation_by_priority(1);
			animator.stop_animation_by_priority(2);

			if animation != Movement::None {
				animator.play_animation(&match animation {
					Movement::Walk(dir) => Movement::Face(dir),
					Movement::Run(dir) => Movement::Face(dir),
					Movement::Face(dir) => Movement::Face(dir),
					_ => Movement::Face(Direction::South),
				});

				if !matches!(animation, Movement::Face(_)) {
					animator.play_animation(&animation);
				} else {
					animator.restart_animation(&animation);
				}
			}
		}
	}
}
