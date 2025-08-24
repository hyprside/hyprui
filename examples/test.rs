use hyprui::{
    WindowOptions,
    layer_shell::{Anchor, LayerShellOptions},
};

fn main() {
    env_logger::init();
    hyprui::create_window(WindowOptions {
        title: "My first hyprui app".into(),
        preferred_size: (0.0, 60.0),
        enable_layer_shell: Some(LayerShellOptions {
            exclusive_zone: 60,
            anchor: Anchor::TOP | Anchor::RIGHT | Anchor::LEFT,
            keyboard_interactivity: hyprui::layer_shell::KeyboardInteractivity::None,
            ..Default::default()
        }),
        ..Default::default()
    });
}
