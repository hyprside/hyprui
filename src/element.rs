pub mod clickable;
pub mod component;
pub mod container;
pub mod text;
use crate::render_context::RenderContext;
/// The core trait for all renderable UI elements in HyprUI.
///
/// Any type that implements `Element` can be rendered as part of the UI tree.
/// This trait is typically implemented by containers, controls, and custom widgets.
///
/// # Usage
///
/// Implementors should define how the element draws itself using the provided [`RenderContext`].
/// All UI primitives (such as [`Container`], [`Text`], [`Clickable`], etc.) implement this trait.
///
/// Implementing this trait forces you to touch internal hyprui code, which may be unstable or subject to change, so if you can build your widget
/// as a component by combining core UI elements, please do so instead of implementing this trait directly, this trait should only be used as a last resort.
/// # Example
///
/// ```rust
/// use hyprui::{Element, RenderContext};
///
/// struct MyWidget;
///
/// impl Element for MyWidget {
///     fn render<'clay: 'render, 'render>(&'render self, ctx: &mut RenderContext<'clay, 'render, '_>) {
///         // Custom rendering logic here
///     }
/// }
/// ```
pub trait Element {
	fn render<'clay: 'render, 'render>(&'render self, ctx: &mut RenderContext<'clay, 'render, '_>);
}

impl Element for Vec<Box<dyn Element>> {
	fn render<'clay: 'render, 'render>(&'render self, ctx: &mut RenderContext<'clay, 'render, '_>) {
		for child in self {
			child.render(ctx);
		}
	}
}
impl Element for Box<dyn Element> {
	fn render<'clay: 'render, 'render>(&'render self, ctx: &mut RenderContext<'clay, 'render, '_>) {
		self.as_ref().render(ctx);
	}
}
