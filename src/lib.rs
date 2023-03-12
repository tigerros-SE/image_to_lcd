mod dither_util;

use image::{DynamicImage, GenericImageView, Rgba};
use image::imageops::FilterType;

const BIT_SPACING: f32 = 255.0 / 7.0;

/// Converts a color into a Space Engineers character that displays that color in a configured LCD panel.
pub fn to_se_char(color: &Rgba<u8>) -> char {
    // I'll let you on in a little secret. I have no idea how this works.
    // The bit shifting I get, to encode all the colors, but the "BIT_SPACING"? No clue.
    // This is the part copied from Whip's Image Converter
    let new_r = ((color.0[0] as f32 / BIT_SPACING).round() as u32) << 6;
    let new_g = ((color.0[1] as f32 / BIT_SPACING).round() as u32) << 3;
    let new_b = (color.0[2] as f32 / BIT_SPACING).round() as u32;

    char::from_u32(0xe100 + new_r + new_g + new_b).unwrap()
}

/// Converts an image to a string composed of characters that can be displayed in a configured LCD panel.
pub fn image_to_se_string(img: &DynamicImage, width_aspect_ratio: u32, height_aspect_ratio: u32) -> String {
    let resized = if width_aspect_ratio == 0 && height_aspect_ratio == 0 {
        (&img).resize(178, 178, FilterType::Nearest)
    } else {
        (&img).resize_exact(width_aspect_ratio * 178, height_aspect_ratio * 178, FilterType::Nearest)
    };

    let (width, height) = resized.dimensions();
    let mut se_string = String::with_capacity((width * height) as usize);

    for y in 0..height {
        for x in 0..width {
            let pix = resized.get_pixel(x, y);
            se_string.push(to_se_char(&pix));
        }

        se_string.push('\n');
    }

    se_string.trim().to_string()
}

/// Converts an image to a string composed of characters that can be displayed in a configured LCD panel.
pub fn image_to_se_string_dithered(img: &mut DynamicImage, width_aspect_ratio: u32, height_aspect_ratio: u32) -> String {
    dither_util::dither_image(img);
    image_to_se_string(&img, width_aspect_ratio, height_aspect_ratio)
}

// The IDE generated some tests for me right here.
// You can guess what I did with them, and it involves the backspace button.