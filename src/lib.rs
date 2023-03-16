//! # Overview
//!
//! A crate that allows you to convert an image to a string that can be displayed in a [Space Engineers](https://www.spaceengineersgame.com/) LCD panel.
//!
//! Check out [the crates.io Readme](https://crates.io/crates/image_to_space_engineers_lcd) for a complete tutorial.

mod tests;
mod dither;
pub mod util;

use image::{DynamicImage, RgbaImage};

/// How many pixels are on one line of a square panel.
/// The character number is actually `178 * 3 = 534`, because each color character is counted as three because it has the three RGB values.
/// It's kind of confusing, yes.
pub const SQUARE_PANEL_WIDTH_PIXEL_AMOUNT: u32 = 178;
/// How many lines are in one square panel.
/// Keep in mind there's only `177` newlines (`\n`), because the last line doesn't have a `\n`.
pub const SQUARE_PANEL_LINE_AMOUNT: u32 = 178;
/// How many characters are on one line of a square panel.
/// This is equal to `SQUARE_PANEL_WIDTH_PIXEL_AMOUNT * 3`, because encoded color characters are counted as three characters.
///
/// **This is not reliable. Fully transparent pixels are replaced with a space which is only one character, whereas characters which have three RGB values encoded in them are counted as three characters.**
pub const SQUARE_PANEL_WIDTH_CHARACTER_AMOUNT: u32 = SQUARE_PANEL_WIDTH_PIXEL_AMOUNT * 3;

/// Resizes the image so that it can be converted.
///
/// **You need to use this function before using [`image_to_se_string`](https://docs.rs/image_to_space_engineers_lcd/latest/image_to_space_engineers_lcd/fn.image_to_se_string.html).**
///
/// # Arguments
///
/// * `img` - The image to resize.
/// * `width_aspect_ratio` - The width component of the aspect ratio of the resized image.
/// * `height_aspect_ratio` - The height component of the aspect ratio of the resized image.
/// * `preserve_original_aspect_ratio` - When resizing the image, determines whether to scale the original to fit the new bounds while keeping the same aspect ratio, or to perfectly fit the new size.
///
/// # Examples
///
/// ```
/// // Using the image <https://crates.io/crates/image/0.24.5)> crate.
/// let path = "path/to/image";
/// let img = image::open(path).unwrap();
/// // For a Wide LCD panel, where we don't want to stretch the image
/// let resized = image_to_space_engineers_lcd::resized(&img, 2, 1, true);
/// ```
pub fn resized(img: &DynamicImage, width_aspect_ratio: u32, height_aspect_ratio: u32, preserve_original_aspect_ratio: bool) -> RgbaImage {
	if width_aspect_ratio == height_aspect_ratio {
		return util::resized_exact_size(img,
		                                SQUARE_PANEL_WIDTH_PIXEL_AMOUNT,
		                                SQUARE_PANEL_WIDTH_PIXEL_AMOUNT,
		                                preserve_original_aspect_ratio);
	}

	let gcd = util::gcd(width_aspect_ratio, height_aspect_ratio);

	if gcd == 1 {
		return util::resized_exact_size(img,
		                                width_aspect_ratio * SQUARE_PANEL_WIDTH_PIXEL_AMOUNT,
		                                height_aspect_ratio * SQUARE_PANEL_WIDTH_PIXEL_AMOUNT,
		                                preserve_original_aspect_ratio);
	}

	let simplified_width_aspect_ratio = width_aspect_ratio / gcd;
	let simplified_height_aspect_ratio = height_aspect_ratio / gcd;

	util::resized_exact_size(img,
	                         simplified_width_aspect_ratio * SQUARE_PANEL_WIDTH_PIXEL_AMOUNT,
	                         simplified_height_aspect_ratio * SQUARE_PANEL_WIDTH_PIXEL_AMOUNT,
	                         preserve_original_aspect_ratio)
}

/// Converts an image to a string composed of characters that can be displayed in a configured LCD panel.
/// The image will be resized in order to fit an LCD panel.
///
/// **You first need to use the [`resized`](https://docs.rs/image_to_space_engineers_lcd/latest/image_to_space_engineers_lcd/fn.resized.html) function and pass the result as the `img` argument.**
///
/// # Arguments
///
/// * `img` - The image to convert.
/// * `dither` - Whether to dither the image. This results in much better results, so you most likely want it `true`.
/// * `preserve_transparency` - Whether to preserve transparency, however, only pixels with full transparency (an alpha value of 255) will be preserved.
///
/// # Examples
///
/// Since a proper example would be rather lengthy, simply check out <https://github.com/tigerros-SE/image_to_lcd#code-example>.
/// And if you haven't already, look at the [usage](https://github.com/tigerros-SE/image_to_lcd#usage).
pub fn image_to_se_string(img: &RgbaImage, dither: bool, preserve_transparency: bool) -> String {
	let img2 = if dither {
		dither::dither_image(&img)
	} else {
		img.clone()
	};

	let (width, height) = img2.dimensions();
	let mut se_string = String::with_capacity((width * height) as usize);

	for y in 0..height {
		for x in 0..width {
			let pix = img2.get_pixel(x, y);

			se_string.push(util::to_se_char(pix, preserve_transparency));
		}

		if y < height - 1 {
			se_string.push('\n');
		}
	}

	se_string
}