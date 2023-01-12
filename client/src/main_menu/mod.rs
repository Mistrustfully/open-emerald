// SPDX-FileCopyrightText: 2023 Christian Fletcher <mistrustfully@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;
use kayak_ui::prelude::{widgets::*, *};

use crate::{
	widgets::menu_button::{MenuButtonBundle, MenuButtonProps},
	GameState, KBundle,
};

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
	fn build(&self, app: &mut App) {
		app.add_enter_system(GameState::Menu, startup)
			.add_system(navigate);
	}
}

#[derive(Clone, PartialEq, Eq, Default)]
pub enum MenuWindow {
	#[default]
	Main,
	Option,
	Multiplayer,
}

#[derive(Component, Clone, PartialEq, Eq, Default)]
pub struct Menu {
	selected_button: usize,
	window: MenuWindow,
}
impl Widget for Menu {}

#[derive(Component, Default, Clone, PartialEq, Eq)]
pub struct MenuState {
	selected_button: usize,
}

KBundle!(MenuBundle, Menu);

fn render_menu(
	In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
	mut commands: Commands,
	menu: Query<&Menu>, // state: Query<&mut MenuState>,
) -> bool {
	let menu = menu.get(entity).unwrap();
	let parent_id = Some(entity);

	rsx! {
		<BackgroundBundle
				styles={KStyle {
					height: StyleProp::Value(Units::Pixels(112.0)),
					..default()
				}}
		>
			{
				match menu.window {
					MenuWindow::Main => { constructor! {
						<ElementBundle>
							<MenuButtonBundle
								props={MenuButtonProps {
									text: "SINGLE PLAYER".into(),
									selected: menu.selected_button == 0,
								}}
							/>
							<MenuButtonBundle
								props={MenuButtonProps {
									text: "MULTIPLAYER".into(),
									selected: menu.selected_button == 1,
								}}
							/>
							<MenuButtonBundle
								props={MenuButtonProps {
									text: "OPTION".into(),
									selected: menu.selected_button == 2,
								}}
							/>
						</ ElementBundle>
					}; },

					MenuWindow::Option => { constructor! {
						<ElementBundle>
							<MenuButtonBundle
								props={MenuButtonProps {
									text: "OPTIONS".into(),
									selected: menu.selected_button == 0,
								}}
							/>
							<MenuButtonBundle
								props={MenuButtonProps {
									text: "OPTIONS".into(),
									selected: menu.selected_button == 0,
								}}
								styles={KStyle {
									height: StyleProp::Value(Units::Pixels(96.0)),
									..default()
								}}
							/>
						</ ElementBundle>
					}; },

					MenuWindow::Multiplayer => { constructor! {
						<ElementBundle>
							<MenuButtonBundle
								props={MenuButtonProps {
									text: "MULTIPLAYER".into(),
									selected: menu.selected_button == 0,
								}}
							/>
						</ ElementBundle>
					}; }
				}
			}
		</BackgroundBundle>
	};

	true
}

fn navigate(mut query: Query<&mut Menu, Without<PreviousWidget>>, keys: Res<Input<KeyCode>>) {
	if let Ok(mut menu) = query.get_single_mut() {
		menu.selected_button = if keys.just_pressed(KeyCode::Down) {
			menu.selected_button + 1
		} else if keys.just_pressed(KeyCode::Up) {
			menu.selected_button.checked_sub(1).unwrap_or(0)
		} else {
			menu.selected_button
		}
		.clamp(0, 2);

		if keys.just_pressed(KeyCode::Z) {
			menu.window = MenuWindow::Option;
		};
	}
}

fn startup(mut commands: Commands, mut widget_context_query: Query<&mut KayakRootContext>) {
	let mut widget_context = widget_context_query.get_single_mut().unwrap();
	widget_context.add_widget_data::<Menu, MenuState>();
	widget_context.add_widget_system(
		Menu::default().get_name(),
		widget_update::<Menu, MenuState>,
		render_menu,
	);

	let parent_id = None;

	rsx! {
		<KayakAppBundle>
			< MenuBundle props={Menu { window: MenuWindow::Main, selected_button: 0}} />
		</KayakAppBundle>
	};
}
