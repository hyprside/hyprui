#![allow(non_snake_case)]
use hyprui::{Clickable,  Container, Element, Text, WindowOptions, use_state};

fn root(_: ()) -> Box<dyn Element> {
	let (counter, set_counter) = use_state(0);

	Box::new(
		Container::new()
			.direction(hyprui::Direction::Column)
			.h_expand()
			.background_color((0x0b, 0x0b, 0x0b))
			.center()
			.child(
				Container::new()
					.background_color((0x22, 0x22, 0x22))
					.padding_all(16)
					.rounded(10.)
					.w_fit()
					.child(
						Clickable::new(
							"toggle_btn",
							Text::new(format!("Counter {}", counter))
								.font_size(16)
								.color((255, 255, 255, 255).into())
								.font_family("UbuntuSans NF"),
						)
						.on_click(move || set_counter(counter + 1)),
					),
			),
	)
}

fn main() {
	env_logger::init();

	hyprui::create_window(
		root,
		(),
		WindowOptions {
			title: "HyprUI Counter Example".into(),
			preferred_size: (340.0, 220.0),
			..Default::default()
		},
	);
}
