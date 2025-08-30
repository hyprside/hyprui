use skia_safe::{FontStyle, font_style::Width};

use crate::{Element, RenderContext};

pub struct Text {
	pub text: String,
	pub font_family: String,
	pub font_weight: i32,
	pub italic: bool,
	pub font_size: u16,
	pub color: clay_layout::Color,
}

impl Text {
	pub fn new(text: impl Into<String>) -> Self {
		Self {
			text: text.into(),
			font_family: "Inter".to_string(),
			font_weight: 400,
			font_size: 14,
			color: (0, 0, 0, 255).into(),
			italic: false,
		}
	}

	pub fn font_size(mut self, size: u16) -> Self {
		self.font_size = size;
		self
	}

	pub fn color(mut self, color: clay_layout::Color) -> Self {
		self.color = color;
		self
	}

	pub fn italic(mut self, italic: bool) -> Self {
		self.italic = italic;
		self
	}

	pub fn font_family(mut self, family: impl Into<String>) -> Self {
		self.font_family = family.into();
		self
	}
}

impl Element for Text {
	fn render<'clay: 'render, 'render>(&'render self, ctx: &mut RenderContext<'clay, 'render, '_>) {
		let skia_font_style = FontStyle::new(
			self.font_weight.into(),
			Width::NORMAL,
			if self.italic {
				skia_safe::font_style::Slant::Italic
			} else {
				skia_safe::font_style::Slant::Upright
			},
		);
		let text_config = clay_layout::text::TextConfig::new()
			.font_size(self.font_size)
			.color(self.color.clone())
			.font_id(ctx.font_manager.get(&self.font_family, skia_font_style))
			.end();
		ctx.font_manager.update_clay_measure_function(&mut ctx.c);
		ctx.c.text(&self.text, text_config);
	}
}
