use std::fs::File;
use std::io::Write;
use std::time::Instant;
use crate::*;

// TODO: Occasionally do the "human eye" test (see function `human_eye_test`)!
// Looks can't be tested with code, because a minor change to the dithering algorithm or
// something similar could result in a different result yet that result could still look good and be valid.
// You don't have to do this everytime, because it creates files.

const TEST_IMAGE_PATH: &str = "test_resources\\test_with_holes.png";

fn get_image() -> DynamicImage {
	image::open(TEST_IMAGE_PATH).unwrap()
}

fn create_file_with_output_string(output_string: String, output_path: &String) {
	let mut file = File::create(&output_path).unwrap();

	file.write_all(output_string.as_ref()).unwrap();
	file.flush().unwrap();
}

/// Creates a file with the dithered output string.
/// Copy the contents of that file (the path should be `TEST_IMAGE_PATH + "_SELCD.txt"` and
/// paste them into a square LCD panel and check how they look.
#[test]
fn human_eye_test() {
	let source = get_image();
	let resized = resized(&source, 1, 1, true);
	let output_string = image_to_se_string(&resized, true, true);
	let output_path = TEST_IMAGE_PATH.to_owned() + "_SELCD.txt";

	create_file_with_output_string(output_string, &output_path);

	assert_eq!((format!("Paste this file: {} into a panel in-game.", output_path)), "");
}

/// Creates a file with the output string (not dithered).
/// Copy the contents of that file (the path should be `TEST_IMAGE_PATH + "_SELCD_NO_DITHER.txt"` and
/// paste them into a square LCD panel and check how they look.
#[test]
fn human_eye_test_no_dither() {
	let source = get_image();
	let resized = resized(&source, 1, 1, true);
	let output_string = image_to_se_string(&resized, false, true);
	let output_path = TEST_IMAGE_PATH.to_owned() + "_SELCD_NO_DITHER.txt";

	create_file_with_output_string(output_string, &output_path);

	assert_eq!((format!("Paste this file: {} into a panel in-game.", output_path)), "");
}

#[test]
fn correct_amount_of_pixels_in_resized_image() {
	let source = get_image();
	let resized = resized(&source, 1, 1, false);

	assert_eq!(resized.width() * resized.height(), SQUARE_PANEL_WIDTH_PIXEL_AMOUNT * SQUARE_PANEL_WIDTH_PIXEL_AMOUNT);
}

#[test]
fn performance_no_dither_under_60ms() {
	let source = get_image();
	let resized = resized(&source, 1, 1, false);

	let now = Instant::now();

	let output_string = image_to_se_string(&resized, false, true);

	let elapsed_millis = now.elapsed().as_millis();

	assert!(elapsed_millis < 60);
}

#[test]
fn performance_dither_under_80ms() {
	let source = get_image();
	let resized = resized(&source, 1, 1, false);

	let now = Instant::now();

	let output_string = image_to_se_string(&resized, true, true);

	let elapsed_millis = now.elapsed().as_millis();

	assert!(elapsed_millis < 80);
}

#[test]
fn aspect_ratios_test() {
	let source = get_image();

	{
		let _3_to_2 = resized(&source, 3, 2, false);

		assert_eq!(_3_to_2.width(), 3 * SQUARE_PANEL_WIDTH_PIXEL_AMOUNT);
		assert_eq!(_3_to_2.height(), 2 * SQUARE_PANEL_WIDTH_PIXEL_AMOUNT);
	}

	{
		let _1_to_2 = resized(&source, 1, 2, false);

		assert_eq!(_1_to_2.width(), SQUARE_PANEL_WIDTH_PIXEL_AMOUNT);
		assert_eq!(_1_to_2.height(), 2 * SQUARE_PANEL_WIDTH_PIXEL_AMOUNT);
	}

	{
		let _6_to_3 = resized(&source, 6, 3, false);

		// Simplify 6:3 to 2:1
		assert_eq!(_6_to_3.width(), 2 * SQUARE_PANEL_WIDTH_PIXEL_AMOUNT);
		assert_eq!(_6_to_3.height(), SQUARE_PANEL_WIDTH_PIXEL_AMOUNT);
	}

	{
		let _18_to_48 = resized(&source, 18, 48, false);

		// Simplify 18:48 to 3:8
		assert_eq!(_18_to_48.width(), 3 * SQUARE_PANEL_WIDTH_PIXEL_AMOUNT);
		assert_eq!(_18_to_48.height(), 8 * SQUARE_PANEL_WIDTH_PIXEL_AMOUNT);
	}
}