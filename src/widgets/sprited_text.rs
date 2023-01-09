use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::asset_loaders::font_config::FontConfig;

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

#[derive(Bundle)]
pub struct SpritedTextBundle {
	pub props: SpritedText,
	pub styles: KStyle,
	pub computed_styles: ComputedStyles,
	pub children: KChildren,
	pub on_event: OnEvent,
	pub widget_name: WidgetName,
}

impl Default for SpritedTextBundle {
	fn default() -> Self {
		Self {
			props: SpritedText::default(),
			styles: KStyle::default(),
			computed_styles: ComputedStyles::default(),
			children: KChildren::default(),
			on_event: OnEvent::default(),
			widget_name: SpritedText::default().get_name(),
		}
	}
}

pub fn render_sprited_text(
	In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
	mut commands: Commands,
	mut state_query: Query<&mut SpritedTextState>,
	props_query: Query<&SpritedText>,
	font_configs: Res<Assets<FontConfig>>,
) -> bool {
	let props = props_query.get(entity).unwrap();
	let parent_id = Some(entity);

	// println!("{:?}", font_configs);
	if let Some(font_config) = font_configs.get(&props.font_config) {
		let state_entity =
			widget_context.use_state(&mut commands, entity, SpritedTextState::default());
		if let Ok(mut state) = state_query.get_mut(state_entity) {
			if state.loaded == false {
				state.loaded = true;
			}
		}

		// println!("{:?}", font_config.layout);

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
	// if let Some(state_entity) = widget_context.get_state(entity) {
	// println!("1");
	// if let Ok(state) = widget_param.state_query.get(state_entity) {
	// println!("2");
	if let Ok(props) = widget_param.props_query.get(entity) {
		// println!("3");
		if font_configs.get(&props.font_config.clone_weak()).is_some()
		// && state.loaded == false
		{
			return true;
		}
	}
	// }
	// }

	widget_param.has_changed(&widget_context, entity, previous_entity)
}
