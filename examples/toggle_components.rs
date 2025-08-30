#![allow(non_snake_case)]
use hyprui::{Clickable, Component, Container, Element, Text, WindowOptions, use_state};

fn ComponenteA() -> Box<dyn Element> {
	let (count, set_count) = use_state(0);
	Box::new(
		Container::new()
			.padding_all(24)
			.background_color((0x20, 0x40, 0x20))
			.rounded(12.0)
			.child(
				Clickable::new(
					"a_btn",
					Text::new(format!("Componente A: {}", count))
						.font_size(18)
						.color((255, 255, 255, 255).into())
						.font_family("UbuntuSans NF"),
				)
				.on_click(move || set_count(count + 1)),
			),
	)
}

fn ComponenteB() -> Box<dyn Element> {
	let (count, set_count) = use_state(100);
	Box::new(
		Container::new()
			.padding_all(24)
			.background_color((0x40, 0x20, 0x20))
			.rounded(12.0)
			.child(
				Clickable::new(
					"b_btn",
					Text::new(format!("Componente B: {}", count))
						.font_size(18)
						.color((255, 255, 255, 255).into())
						.font_family("UbuntuSans NF"),
				)
				.on_click(move || set_count(count + 10)),
			),
	)
}

fn root(_: ()) -> Box<dyn Element> {
	let (show_a, set_show_a) = use_state(true);

	Box::new(
		Container::new()
			.direction(hyprui::Direction::Column)
			.child(
				Container::new()
					.background_color((0x22, 0x22, 0x22))
					.padding_all(16)
					.child(
						Clickable::new(
							"toggle_btn",
							Text::new("Swap Component")
								.font_size(16)
								.color((255, 255, 255, 255).into())
								.font_family("UbuntuSans NF"),
						)
						.on_click(move || set_show_a(!show_a)),
					),
			)
			.child(if show_a {
				Component::from(ComponenteA)
			} else {
				Component::from(ComponenteB)
			}),
	)
}

fn main() {
	env_logger::init();

	hyprui::create_window(
		root,
		(),
		WindowOptions {
			title: "HyprUI Toggle Example".into(),
			preferred_size: (340.0, 220.0),
			..Default::default()
		},
	);
}
