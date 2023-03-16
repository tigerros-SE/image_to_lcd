use std::fs::File;
use std::io::Write;
use std::time::Instant;
use crate::*;

// For testing in-game
// let output_path = trimmed_path.to_owned() + "_space_engineers_text.txt";
// let mut file = File::create(&output_path).unwrap();
//
// file.write_all(output_string.as_ref()).unwrap();
// file.flush().unwrap();

// TODO: Always do the human eye "good enough" test by copying the above code and trying the string in-game!
// This can't be tested with code, because a minor change to the dithering algorithm or
// something similar could result in a different result yet that result could still look good and be valid.

#[test]
fn performance_no_dither_under_60ms() {
	let path = "C:\\Users\\glaci\\OneDrive\\Obrázky\\Riker.png";
	let trimmed_path = path.trim();
	let source = image::open(trimmed_path).expect("Open image");
	let resized = resized(&source, 1, 1, false);

	let now = Instant::now();

	let output_string = image_to_se_string(&resized, false, true);

	let elapsed_millis = now.elapsed().as_millis();

	println!("Elapsed milliseconds: {:?}", elapsed_millis);
	assert!(elapsed_millis < 100);
}

#[test]
fn performance_dither_under_90ms() {
	let path = "C:\\Users\\glaci\\OneDrive\\Obrázky\\Riker.png";
	let trimmed_path = path.trim();
	let source = image::open(trimmed_path).expect("Open image");
	let resized = resized(&source, 1, 1, false);

	let now = Instant::now();

	let output_string = image_to_se_string(&resized, true, true);

	let elapsed_millis = now.elapsed().as_millis();

	println!("Elapsed milliseconds: {:?}", elapsed_millis);
	assert!(elapsed_millis < 100);
}

#[test]
fn size_test() {
	let path = "C:\\Users\\glaci\\OneDrive\\Obrázky\\Riker.png";
	let trimmed_path = path.trim();
	let source = image::open(trimmed_path).expect("Open image");

	{
		let _3_to_2 = resized(&source, 3, 2, false);

		assert_eq!(_3_to_2.width(), 3 * LCD_SQUARE_PANEL_CHARACTER_SIZE);
		assert_eq!(_3_to_2.height(), 2 * LCD_SQUARE_PANEL_CHARACTER_SIZE);
	}

	{
		let _1_to_2 = resized(&source, 1, 2, false);

		assert_eq!(_1_to_2.width(), LCD_SQUARE_PANEL_CHARACTER_SIZE);
		assert_eq!(_1_to_2.height(), 2 * LCD_SQUARE_PANEL_CHARACTER_SIZE);
	}

	{
		let _6_to_3 = resized(&source, 6, 3, false);

		// Simplify 6:3 to 2:1
		assert_eq!(_6_to_3.width(), 2 * LCD_SQUARE_PANEL_CHARACTER_SIZE);
		assert_eq!(_6_to_3.height(), LCD_SQUARE_PANEL_CHARACTER_SIZE);
	}

	{
		let _18_to_48 = resized(&source, 18, 48, false);

		// Simplify 18:48 to 3:8
		assert_eq!(_18_to_48.width(), 3 * LCD_SQUARE_PANEL_CHARACTER_SIZE);
		assert_eq!(_18_to_48.height(), 8 * LCD_SQUARE_PANEL_CHARACTER_SIZE);
	}
}