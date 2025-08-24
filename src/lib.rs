use std::{cell::RefCell, collections::HashMap, rc::Rc};
mod clay_renderer;
mod font_manager;
mod window_options;
mod winit;
use clay_layout::{
    Color, Declaration, grow,
    layout::Alignment,
    math::{Dimensions, Vector2},
    text::TextConfig,
};
use skia_safe::Color4f;
pub use window_options::WindowOptions;

use crate::{
    clay_renderer::{clay_skia_render, create_measure_text_function},
    font_manager::FONTS,
    winit::{Callbacks, WinitApp},
};
pub mod layer_shell {
    pub use crate::window_options::{Anchor, KeyboardInteractivity, LayerShellOptions};
}
thread_local! {
    static REQUEST_REDRAW: RefCell<Box<dyn Fn()>> =  RefCell::new(Box::new(|| {}));
}
trait GlobalClosure {
    fn call(&'static self);
}
impl GlobalClosure for std::thread::LocalKey<RefCell<Box<dyn Fn()>>> {
    fn call(&'static self) {
        self.with(|r| r.borrow()())
    }
}
pub fn create_window(options: WindowOptions) {
    color_eyre::install().ok();

    let clay = Rc::new(RefCell::new(clay_layout::Clay::new((0.0, 0.0).into())));
    clay.borrow_mut()
        .set_measure_text_function(create_measure_text_function(&FONTS));
    let mouse_buttons_state = Rc::new(RefCell::new(HashMap::<u16, bool>::new()));
    let mouse_position = Rc::new(RefCell::new((0f32, 0f32)));
    // clay.set_measure_text_function(font_manager.create_clay_measure_function());
    let winit_app = WinitApp::new(
        options,
        Callbacks {
            on_render_callback: {
                let clay = Rc::clone(&clay);
                Box::new(move |canvas| {
                    let mut clay = clay.borrow_mut();
                    {
                        let mut c = clay.begin();
                        c.with(
                            Declaration::new()
                                .layout()
                                .child_alignment(Alignment::new(
                                    clay_layout::layout::LayoutAlignmentX::Center,
                                    clay_layout::layout::LayoutAlignmentY::Center,
                                ))
                                .width(grow!())
                                .height(grow!())
                                .end(),
                            |c| {
                                c.text(
                                    "Hello World!",
                                    TextConfig::new()
                                        .color(Color::rgb(255.0, 255.0, 255.0))
                                        .font_size(30)
                                        .end(),
                                );
                            },
                        );
                        clay_skia_render::<()>(canvas, c.end(), |_, _, _| {}, &FONTS);
                    }
                })
            },
            on_mouse_move: {
                let clay = Rc::clone(&clay);
                let mouse_buttons_state = Rc::clone(&mouse_buttons_state);
                let mouse_position = Rc::clone(&mouse_position);
                Box::new(move |x, y| {
                    *mouse_position.borrow_mut() = (x as _, y as _);
                    let mouse_buttons_state = mouse_buttons_state.borrow();
                    let clay = clay.borrow_mut();
                    clay.pointer_state(
                        Vector2::new(x as _, y as _),
                        mouse_buttons_state.get(&0).copied().unwrap_or(false),
                    );
                })
            },
            on_mouse_button: {
                let clay = Rc::clone(&clay);
                let mouse_buttons_state = Rc::clone(&mouse_buttons_state);
                let mouse_position = Rc::clone(&mouse_position);
                Box::new(move |pressed, button| {
                    let clay = clay.borrow_mut();
                    let mut mouse_buttons_state = mouse_buttons_state.borrow_mut();
                    mouse_buttons_state.insert(button, pressed);
                    if button == 0 {
                        let (x, y) = mouse_position.borrow().to_owned();
                        clay.pointer_state(Vector2::new(x, y), pressed);
                    }
                })
            },
            on_window_resize: {
                let clay = Rc::clone(&clay);
                Box::new(move |width, height| {
                    let clay = clay.borrow_mut();
                    clay.set_layout_dimensions(Dimensions::new(width as _, height as _));
                })
            },
        },
    );
    winit_app.run();
}
