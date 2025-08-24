use std::cell::RefCell;

mod window_options;
mod winit;
pub use window_options::WindowOptions;

use crate::winit::WinitApp;
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

    let winit_app = WinitApp::new(options);
    winit_app.run();
}
