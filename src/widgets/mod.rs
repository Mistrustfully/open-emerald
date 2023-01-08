use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use kayak_ui::{prelude::*, widgets::*};
use menu_button::*;

pub mod menu_button;

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

		let mut camera = UICameraBundle::new(widget_context);
		// Allows for transparent UI elements.
		camera.camera_ui.clear_color = ClearColorConfig::None;

		app.world.spawn((camera, UICamera));
	}
}
