use hyprui::{rsml, Element, WindowOptions};

fn simple_component(_: ()) -> Box<dyn Element> {
    rsml! {
        <container padding_all={20} center>
            <text font_size={16}>Hello from RSML!</text>
        </container>
    }
}

fn main() {
    env_logger::init();

    hyprui::create_window(
        simple_component,
        (),
        WindowOptions {
            title: "Minimal RSML Test".into(),
            preferred_size: (300.0, 200.0),
            ..Default::default()
        },
    );
}
