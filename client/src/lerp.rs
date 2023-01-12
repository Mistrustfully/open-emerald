// SPDX-FileCopyrightText: 2023 Christian Fletcher <mistrustfully@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::time::Duration;

use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Lerp {
	pub goal: Vec3,
	pub duration: Duration,
	pub elapsed: f32,
	pub start: Vec3,
}

pub fn update_lerp(mut query: Query<(&mut Transform, &mut Lerp)>, time: Res<Time>) {
	for (mut transform, mut lerp) in query.iter_mut() {
		let percentage = lerp.elapsed / lerp.duration.as_secs_f32();
		transform.translation = lerp.start.lerp(lerp.goal, percentage);

		lerp.elapsed += time.delta_seconds();
		if lerp.elapsed > lerp.duration.as_secs_f32() {
			transform.translation = lerp.goal;
		}
	}
}
