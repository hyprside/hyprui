use crate::{State, begin_component, end_component, use_state};
use crate::{element::Element,  render_context::RenderContext};

/// Estado interno do Clickable para tracking de hover/press
#[derive(Default, Clone, Copy)]
struct ClickableState {
	hovered: bool,
	pressed: bool,
	right_pressed: bool,
}
/// Turns the parent container into a clickable element.

pub struct Clickable {
	pub id: String,
	pub child: Box<dyn Element>,
	pub on_click: Option<Box<dyn Fn()>>,
	pub on_mouse_enter: Option<Box<dyn Fn()>>,
	pub on_mouse_leave: Option<Box<dyn Fn()>>,
	pub on_right_click: Option<Box<dyn Fn()>>,
	state: State<ClickableState>,
}

impl Clickable {
	pub fn new(id: impl Into<String>, child: impl Element + 'static) -> Self {
		begin_component(format!("{}\0{}\0{}", file!(), line!(), module_path!()));
		let state = use_state(ClickableState::default());
		end_component();
		Self {
			id: id.into(),
			child: Box::new(child),
			on_click: None,
			on_mouse_enter: None,
			on_mouse_leave: None,
			on_right_click: None,
			state,
		}
	}
	pub fn on_click(mut self, handler: impl Fn() + 'static) -> Self {
		self.on_click = Some(Box::new(handler));
		self
	}

	pub fn on_mouse_enter(mut self, handler: impl Fn() + 'static) -> Self {
		self.on_mouse_enter = Some(Box::new(handler));
		self
	}

	pub fn on_mouse_leave(mut self, handler: impl Fn() + 'static) -> Self {
		self.on_mouse_leave = Some(Box::new(handler));
		self
	}

	pub fn on_right_click(mut self, handler: impl Fn() + 'static) -> Self {
		self.on_right_click = Some(Box::new(handler));
		self
	}
}
impl Element for Clickable {
	fn render<'clay: 'render, 'render>(&'render self, ctx: &mut RenderContext<'clay, 'render, '_>) {
		let is_clicked = ctx.input_manager.is_mouse_button_just_pressed(0) && ctx.c.hovered();
		if is_clicked != self.state.0.pressed {
			(self.state.1)(ClickableState {
				pressed: is_clicked,
				..self.state.0
			});
		}
		if let Some(on_click) = &self.on_click {
			if is_clicked {
				on_click();
			}
		}
		let is_right_clicked = ctx.input_manager.is_mouse_button_just_pressed(1) && ctx.c.hovered();
		if is_right_clicked != self.state.0.right_pressed {
			(self.state.1)(ClickableState {
				right_pressed: is_right_clicked,
				..self.state.0
			});
		}
		if let Some(on_right_click) = &self.on_right_click {
			if is_right_clicked {
				on_right_click();
			}
		}
		let is_hovered = ctx.c.hovered();
		if is_hovered != self.state.0.hovered {
			(self.state.1)(ClickableState {
				hovered: is_hovered,
				..self.state.0
			});
			if is_hovered {
				if let Some(on_mouse_enter) = &self.on_mouse_enter {
					on_mouse_enter();
				}
			} else {
				if let Some(on_mouse_leave) = &self.on_mouse_leave {
					on_mouse_leave();
				}
			}
		}
		self.child.render(ctx);
	}
}
