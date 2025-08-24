use std::sync::LazyLock;

use skia_safe::{FontMgr, FontStyle, Typeface};

pub static UBUNTU_FONT: LazyLock<Typeface> = LazyLock::new(|| {
    FontMgr::new()
        .match_family_style("UbuntuSans NF", FontStyle::normal())
        .unwrap()
});

pub static FONTS: LazyLock<Vec<&Typeface>> = LazyLock::new(|| vec![&UBUNTU_FONT]);

// TODO: Make the font manager
// The font manager will allow loading a font, appending it to the global list of loaded fonts
// and return a numeric ID as a handle to the font to be used in clay.
//
// let font_manager = FontManager::new();
// font_manager.get("UbuntuSans NF", FontStyle::normal()); // 1
// font_manager.get("UbuntuSans NF", FontStyle::bold());   // 2
// font_manager.get("Comic Sans", FontStyle::normal());    // 3
// font_manager.get_fonts(); // &[Typeface]
// font_manager.create_clay_measure_function(); // impl Fn(&str, &TextConfig) -> Dimensions + 'static
