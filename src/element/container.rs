use crate::render_context::RenderContext;
use crate::{Component, element::Element};
use clay_layout::{
	Color, Declaration,
	layout::{Alignment, LayoutDirection, Padding, Sizing},
};
pub type Justify = clay_layout::layout::LayoutAlignmentX;
pub type Align = clay_layout::layout::LayoutAlignmentY;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Direction {
	#[default]
	Row,
	Column,
}
#[derive(Debug, Clone)]
pub struct ContainerStyle {
	pub background_color: Color,
	pub border_radius: (f32, f32, f32, f32),
	pub size: (Sizing, Sizing),
	pub gap: u16,
	pub align: Align,
	pub justify: Justify,
	pub direction: Direction,
	pub padding: (u16, u16, u16, u16),
}
impl Default for ContainerStyle {
	fn default() -> Self {
		Self {
			padding: (0, 0, 0, 0),
			background_color: Color::rgba(0., 0., 0., 0.),
			border_radius: (0., 0., 0., 0.),
			size: (Sizing::Grow(0., f32::MAX), Sizing::Fit(0., f32::MAX)),
			gap: 0,
			align: Align::Top,
			justify: Justify::Left,
			direction: Direction::Row,
		}
	}
}
/// A generic container element that can hold other elements.
///
/// This container element is designed to be flexible and can be used to create a variety of layouts.
/// It supports various styling options such as background color, border radius, size, gap, alignment, and direction.
/// This is the equivalent of a `<div>` element with `display: flex´ in HTML, and can be used to build a variety of different components.
///
/// If you need the container to be interactive, you can nest a `Clickable` element to handle user interactions.
pub struct Container {
	pub children: Vec<Box<dyn Element>>,
	pub style: ContainerStyle,
	pub style_if_hovered: Box<dyn Fn(ContainerStyle) -> ContainerStyle>,
}

impl Default for Container {
	fn default() -> Self {
		Self {
			children: Vec::new(),
			style: ContainerStyle::default(),
			style_if_hovered: Box::new(|style| style),
		}
	}
}

impl Container {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn child(mut self, element: impl Element + 'static) -> Self {
		self.children.push(Box::new(element));
		self
	}
	pub fn component(mut self, component: impl Into<Component>) -> Self {
		self.children.push(Box::new(component.into()));
		self
	}
	pub fn background_color(mut self, color: impl Into<Color>) -> Self {
		self.style.background_color = color.into();
		self
	}

	pub fn w_expand(mut self) -> Self {
		self.style.size.0 = Sizing::Grow(0., f32::MAX);
		self
	}
	pub fn h_expand(mut self) -> Self {
		self.style.size.1 = Sizing::Grow(0., f32::MAX);
		self
	}
	pub fn w_fit(mut self) -> Self {
		self.style.size.0 = Sizing::Fit(0., f32::MAX);
		self
	}
	pub fn min_width(mut self, width: f32) -> Self {
		self.style.size.0 = match self.style.size.0 {
			Sizing::Fit(_, max) => Sizing::Fit(width, max),
			Sizing::Fixed(size) => Sizing::Fixed(size.min(width)),
			Sizing::Grow(_, max) => Sizing::Grow(width, max),
			o => o,
		};
		self
	}

	pub fn min_height(mut self, height: f32) -> Self {
		self.style.size.1 = match self.style.size.1 {
			Sizing::Fit(_, max) => Sizing::Fit(height, max),
			Sizing::Fixed(size) => Sizing::Fixed(size.min(height)),
			Sizing::Grow(_, max) => Sizing::Grow(height, max),
			o => o,
		};
		self
	}

	pub fn max_width(mut self, width: f32) -> Self {
		self.style.size.0 = match self.style.size.0 {
			Sizing::Fit(min, _) => Sizing::Fit(min, width),
			Sizing::Fixed(size) => Sizing::Fixed(size.min(width)),
			Sizing::Grow(min, _) => Sizing::Grow(min, width),
			o => o,
		};
		self
	}

