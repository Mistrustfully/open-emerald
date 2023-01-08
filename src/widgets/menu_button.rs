use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

#[derive(Component, Clone, PartialEq, Eq, Default)]
pub struct MenuButtonProps {
	pub text: String,
	pub selected: bool,
}
impl Widget for MenuButtonProps {}

#[derive(Component, Clone, PartialEq, Eq, Default)]
pub struct MenuButtonState {
	pub current_button_index: usize,
}

#[derive(Bundle)]
pub struct MenuButtonBundle {
	pub props: MenuButtonProps,
	pub styles: KStyle,
	pub computed_styles: ComputedStyles,
	pub children: KChildren,
	pub on_event: OnEvent,
	pub widget_name: WidgetName,
}

impl Default for MenuButtonBundle {
	fn default() -> Self {
		Self {
			props: MenuButtonProps::default(),
			styles: KStyle {
				left: StyleProp::Value(Units::Pixels(8.0)),
				right: StyleProp::Value(Units::Pixels(8.0)),

				..default()
			},
			computed_styles: ComputedStyles::default(),
			children: KChildren::default(),
			on_event: OnEvent::default(),
			widget_name: MenuButtonProps::default().get_name(),
		}
	}
}

pub fn menu_button_render(
	In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
	mut commands: Commands,
	menu_button_query: Query<&MenuButtonProps>,
	state: Query<&MenuButtonState>,
	asset_server: Res<AssetServer>,
) -> bool {
	let state_entity = widget_context.use_state(
		&mut commands,
		entity,
		MenuButtonState {
			current_button_index: 0,
		},
	);

	if let Ok(_) = state.get(state_entity) {
		let props = menu_button_query.get(entity).unwrap();
		let parent_id = Some(entity);

		let color = if props.selected { 1.0 } else { 0.3 };

		rsx! {
			<NineTiledBundle
				nine_patch={NineTiled { handle: asset_server.load("text_window/8.png"), border: Edge::all(8.0) }}
				styles={KStyle {
					width: StyleProp::Value(Units::Pixels(224.0)),
					height: StyleProp::Value(Units::Pixels(48.0)),
					left: StyleProp::Value(Units::Pixels(8.0)),
					right: StyleProp::Value(Units::Pixels(8.0)),
					bottom: StyleProp::Value(Units::Pixels(4.0)),
					top: StyleProp::Value(Units::Pixels(4.0)),
					color: StyleProp::Value(Color::Rgba {
						red: color,
						green: color,
						blue: color,
						alpha: if color == 0.3 { 0.05 } else { 1.0 },
					}),
					..default()
				}}
			>
				<TextWidgetBundle
					text={TextProps {
						content: props.text.clone(),
						size: 16.0,
						alignment: Alignment::Middle,
						..default()
					}}
					styles={KStyle {
						color: StyleProp::Value(Color::Rgba {
							red: 0.0,
							green: 0.0,
							blue: 0.0,
							alpha: 0.0,
						}),
						top: StyleProp::Value(Units::Stretch(1.0)),
						bottom: StyleProp::Value(Units::Stretch(1.0)),
						..default()
					}}
				/>
			</NineTiledBundle>
		};
	};

	true
}
