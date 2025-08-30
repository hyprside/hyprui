use winit::dpi::LogicalSize;
use winit::icon::RgbaIcon;
use winit::monitor::Fullscreen;
pub use winit::platform::wayland::Anchor;
pub use winit::platform::wayland::KeyboardInteractivity;
use winit::platform::wayland::WindowAttributesWayland;
use winit::window::WindowAttributes;

#[derive(Clone)]
pub struct LayerShellOptions {
	pub anchor: Anchor,
	pub exclusive_zone: i32,
	pub margin: (i32, i32, i32, i32),
	pub keyboard_interactivity: KeyboardInteractivity,
	pub output: Option<u64>,
}
impl Default for LayerShellOptions {
	fn default() -> Self {
		Self {
			anchor: Anchor::empty(),
			exclusive_zone: 0,
			margin: (0, 0, 0, 0),
			keyboard_interactivity: KeyboardInteractivity::None,
			output: None,
		}
	}
}
#[derive(Default, Clone)]
pub struct WindowOptions<'a> {
	pub title: String,
	pub min_size: (f64, f64),
	pub preferred_size: (f64, f64),
	pub max_size: (f64, f64),
	pub enable_layer_shell: Option<LayerShellOptions>,
	pub opaque: bool,
	pub allow_backdrop_blur: bool,
	pub wayland_name: Option<&'a str>,
	pub no_border: bool,
	pub fullscreen: bool,
	pub icon: Option<RgbaIcon>,
}
impl From<WindowOptions<'_>> for WindowAttributes {
	fn from(options: WindowOptions) -> Self {
		let mut winit_opt = WindowAttributes::default()
			.with_blur(options.allow_backdrop_blur)
			.with_transparent(!options.opaque)
			.with_decorations(!options.no_border)
			.with_fullscreen(if options.fullscreen {
				Some(Fullscreen::Borderless(None))
			} else {
				None
			})
			.with_title(if options.title.is_empty() {
				"<Untitled>".to_string()
			} else {
				options.title
			})
			.with_window_icon(options.icon.map(|i| i.into()));
		if options.min_size != (0., 0.) {
			winit_opt =
				winit_opt.with_min_surface_size(LogicalSize::new(options.min_size.0, options.min_size.1));
		}
		if options.preferred_size != (0., 0.) {
			winit_opt = winit_opt.with_surface_size(LogicalSize::new(
				options.preferred_size.0,
				options.preferred_size.1,
			))
		}
		if options.max_size != (0., 0.) {
			winit_opt =
				winit_opt.with_max_surface_size(LogicalSize::new(options.max_size.0, options.max_size.1))
		}

		let mut wayland_opts = WindowAttributesWayland::default();
		let mut has_wl_opts = false;
		if let Some(l) = options.enable_layer_shell {
			wayland_opts = wayland_opts
				.with_layer_shell()
				.with_margin(l.margin.0, l.margin.1, l.margin.2, l.margin.3)
				.with_anchor(l.anchor)
				.with_exclusive_zone(l.exclusive_zone);
			if let Some(output) = l.output {
				wayland_opts = wayland_opts.with_output(output);
			}
			has_wl_opts = true;
		}
		if let Some(wayland_name) = options.wayland_name {
			wayland_opts = wayland_opts.with_name(wayland_name, "");
			has_wl_opts = true;
		}
		if has_wl_opts {
			winit_opt = winit_opt.with_platform_attributes(Box::new(wayland_opts));
		}
		winit_opt
	}
}
