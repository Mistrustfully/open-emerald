use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::{
	widgets::sprited_text::{SpritedText, SpritedTextBundle},
	KBundle,
};

#[derive(Component, Clone, PartialEq, Eq, Default)]
pub struct MenuButtonProps {
	pub text: String,
	pub selected: bool,
}
impl Widget for MenuButtonProps {}

KBundle!(MenuButtonBundle, MenuButtonProps);

pub fn menu_button_render(
	In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
	mut commands: Commands,
	menu_button_query: Query<&MenuButtonProps>,
	asset_server: Res<AssetServer>,
) -> bool {
	let props = menu_button_query.get(entity).unwrap();
	let parent_id = Some(entity);

	let color = if props.selected {
		Color::hex("ffffff").unwrap()
	} else {
		Color::hex("8C8E8C").unwrap()
	};

	let text_color = if props.selected {
		Color::hex("8C8E8C").unwrap()
	} else {
		Color::hex("ffffff").unwrap()
	};

	rsx! {
		<NineTiledBundle
			nine_patch={NineTiled { handle: asset_server.load("text_window/2.png"), border: Edge::all(8.0) }}
			styles={KStyle {
				width: StyleProp::Value(Units::Pixels(224.0)),
				height: StyleProp::Value(Units::Pixels(32.0)),
				left: StyleProp::Value(Units::Pixels(8.0)),
				right: StyleProp::Value(Units::Pixels(8.0)),
				bottom: StyleProp::Value(Units::Pixels(4.0)),
				top: StyleProp::Value(Units::Pixels(4.0)),
				color: StyleProp::Value(color),
				..default()
			}}
		>
			<SpritedTextBundle
				props={SpritedText {
					text: props.text.clone(),
					font: asset_server.load("fonts/latin_normal.png"),
					font_config: asset_server.load("fonts/latin_normal.font_config"),
				}}
				styles={KStyle {
					color: StyleProp::Value(text_color),
					..default()
				}}
			/>
		</NineTiledBundle>
	};

	true
}
