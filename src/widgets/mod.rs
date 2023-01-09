use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use kayak_ui::{prelude::*, widgets::*};
use menu_button::*;

use crate::camera::TRANSPARENT;

use self::sprited_text::{render_sprited_text, sprited_text_update, SpritedText, SpritedTextState};

pub mod menu_button;
pub mod sprited_text;

#[derive(Component)]
pub struct UICamera;

pub struct WidgetPlugin;
impl Plugin for WidgetPlugin {
	fn build(&self, app: &mut App) {
		let mut widget_context = KayakRootContext::new();
		widget_context.add_plugin(KayakWidgetsContextPlugin);

		widget_context.add_widget_data::<MenuButtonProps, MenuButtonState>();
		widget_context.add_widget_system(
			MenuButtonProps::default().get_name(),
			widget_update::<MenuButtonProps, MenuButtonState>,
			menu_button_render,
		);

		widget_context.add_widget_data::<SpritedText, SpritedTextState>();
		widget_context.add_widget_system(
			SpritedText::default().get_name(),
			sprited_text_update,
			render_sprited_text,
		);

		let mut camera = UICameraBundle::new(widget_context);
		// Allows for transparent UI elements.
		camera.camera_ui.clear_color = ClearColorConfig::Custom(TRANSPARENT);

		app.world.spawn((camera, UICamera));
	}
}
