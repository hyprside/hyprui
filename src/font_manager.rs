use super::clay_renderer::create_measure_text_function;
use clay_layout::Clay;
use skia_safe::{FontMgr, FontStyle, Typeface};

pub struct FontManager {
	fonts: Vec<Typeface>,
	updated_fonts: bool,
	font_mgr: FontMgr,
}

impl FontManager {
	pub fn new() -> Self {
		FontManager {
			fonts: Vec::new(),
			updated_fonts: true,
			font_mgr: FontMgr::new(),
		}
	}

	/// Loads a font by family and style, appends it if not already present, and returns its numeric ID (1-based).
	pub fn get(&mut self, family: &str, style: FontStyle) -> u16 {
		// Try to find an existing font
		if let Some((idx, _)) = self
			.fonts
			.iter()
			.enumerate()
			.find(|(_, tf)| tf.family_name() == family && tf.font_style() == style)
		{
			return idx as u16;
		}
		if self.fonts.len() > u16::MAX as usize {
			panic!("Too many fonts loaded");
		}
		// Otherwise, load and append
		let typeface = self
			.font_mgr
			.match_family_style(family, style)
			.unwrap_or_else(|| panic!("Font '{}' with style {:?} not found", family, style));
		self.fonts.push(typeface);
		self.updated_fonts = true;
		self.fonts.len() as u16 - 1
	}

	/// Returns a slice of all loaded fonts.
	pub fn get_fonts(&self) -> &[Typeface] {
		&self.fonts
	}

	/// Creates a clay measure function using the loaded fonts.
	pub fn update_clay_measure_function(&mut self, clay: &mut Clay) {
		if self.updated_fonts {
			let fonts = self.fonts.clone();
			clay.set_measure_text_function(create_measure_text_function(fonts));
			self.updated_fonts = false;
		}
	}
}
