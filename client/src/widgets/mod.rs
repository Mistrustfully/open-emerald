use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use kayak_ui::{prelude::*, widgets::*};
use menu_button::*;

use crate::camera::TRANSPARENT;

use self::sprited_text::{render_sprited_text, sprited_text_update, SpritedText, SpritedTextState};

pub mod menu_button;
pub mod sprited_text;

#[derive(Component)]
pub struct UICamera;

#[macro_export]
macro_rules! KBundle {
	($i:ident, $typ:ty) => {
		#[derive(Bundle)]
		pub struct $i {
			pub props: $typ,
			pub styles: KStyle,
			pub computed_styles: ComputedStyles,
			pub children: KChildren,
			pub on_event: OnEvent,
			pub widget_name: WidgetName,
		}

		impl Default for $i {
			fn default() -> Self {
				Self {
					props: <$typ>::default(),
					styles: KStyle::default(),
					computed_styles: ComputedStyles::default(),
					children: KChildren::default(),
					on_event: OnEvent::default(),
					widget_name: <$typ>::default().get_name(),
				}
			}
		}
	};
}

pub struct WidgetPlugin;
impl Plugin for WidgetPlugin {
	fn build(&self, app: &mut App) {
		let mut widget_context = KayakRootContext::new();
		widget_context.add_plugin(KayakWidgetsContextPlugin);

		widget_context.add_widget_data::<SpritedText, SpritedTextState>();
		widget_context.add_widget_system(
			SpritedText::default().get_name(),
			sprited_text_update,
			render_sprited_text,
		);

		widget_context.add_widget_data::<MenuButtonProps, EmptyState>();
		widget_context.add_widget_system(
			MenuButtonProps::default().get_name(),
			widget_update::<MenuButtonProps, EmptyState>,
			menu_button_render,
		);

		let mut camera = UICameraBundle::new(widget_context);
		// Allows for transparent UI elements.
		camera.camera_ui.clear_color = ClearColorConfig::Custom(TRANSPARENT);

		app.world.spawn((camera, UICamera));
	}
}