	pub fn max_height(mut self, height: f32) -> Self {
		self.style.size.1 = match self.style.size.1 {
			Sizing::Fit(min, _) => Sizing::Fit(min, height),
			Sizing::Fixed(size) => Sizing::Fixed(size.min(height)),
			Sizing::Grow(min, _) => Sizing::Grow(min, height),
			o => o,
		};
		self
	}

	pub fn gap(mut self, gap: u16) -> Self {
		self.style.gap = gap;
		self
	}

	pub fn align(mut self, align: Align) -> Self {
		self.style.align = align;
		self
	}

	pub fn justify(mut self, justify: Justify) -> Self {
		self.style.justify = justify;
		self
	}

	pub fn center(mut self) -> Self {
		self.style.align = Align::Center;
		self.style.justify = Justify::Center;
		self
	}

	pub fn direction(mut self, direction: Direction) -> Self {
		self.style.direction = direction;
		self
	}

	// Convenience methods for common patterns
	pub fn row() -> Self {
		Self::new().direction(Direction::Row)
	}

	pub fn column() -> Self {
		Self::new().direction(Direction::Column)
	}

	pub fn weird_padding(mut self, top: u16, right: u16, bottom: u16, left: u16) -> Self {
		self.style.padding = (left, right, top, bottom);
		self
	}

	pub fn symmetric_padding(mut self, horizontal: u16, vertical: u16) -> Self {
		self.style.padding = (horizontal, horizontal, vertical, vertical);
		self
	}

	pub fn padding_all(mut self, all: u16) -> Self {
		self.style.padding = (all, all, all, all);
		self
	}
	pub fn rounded_l(mut self, left_radius: f32) -> Self {
		self.style.border_radius.0 = left_radius;
		self.style.border_radius.2 = left_radius;
		self
	}
	pub fn rounded_r(mut self, right_radius: f32) -> Self {
		self.style.border_radius.1 = right_radius;
		self.style.border_radius.3 = right_radius;
		self
	}
	pub fn rounded_b(mut self, bottom_radius: f32) -> Self {
		self.style.border_radius.2 = bottom_radius;
		self.style.border_radius.3 = bottom_radius;
		self
	}
	pub fn rounded_t(mut self, top_radius: f32) -> Self {
		self.style.border_radius.0 = top_radius;
		self.style.border_radius.1 = top_radius;
		self
	}

	pub fn rounded(mut self, radius: f32) -> Self {
		self.style.border_radius.0 = radius;
		self.style.border_radius.1 = radius;
		self.style.border_radius.2 = radius;
		self.style.border_radius.3 = radius;
		self
	}
	pub fn style_if_hovered<F>(mut self, f: F) -> Self
	where
		F: Fn(ContainerStyle) -> ContainerStyle + 'static,
	{
		self.style_if_hovered = Box::new(f);
		self
	}
}

impl Element for Container {
	fn render<'clay: 'render, 'render>(&'render self, ctx: &mut RenderContext<'clay, 'render, '_>) {
		ctx.c.with_styling(
			|c| {
				let mut declaration = Declaration::new();
				let effective_style = if c.hovered() {
					(self.style_if_hovered)(self.style.clone())
				} else {
					self.style.clone()
				};
				declaration
					.layout()
					.direction(match effective_style.direction {
						Direction::Row => LayoutDirection::LeftToRight,
						Direction::Column => LayoutDirection::TopToBottom,
					})
					.width(effective_style.size.0)
					.height(effective_style.size.1)
					.child_gap(effective_style.gap)
					.child_alignment(Alignment::new(
						effective_style.justify,
						effective_style.align,
					))
					.padding(Padding::new(
						effective_style.padding.0,
						effective_style.padding.1,
						effective_style.padding.2,
						effective_style.padding.3,
					))
					.end()
					.corner_radius()
					.top_left(effective_style.border_radius.0)
					.top_right(effective_style.border_radius.1)
					.bottom_left(effective_style.border_radius.2)
					.bottom_right(effective_style.border_radius.3)
					.end()
					.background_color(effective_style.background_color);
				declaration
			},
			|c| {
				let mut child_ctx = RenderContext {
					c,
					font_manager: ctx.font_manager,
					input_manager: ctx.input_manager,
				};
				for child in &self.children {
					child.render(&mut child_ctx);
				}
			},
		);
	}
}
