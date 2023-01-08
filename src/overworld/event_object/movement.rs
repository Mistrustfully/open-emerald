use std::time::Duration;

use bevy::prelude::*;

use crate::lerp::Lerp;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub enum Direction {
	North,
	East,
	South,
	West,
}

#[derive(PartialEq, Debug, Clone, Copy, Hash, Eq)]
pub enum Movement {
	None,

	// 8 Frames
	Face(Direction),
	Run(Direction),

	// 16 Frames
	Walk(Direction),
}

#[derive(Component, Debug)]
pub struct MovementRecord {
	pub last: Movement,
	pub next: Movement,
	pub facing: Direction,
	pub consumed: bool,
	pub moving: bool,
	timer: Timer,
}

impl Default for MovementRecord {
	fn default() -> Self {
		Self {
			last: Movement::None,
			next: Movement::None,
			facing: Direction::South,
			consumed: false,
			moving: false,
			timer: Timer::new(duration_from_frame_count(16), TimerMode::Once),
		}
	}
}

pub fn duration_from_frame_count(frames: u8) -> Duration {
	return Duration::from_secs_f32(1.0 / 60.0 * frames as f32);
}

pub fn apply_movements(
	mut movement_query: Query<(&mut MovementRecord, &mut Lerp)>,
	time: Res<Time>,
) {
	for (mut movement_record, mut lerp) in movement_query.iter_mut() {
		movement_record.timer.tick(time.delta());
		movement_record.consumed = false;

		if movement_record.timer.finished() {
			movement_record.consumed = true;

			if movement_record.next != Movement::None {
				movement_record.timer.reset();
				let next = movement_record.next;
				let movement_duration = match next {
					Movement::Face(_) => duration_from_frame_count(8),
					Movement::Run(_) => duration_from_frame_count(8),
					Movement::Walk(_) => duration_from_frame_count(16),
					_ => duration_from_frame_count(0),
				};

				movement_record.timer.set_duration(movement_duration);

				let delta = match next {
					Movement::Walk(Direction::North) => (0., 16.),
					Movement::Walk(Direction::South) => (0., -16.),
					Movement::Walk(Direction::West) => (-16., 0.),
					Movement::Walk(Direction::East) => (16., 0.),
					Movement::Run(Direction::North) => (0., 16.),
					Movement::Run(Direction::South) => (0., -16.),
					Movement::Run(Direction::West) => (-16., 0.),
					Movement::Run(Direction::East) => (16., 0.),
					_ => (0.0, 0.0),
				};

				lerp.start = lerp.goal;
				lerp.goal = lerp.goal + Vec3::new(delta.0, delta.1, 0.0);
				lerp.elapsed = 0.0;
				lerp.duration = movement_duration;

				movement_record.facing = match next {
					Movement::Walk(dir) => dir,
					Movement::Run(dir) => dir,
					Movement::Face(dir) => dir,
					_ => Direction::South,
				}
			}

			movement_record.last = movement_record.next;
		}
	}
}
