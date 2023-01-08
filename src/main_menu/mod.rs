use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;
use kayak_ui::prelude::{widgets::*, *};

use crate::{
	widgets::menu_button::{MenuButtonBundle, MenuButtonProps},
	GameState,
};

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
	fn build(&self, app: &mut App) {
		app.add_enter_system(GameState::Menu, startup)
			.add_system(navigate);
	}
}

#[derive(Component, Clone, PartialEq, Eq, Default)]
pub struct Menu {
	selected_button: usize,
}
impl Widget for Menu {}

#[derive(Component, Default, Clone, PartialEq, Eq)]
pub struct MenuState {
	selected_button: usize,
}

#[derive(Bundle)]
pub struct MenuBundle {
	pub props: Menu,
	pub styles: KStyle,
	pub computed_styles: ComputedStyles,
	pub children: KChildren,
	pub on_event: OnEvent,
	pub widget_name: WidgetName,
}

impl Default for MenuBundle {
	fn default() -> Self {
		Self {
			props: Menu::default(),
			styles: KStyle::default(),
			computed_styles: ComputedStyles::default(),
			children: KChildren::default(),
			on_event: OnEvent::default(),
			widget_name: Menu::default().get_name(),
		}
	}
}

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
					height: StyleProp::Value(Units::Pixels(160.0)),
					..default()
				}}
								>
				<MenuButtonBundle
					props={MenuButtonProps {
						text: "Single Player".into(),
						selected: menu.selected_button == 0,
					}}
				/>
				<MenuButtonBundle
					props={MenuButtonProps {
						text: "Multiplayer".into(),
						selected: menu.selected_button == 1,
					}}
				/>
				<MenuButtonBundle
					props={MenuButtonProps {
						text: "Options".into(),
						selected: menu.selected_button == 2,
				}}
			/>
		</BackgroundBundle>
	};

	true
}

fn navigate(mut query: Query<&mut Menu, Without<PreviousWidget>>, keys: Res<Input<KeyCode>>) {
	let mut menu = query.get_single_mut().expect("Expected Menu!");

	menu.selected_button = (if keys.just_pressed(KeyCode::Down) {
		menu.selected_button + 1
	} else if keys.just_pressed(KeyCode::Up) {
		menu.selected_button.checked_sub(1).unwrap_or(0)
	} else {
		menu.selected_button
	})
	.clamp(0, 2);

	println!("{}", menu.selected_button);
}

fn startup(
	mut commands: Commands,
	mut font_mapping: ResMut<FontMapping>,
	asset_server: Res<AssetServer>,
	mut widget_context_query: Query<&mut KayakRootContext>,
) {
	font_mapping.set_default(asset_server.load("font.kayak_font"));

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
			< MenuBundle />

		</KayakAppBundle>
	};
}
