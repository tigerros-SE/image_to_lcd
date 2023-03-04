use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

const BIT_SPACING: f32 = 255.0 / 7.0;

fn calculate_error(error: (i16, i16, i16), rate: i16) -> (i16, i16, i16) {
	(((error.0 * rate) as f32 / 16.0).round() as i16, ((error.1 * rate) as f32 / 16.0).round() as i16, ((error.2 * rate) as f32 / 16.0).round() as i16)
}

fn add_error_to_color(color: &Rgba<u8>, error: (i16, i16, i16)) -> Rgba<u8> {
	Rgba::<u8>::from([
		(color.0[0] as i16 + error.0).clamp(0, 255) as u8,
		(color.0[1] as i16 + error.1).clamp(0, 255) as u8,
		(color.0[2] as i16 + error.2).clamp(0, 255) as u8,
		color.0[3]
	])
}

fn convert_to_se_color(color: &Rgba<u8>) -> Rgba<u8> {
	let r = ((color.0[0] as f32 / BIT_SPACING).round() * BIT_SPACING) as u8;
	let g = ((color.0[1] as f32 / BIT_SPACING).round() * BIT_SPACING) as u8;
	let b = ((color.0[2] as f32 / BIT_SPACING).round() * BIT_SPACING) as u8;

	Rgba::from([r, g, b, color.0[3]])
}

pub fn dither_image(img: &mut DynamicImage) {
	let (width, height) = img.dimensions();

	for y in 0..height {
		for x in 0..width {
			let pix = img.get_pixel(x, y);
			let pix_quantized = convert_to_se_color(&pix);
			let error = (
				pix.0[0] as i16 - pix_quantized.0[0] as i16,
				pix.0[1] as i16 - pix_quantized.0[1] as i16,
				pix.0[2] as i16 - pix_quantized.0[2] as i16,
			);

			// (x or y)pN = x or y + N
			// (x or y)nN = x or y - N

			img.put_pixel(x, y, pix_quantized);

			if x + 1 < width {
				let xp1_y = img.get_pixel(x + 1, y);
				let xp1_y_error = calculate_error(error, 7);

				img.put_pixel(x + 1, y, add_error_to_color(&xp1_y, xp1_y_error));
			}

			if (x as i32) - 1 >= 0 && y + 1 < height {
				let xn1_yp1 = img.get_pixel(x - 1, y + 1);
				let xn1_yp1_error = calculate_error(error, 3);

				img.put_pixel(x - 1, y + 1, add_error_to_color(&xn1_yp1, xn1_yp1_error));
			}

			if y + 1 < height {
				let x_yp1 = img.get_pixel(x, y + 1);
				let x_yp1_error = calculate_error(error, 5);

				img.put_pixel(x, y + 1, add_error_to_color(&x_yp1, x_yp1_error));
			}

			if x + 1 < width && (y as i32) - 1 >= 0 {
				let xp1_yn1 = img.get_pixel(x + 1, y - 1);
				let xp1_yn1_error = calculate_error(error, 1);

				img.put_pixel(x + 1, y - 1, add_error_to_color(&xp1_yn1, xp1_yn1_error));
			}
		}
	}
}