use crate::{Container, InputManager};

/// Estado interno do Clickable para tracking de hover/press
#[derive(Default, Clone, Copy)]
pub struct ClickableState {
	pub hovered: bool,
	pub pressed: bool,
	pub right_pressed: bool,
}
/// Turns the parent container into a clickable element.

pub(crate) struct Clickable {
	pub(crate) on_click: Option<Box<dyn Fn()>>,
	pub(crate) on_mouse_enter: Option<Box<dyn Fn()>>,
	pub(crate) on_mouse_leave: Option<Box<dyn Fn()>>,
	pub(crate) on_right_click: Option<Box<dyn Fn()>>,
}

impl Clickable {
	pub fn new() -> Self {
		Self {
			on_click: None,
			on_mouse_enter: None,
			on_mouse_leave: None,
			on_right_click: None,
		}
	}
	pub fn update(
		&self,
		input_manager: &dyn InputManager,
		state: &mut ClickableState,
		is_hovered: bool,
	) {
		let is_clicked = input_manager.is_mouse_button_just_pressed(0) && is_hovered;
		if is_clicked != state.pressed {
			state.pressed = is_clicked;
		}
		if let Some(on_click) = &self.on_click {
			if is_clicked {
				on_click();
			}
		}
		let is_right_clicked = input_manager.is_mouse_button_just_pressed(1) && is_hovered;
		if is_right_clicked != state.right_pressed {
			state.right_pressed = is_right_clicked;
		}
		if let Some(on_right_click) = &self.on_right_click {
			if is_right_clicked {
				on_right_click();
			}
		}
		if is_hovered != state.hovered {
			state.hovered = is_hovered;
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
	}
}
impl Container {
	fn ensure_clickable(&mut self) {
		if self.clickable.is_none() {
			self.clickable = Some(Clickable::new());
		}
	}
	pub fn on_click(mut self, handler: impl Fn() + 'static) -> Self {
		self.ensure_clickable();
		self.clickable.as_mut().unwrap().on_click = Some(Box::new(handler));
		self
	}

	pub fn on_mouse_enter(mut self, handler: impl Fn() + 'static) -> Self {
		self.ensure_clickable();
		self.clickable.as_mut().unwrap().on_mouse_enter = Some(Box::new(handler));
		self
	}

	pub fn on_mouse_leave(mut self, handler: impl Fn() + 'static) -> Self {
		self.ensure_clickable();
		self.clickable.as_mut().unwrap().on_mouse_leave = Some(Box::new(handler));
		self
	}

	pub fn on_right_click(mut self, handler: impl Fn() + 'static) -> Self {
		self.ensure_clickable();
		self.clickable.as_mut().unwrap().on_right_click = Some(Box::new(handler));
		self
	}
}
