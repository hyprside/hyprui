use crate::{InputManager, font_manager::FontManager};
use clay_layout::ClayLayoutScope;
use skia_safe::Image;

pub struct RenderContext<'clay: 'render, 'render: 'a, 'a> {
	pub c: &'a mut ClayLayoutScope<'clay, 'render, Image, ()>,
	pub font_manager: &'a mut FontManager,
	pub input_manager: &'a dyn InputManager,
}
