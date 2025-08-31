#![allow(non_snake_case)]

use hyprui::*;
fn Button(_: ()) -> Box<dyn Element> {
	let (count, set_count) = use_state(0);
	// let clickable_ref = use_ref(None);
	// clickable_ref.unwrap().
	//                        ------------------------------
	//                        | is_hovered: boolean        |
	//                        | is_down: boolean           |
	//                        | is_focused: boolean        |
	//                        | is_disabled: boolean       |
	//                        -----------------------------
	rsml! {
		<container
			padding_all={8}
			rounded={4.}
			w_fit
			border_width={1}
			border_color={(0xff, 0xff, 0xff, 0x20)}
			on_click={move || set_count(count + 1)}
			style_if_hovered={|s| ContainerStyle {background_color: (0xff, 0xff, 0xff, 0x20).into(), ..s}}
			style_if_pressed={|s| ContainerStyle {background_color: (0xff, 0xff, 0xff, 0x40).into(), ..s}}
		>
			<text color={(255, 255, 255, 255)} font_family="UbuntuSans NF">
				{format!("Count: {}", count)}
			</text>
		</container>
	}
}

fn Root(_: ()) -> Box<dyn Element> {
	rsml! {
		<container h_expand w_expand background_color={(0x0b, 0x0b, 0x0b)} center gap={3*4} direction={Direction::Row}>
			<Button/>
			<Button/>
		</container>
	}
}

fn main() {
	hyprui::create_window(
		Root,
		(),
		WindowOptions {
			title: "Focus Example".into(),
			..Default::default()
		},
	)
}
