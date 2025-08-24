use hyprui::WindowOptions;

fn main() {
    env_logger::init();
    hyprui::create_window(WindowOptions {
        title: "My first hyprui app".into(),
        ..Default::default()
    });
}
