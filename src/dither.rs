use image::{Rgba, RgbaImage};

// See https://en.wikipedia.org/wiki/Floyd%E2%80%93Steinberg_dithering#Implementation
const RIGHT_DIFFUSION_RATE: f32 = 7.0 / 16.0;
const BOTTOM_RIGHT_DIFFUSION_RATE: f32 = 1.0 / 16.0;
const BOTTOM_DIFFUSION_RATE: f32 = 5.0 / 16.0;
const BOTTOM_LEFT_DIFFUSION_RATE: f32 = 3.0 / 16.0;
const BIT_SPACING: f32 = 255.0 / 7.0;

/// Dithers the image using [Floyd-Steinberg dithering](https://en.wikipedia.org/wiki/Floyd%E2%80%93Steinberg_dithering).
///
/// # Arguments
///
/// * `img` - The image to dither.
pub(crate) fn dither_image(img: &RgbaImage) -> RgbaImage {
	let mut dithered = img.to_owned();
	let (width, height) = dithered.dimensions();

	for y in 0..height {
		for x in 0..width {
			let pix = dithered.get_pixel(x, y);
			let pix_quantized = convert_to_se_color(&pix);
			let error = quantization_error((pix.0[0], pix.0[1], pix.0[2]), (pix_quantized.0[0], pix_quantized.0[1], pix_quantized.0[2]));

			dithered.put_pixel(x, y, pix_quantized);

			if x + 1 < width {
				let right_pix = dithered.get_pixel(x + 1, y);
				let right_error = mul_error(error, RIGHT_DIFFUSION_RATE);

				dithered.put_pixel(x + 1, y, add_error_to_color(&right_pix, right_error));
			}

			if (x as i32) - 1 >= 0 && y + 1 < height {
				let bottom_left_pix = dithered.get_pixel(x - 1, y + 1);
				let bottom_left_error = mul_error(error, BOTTOM_LEFT_DIFFUSION_RATE);

				dithered.put_pixel(x - 1, y + 1, add_error_to_color(&bottom_left_pix, bottom_left_error));
			}

			if y + 1 < height {
				let bottom_pix = dithered.get_pixel(x, y + 1);
				let bottom_error = mul_error(error, BOTTOM_DIFFUSION_RATE);

				dithered.put_pixel(x, y + 1, add_error_to_color(&bottom_pix, bottom_error));
			}

			if x + 1 < width && y + 1 < height {
				let bottom_right_pix = dithered.get_pixel(x + 1, y + 1);
				let bottom_right_error = mul_error(error, BOTTOM_RIGHT_DIFFUSION_RATE);

				dithered.put_pixel(x + 1, y + 1, add_error_to_color(&bottom_right_pix, bottom_right_error));
			}
		}
	}

	dithered
}

fn mul_error(error: (i16, i16, i16), rate: f32) -> (i16, i16, i16) {
	((error.0 as f32 * rate).round() as i16,
	 (error.1 as f32 * rate).round() as i16,
	 (error.2 as f32 * rate).round() as i16)
}

fn quantization_error(non_quantized: (u8, u8, u8), quantized: (u8, u8, u8)) -> (i16, i16, i16) {
	(
		non_quantized.0 as i16 - quantized.0 as i16,
		non_quantized.1 as i16 - quantized.1 as i16,
		non_quantized.2 as i16 - quantized.2 as i16,
	)
}

fn add_error_to_color(color: &Rgba<u8>, error: (i16, i16, i16)) -> Rgba<u8> {
	Rgba::from([
		(color.0[0] as i16 + error.0).clamp(0, 255) as u8,
		(color.0[1] as i16 + error.1).clamp(0, 255) as u8,
		(color.0[2] as i16 + error.2).clamp(0, 255) as u8,
		color.0[3],
	])
}

fn convert_to_se_color(color: &Rgba<u8>) -> Rgba<u8> {
	let r = ((color.0[0] as f32 / BIT_SPACING).round() * BIT_SPACING) as u8;
	let g = ((color.0[1] as f32 / BIT_SPACING).round() * BIT_SPACING) as u8;
	let b = ((color.0[2] as f32 / BIT_SPACING).round() * BIT_SPACING) as u8;

	Rgba::from([r, g, b, color.0[3]])
}