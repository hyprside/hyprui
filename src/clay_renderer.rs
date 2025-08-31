use clay_layout::math::{BoundingBox, Dimensions};
use clay_layout::render_commands::{Border, Custom, RenderCommand, RenderCommandConfig};
use clay_layout::text::TextConfig;
use clay_layout::{ClayLayoutScope, Color as ClayColor};
use skia_safe::{
	Canvas, ClipOp, Color, Color4f, Font, Image, Paint, PaintCap, Path, Point, RRect, Rect,
	SamplingOptions, Typeface,
};

pub fn clay_to_skia_color(color: ClayColor) -> Color4f {
	Color4f::new(
		color.r / 255.,
		color.g / 255.,
		color.b / 255.,
		color.a / 255.,
	)
}

fn clay_to_skia_rect(rect: BoundingBox) -> Rect {
	Rect::from_xywh(rect.x, rect.y, rect.width, rect.height)
}
/// This is a direct* port of Clay's raylib renderer using skia_safe as the drawing API.
pub fn clay_skia_render<'a, CustomElementData: 'a>(
	canvas: &Canvas,
	render_commands: impl Iterator<Item = RenderCommand<'a, Image, CustomElementData>>,
	mut render_custom_element: impl FnMut(
		&RenderCommand<'a, Image, CustomElementData>,
		&Custom<'a, CustomElementData>,
		&Canvas,
	),
	fonts: &[Typeface],
) {
	for command in render_commands {
		match command.config {
			RenderCommandConfig::Text(text) => {
				let text_data = text.text;
				let mut paint = Paint::default();
				paint.set_color4f(clay_to_skia_color(text.color), None);
				let font = Font::new(fonts[text.font_id as usize].clone(), text.font_size as f32);
				let pos = Point::new(
					command.bounding_box.x,
					command.bounding_box.y + text.font_size as f32,
				);
				canvas.draw_str(&text_data, pos, &font, &paint);
			}

			RenderCommandConfig::Image(image) => {
				let skia_image = image.data;
				let mut paint = Paint::default();
				paint.set_color(Color::WHITE);
				paint.set_anti_alias(true);

				let bounds = clay_to_skia_rect(command.bounding_box);
				let has_border_radius = image.corner_radii.top_left > 0.
					|| image.corner_radii.top_right > 0.
					|| image.corner_radii.bottom_left > 0.
					|| image.corner_radii.bottom_right > 0.;
				if has_border_radius {
					canvas.save();
					let rrect = RRect::new_rect_radii(
						bounds,
						&[
							Point::new(image.corner_radii.top_left, image.corner_radii.top_left),
							Point::new(image.corner_radii.top_right, image.corner_radii.top_right),
							Point::new(
								image.corner_radii.bottom_left,
								image.corner_radii.bottom_left,
							),
							Point::new(
								image.corner_radii.bottom_right,
								image.corner_radii.bottom_right,
							),
						],
					);
					canvas.clip_rrect(rrect, ClipOp::Intersect, true);
				}

				canvas.draw_image_rect_with_sampling_options(
					skia_image,
					None,
					bounds,
					SamplingOptions::new(skia_safe::FilterMode::Linear, skia_safe::MipmapMode::Linear),
					&paint,
				);

				// Restore canvas state if we applied a clip
				if has_border_radius {
					canvas.restore();
				}
			}

			RenderCommandConfig::ScissorStart() => {
				// Save the current state then clip to the bounding box.
				canvas.save();
				let clip_rect = clay_to_skia_rect(command.bounding_box);
				canvas.clip_rect(clip_rect, ClipOp::Intersect, true);
			}

			RenderCommandConfig::ScissorEnd() => {
				// Restore the previous state
				canvas.restore();
			}

			RenderCommandConfig::Rectangle(rect) => {
				let paint = {
					let mut p = Paint::default();
					p.set_color4f(clay_to_skia_color(rect.color), None);
					p.set_anti_alias(true);
					p.set_style(skia_safe::PaintStyle::Fill);
					p
				};
				let bounds = clay_to_skia_rect(command.bounding_box);
				if rect.corner_radii.top_left > 0.
					|| rect.corner_radii.top_right > 0.
					|| rect.corner_radii.bottom_left > 0.
					|| rect.corner_radii.bottom_right > 0.
				{
					let rrect = RRect::new_rect_radii(
						bounds,
						&[
							Point::new(rect.corner_radii.top_left, rect.corner_radii.top_left),
							Point::new(rect.corner_radii.top_right, rect.corner_radii.top_right),
							Point::new(rect.corner_radii.bottom_left, rect.corner_radii.bottom_left),
							Point::new(
								rect.corner_radii.bottom_right,
								rect.corner_radii.bottom_right,
							),
						],
					);
					canvas.draw_rrect(rrect, &paint);
				} else {
					canvas.draw_rect(bounds, &paint);
				}
			}

			RenderCommandConfig::Border(border) => {
				// Helper to draw a single side of a rounded border using rrect stroke and a clip path.
				fn draw_side_border_rrect(
					canvas: &Canvas,
					bounds: Rect,
					rrect: &RRect,
					center: Point,
					side: usize, // 0: left, 1: top, 2: right, 3: bottom
					stroke_width: f32,
					color: Color4f,
					border: &Border,
				) {
					let mut path = Path::new();
					match side {
						0 => {
							// Left
							path.move_to(center);
							path.line_to(Point::new(bounds.left, bounds.top));
							path.line_to(Point::new(bounds.left, bounds.bottom));
							path.close();
						}
						1 => {
							// Top
							path.move_to(center);
							path.line_to(Point::new(bounds.left, bounds.top));
							path.line_to(Point::new(bounds.right, bounds.top));
							path.close();
						}
						2 => {
							// Right
							path.move_to(center);
							path.line_to(Point::new(bounds.right, bounds.top));
							path.line_to(Point::new(bounds.right, bounds.bottom));
							path.close();
						}
						3 => {
							// Bottom
							path.move_to(center);
							path.line_to(Point::new(bounds.left, bounds.bottom));
							path.line_to(Point::new(bounds.right, bounds.bottom));
							path.close();
						}
						_ => {}
					}
					canvas.save();
					canvas.clip_path(&path, ClipOp::Intersect, false);

					let mut paint = Paint::default();
					paint.set_color4f(color, None);
					paint.set_anti_alias(true);
					paint.set_style(skia_safe::PaintStyle::Stroke);
					paint.set_stroke_width(stroke_width);
					let rrect = RRect::new_rect_radii(
						Rect::from_ltrb(
							rrect.rect().left + (border.width.left as f32 / 2.0),
							rrect.rect().top + (border.width.top as f32 / 2.0),
							rrect.rect().right - (border.width.right as f32 / 2.0),
							rrect.rect().bottom - (border.width.bottom as f32 / 2.0),
						),
						rrect.radii_ref(),
					);
					canvas.draw_rrect(rrect, &paint);

					canvas.restore();
				}

				let bb = &command.bounding_box;
				let bounds = clay_to_skia_rect(*bb);

				let rrect = RRect::new_rect_radii(
					bounds,
					&[
						Point::new(border.corner_radii.top_left, border.corner_radii.top_left),
						Point::new(border.corner_radii.top_right, border.corner_radii.top_right),
						Point::new(
							border.corner_radii.bottom_right,
							border.corner_radii.bottom_right,
						),
						Point::new(
							border.corner_radii.bottom_left,
							border.corner_radii.bottom_left,
						),
					],
				);

				let center = Point::new(
					bounds.left + bounds.width() / 2.0,
					bounds.top + bounds.height() / 2.0,
				);

				// Draw each border side with its own width and color.
				let border_colors = [
					clay_to_skia_color(border.color), // left
					clay_to_skia_color(border.color), // top
					clay_to_skia_color(border.color), // right
					clay_to_skia_color(border.color), // bottom
				];
				let border_widths = [
					border.width.left as f32,
					border.width.top as f32,
					border.width.right as f32,
					border.width.bottom as f32,
				];

				for side in 0..4 {
					if border_widths[side] > 0.0 {
						draw_side_border_rrect(
							canvas,
							bounds,
							&rrect,
							center,
							side,
							border_widths[side],
							border_colors[side],
							&border,
						);
					}
				}
			}
			RenderCommandConfig::Custom(ref custom) => render_custom_element(&command, custom, canvas),
			RenderCommandConfig::None() => {}
		}
	}
}

pub type SkiaClayScope<'clay, 'render, CustomElements> =
	ClayLayoutScope<'clay, 'render, Image, CustomElements>;

pub fn get_source_dimensions_from_skia_image(image: &Image) -> Dimensions {
	(image.width() as f32, image.height() as f32).into()
}

pub fn create_measure_text_function(
	fonts: Vec<Typeface>,
) -> impl Fn(&str, &TextConfig) -> Dimensions {
	move |text, text_config| {
		let font = Font::new(
			&fonts[text_config.font_id as usize],
			text_config.font_size as f32,
		);
		let width = font.measure_str(text, None).0;
		(width, font.metrics().1.bottom - font.metrics().1.top).into()
	}
}
