use image::{DynamicImage, Rgba, RgbaImage};
use image::imageops::FilterType;

const BIT_SPACING: f32 = 255.0 / 7.0;
/// Serves simply as a temporary "transparent" character.
/// Should be replaced by `TEMPORARY_TEMP_REPLACEMENT`.
/// The reason why we can't use `TEMPORARY_TEMP_REPLACEMENT` right away, is because the `to_se_char`
/// function returns a character, not a string.
pub(crate) const TRANSPARENCY_TEMP: char = ' ';
/// The truly transparent string.
pub(crate) const TRANSPARENCY_TEMP_REPLACEMENT: &str = "\u{E075}\u{E072}\u{E070}";

/// Converts a color into a Space Engineers character that displays that color in a configured LCD panel.
/// # Arguments
///
/// * `color` - The RGBA color to convert.
/// * `preserve_transparency` - Whether to return the `TRANSPARENCY_TEMP` if the `color` has an alpha channel value of 0 (it is fully transparent).
pub fn to_se_char(color: &Rgba<u8>, preserve_transparency: bool) -> char {
	if preserve_transparency && color.0[3] == 0 {
		return TRANSPARENCY_TEMP;
	}

	// I'll let you on in a little secret. I have no idea how this works.
	// The bit shifting I get, to encode all the colors, but the "BIT_SPACING"? No clue.
	// This is the part copied from Whip's Image Converter
	let new_r = ((color.0[0] as f32 / BIT_SPACING).round() as u32) << 6;
	let new_g = ((color.0[1] as f32 / BIT_SPACING).round() as u32) << 3;
	let new_b = (color.0[2] as f32 / BIT_SPACING).round() as u32;

	char::from_u32(0xe100 + new_r + new_g + new_b).unwrap()
}

/// Returns the Greatest Common Divisor (GCD) of two numbers (iterative version).
pub(crate) fn gcd(a: u32, b: u32) -> u32 {
	if a == b {
		return a;
	} else if a == 1 || b == 1 {
		return 1;
	} else if a == 2 && b % 2 == 0 {
		return 2;
	} else if b == 2 && a % 2 == 0 {
		return 2;
	}

	let (mut x, mut y) = if a > b {
		(a, b)
	} else {
		(b, a)
	};

	let mut rem = x % y;

	while rem > 0 {
		x = y;
		y = rem;
		rem = x % y;
	}

	y
}

/// Resizes the image to an exact size, can preserve the original image's aspect ratio.
///
/// # Arguments
///
/// * `img` - The image to resize.
/// * `width` - The width of the resized image.
/// * `height` - The height of the resized image.
/// * `preserve_original_aspect_ratio` - When resizing the image, determines whether to scale the original to fit the new bounds while keeping the same aspect ratio, or to perfectly fit the new size.
pub(crate) fn resized_exact_size(img: &DynamicImage, width: u32, height: u32, preserve_original_aspect_ratio: bool) -> RgbaImage {
	if preserve_original_aspect_ratio {
		img.resize(width, height, FilterType::Gaussian).to_rgba8()
	} else {
		img.resize_exact(width, height, FilterType::Gaussian).to_rgba8()
	}
}