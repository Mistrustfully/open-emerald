use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::{asset_loaders::font_config::FontConfig, KBundle};

#[derive(Component, Clone, PartialEq, Eq, Default)]
pub struct SpritedText {
	pub text: String,
	pub font: Handle<Image>,
	pub font_config: Handle<FontConfig>,
}
impl Widget for SpritedText {}

#[derive(Component, Clone, PartialEq, Eq, Default)]
pub struct SpritedTextState {
	loaded: bool,
}

KBundle!(SpritedTextBundle, SpritedText);

pub fn render_sprited_text(
	In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
	mut commands: Commands,
	props_query: Query<&SpritedText>,
	// style_query: Query<&KStyle>,
	font_configs: Res<Assets<FontConfig>>,
) -> bool {
	let props = props_query.get(entity).unwrap();
	let parent_id = Some(entity);

	if let Some(font_config) = font_configs.get(&props.font_config) {
		widget_context.use_state(&mut commands, entity, SpritedTextState { loaded: true });
		// let style = style_query.get(entity).unwrap();

		rsx! {
			<ElementBundle
				styles={KStyle {
					layout_type: LayoutType::Row.into(),
					left: StyleProp::Value(Units::Pixels(8.0)),
					top: StyleProp::Value(Units::Pixels(8.0)),
					..default()
				}}
			>
				{
					for char in props.text.chars() {
						let position = font_config.layout.iter().position(|each| *each == char.to_string()).expect("Couldn't find character!");
						let x = (position % 16) * 16;
						let y = (position / 16) * 16;

						let size_x = font_config.spacing.get(&char.to_string()).unwrap() + 0;

						constructor! {
							<TextureAtlasBundle
								atlas={TextureAtlasProps {
									handle: props.font.clone_weak(),
									position: Vec2::new(x as f32, y as f32),
									tile_size: Vec2::new(size_x as f32, 16.0),
								}}
								styles={ KStyle {
									height: StyleProp::Value(Units::Pixels(16.0)),
									width: StyleProp::Value(Units::Pixels(size_x as f32)),
									..default()
								}}
							/>
						}
					}
				}
			</ ElementBundle>
		};
	}

	true
}

pub fn sprited_text_update(
	In((widget_context, entity, previous_entity)): In<(KayakWidgetContext, Entity, Entity)>,
	widget_param: WidgetParam<SpritedText, SpritedTextState>,
	font_configs: Res<Assets<FontConfig>>,
) -> bool {
	return if let Ok(props) = widget_param.props_query.get(entity) {
		if let Some(state_entity) = widget_context.get_state(entity) {
			if let Ok(state) = widget_param.state_query.get(state_entity) {
				if state.loaded {
					widget_param.has_changed(&widget_context, entity, previous_entity);
				}
			}
		}

		if font_configs.get(&props.font_config.clone_weak()).is_some() {
			true
		} else {
			widget_param.has_changed(&widget_context, entity, previous_entity)
		}
	} else {
		widget_param.has_changed(&widget_context, entity, previous_entity)
	};
}
