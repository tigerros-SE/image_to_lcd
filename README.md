# Table of contents

- [Description](#table-of-contents)
- [Preface](#preface)
- [Example results](#example-results)
- [Usage](#usage)
- [Further configurations](#further-configurations)
  - [Removing the default padding](#removing-the-default-padding)
  - [Changing the text alignment](#changing-the-text-alignment)
  - [Changing the color tone](#changing-the-color-tone)
  - [Final comparison](#final-comparison)
- [Code example](#code-example)
- [How it works](#how-it-works)

## Description

A crate that allows you to convert an image to a string that can be displayed in a [Space Engineers](https://www.spaceengineersgame.com/) LCD panel.

This project uses [Semantic Versioning](https://semver.org/) (tries to).

## Preface

###### You can skip this part.

I would like to mention [Whip's Image Converter](https://github.com/Whiplash141/Whips-Image-Converter) by [Whiplash141](https://github.com/Whiplash141)
for the inspiration and the method of converting a color to a Space Engineers LCD panel character.
I don't know where he found how to do that. I made this for 5 reasons:

1. I wanted to learn Rust.
2. His program is slower.
3. His program is (IMO) a little overcomplicated.
4. His program only runs on Windows.
5. This is a reusable crate version, whereas he only has an executable.

I don't mean those in a bad way, but it's the truth.
Except for #3, that's subjective. But I mean, who needs to choose a dithering method?
However, I applaud him for making a special case for *every panel variation in the game* when it comes to resizing the image.
I took the easy way out with aspect ratios.

## Example results

Original image (image of actor Jonathan Frakes portraying the fictional character "William Thomas Riker" from the franchise Star Trek: The Next Generation):

<img src="https://raw.githubusercontent.com/tigerros-SE/image_to_lcd/master/img/original.png" width="300" alt='Original image (image of actor Jonathan Frakes portraying the fictional character "William Thomas Riker" from the franchise Star Trek: The Next Generation)'/>

Undithered version as seen in Space Engineers:

<img src="https://raw.githubusercontent.com/tigerros-SE/image_to_lcd/master/img/undithered.jpg" width="300" alt="Undithered version in Space Engineers"/>

Dithered version as seen in Space Engineers:

<img src="https://raw.githubusercontent.com/tigerros-SE/image_to_lcd/master/img/dithered.jpg" width="300" alt="Dithered version in Space Engineers"/>

In these screenshots, the undithered version looks darker than the dithered version, but that is only because of the in-game lighting.
I took the screenshots on different panels at different times in the game's day/night cycle.
You can see that the surrounding yellow blocks are also darker in the undithered screenshot.

## Usage

1. Obtain a [`DynamicImage`](https://docs.rs/image/latest/image/enum.DynamicImage.html)
using the [image](https://crates.io/crates/image) crate.
2. Resize the image using [`resized`](https://docs.rs/image_to_space_engineers_lcd/latest/image_to_space_engineers_lcd/fn.resized.html)
function. The aspect ratio is important! It determines the dimensions of the image in-game.
<br/><br/>For example, for a Wide LCD panel, you would need a `2:1` aspect ratio. That is, a `width_aspect_ratio` of 2 and a `height_aspect_ratio` of 1.
If you want to preserve the aspect ratio of the original image (don't stretch it), set the `preserve_aspect_ratio` argument to `true`.
3. Call the [`image_to_se_string`](https://docs.rs/image_to_space_engineers_lcd/latest/image_to_space_engineers_lcd/fn.image_to_se_string.html)
function with the resized image. Setting the `dither` argument to `true` is highly recommended, as your image will look badly otherwise.
4. Now that you have the string, you need to configure your panel. Set the font size to the lowest possible value (0.1)
and the font to `Monospace`.

## Further configurations

### Removing the default padding

By default, Space Engineers has a 2% text padding for all panels.
This means whatever text is on the panel will be moved down/up (depends on the rotation) and
to the right/left (depends on the alignment). For text, this can be nice, but for images, you probably want to remove this,
since it ends up covering some edge pixels because it moves them off the screen.

#### Comparison (top panel has default padding, bottom panel doesn't):

<img src="https://raw.githubusercontent.com/tigerros-SE/image_to_lcd/master/img/center_aligned_default_padding_comparison.png" width="600" alt="Comparison between configured panel (top) and panel with default 2% padding (bottom)"/>

### Changing the text alignment

Imagine you have a portrait (greater height over width) that you want to display, but you don't want to stretch it to fit the panel.
In that case, if you were to display it on a square panel, there would be a big black gap on the right (since the default is left aligned and your image is thinner than the panel).
You can change the alignment to center the image, therefore having an equivalent gap on each side of the panel.

#### Comparison (top panel is aligned to the left, bottom panel isn't):

<img src="https://raw.githubusercontent.com/tigerros-SE/image_to_lcd/master/img/left_aligned_no_padding_comparison.png" width="600" alt="Comparison between configured panel (top) and panel with default left alignment"/>

### Changing the color tone

Normally, when you want text that is of a different color than white, you just change the color of the panel.
This works even with an image, even if the image "text" already has a color. Changing the color affects the tone of the image.

Our image looks a little too bright in-game. Using the color sliders, we can lower the value of each RGB component.
In this case, I lowered them to all be at 100 from the default 255 (effectively lowering the brightness), and I think it looks much closer to the original.

#### Comparison (top panel doesn't have any color modifications, bottom panel color is set to 100 throughout all RGB components):

<img src="https://raw.githubusercontent.com/tigerros-SE/image_to_lcd/master/img/lower_brightness_comparison.png" width="600" alt="Comparison between panel without color modifications (top) and panel with all RGB components set to 100 (bottom)"/>

You can obviously change each individual RGB component. For example, increasing the red component can often make images (specifically of people or landscapes)
look more "lively". Let's combine this with the previous method. This time, the red component is set to 220, but green and blue to 180.
We lowered all of them, but made red stand out.

#### Comparison (top panel doesn't have any color modifications, bottom panel has a red component value of 220, but a green and blue value of 180):

<img src="https://raw.githubusercontent.com/tigerros-SE/image_to_lcd/master/img/increased_red_comparison.png" width="600" alt="Comparison between panel without color modifications (top) and panel with a red component value of 220, but a green and blue value of 180 (bottom)"/>

### Final comparison

Following the simple steps above, you can get your panel to resemble the original image much more.
Take a look at this comparison (top image is the original image, bottom left is the default panel configuration, bottom right is the tweaked panel):

<img src="https://raw.githubusercontent.com/tigerros-SE/image_to_lcd/master/img/final_comparison.png" width="600" alt="Comparison between original image (top), default panel without any configurations (bottom left) and tweaked panel (bottom right)"/>

Notice the absence of the gap on the left and top of the panel, and the closer color resemblance to the original image.

## Code example

This is a simple example binary program that does nothing more but ask the user all the necessary information to convert an image
to a Space Engineers LCD panel. You can improve on this, such as copying the output string to the user's clipboard
instead of creating a new file to store the string, or optimize the code. I'm only a Rust beginner.

```toml
# Cargo.toml

[package]
# Package info

[dependencies]
# The versions below might not be up to date, but they will work for the program below.
image = "0.24.0"
image_to_space_engineers_lcd = "0.1.3"
```
```rust
// main.rs

use std::fs::{File};
use std::io;
use std::io::{Write};

/// Utility function.
fn read_line() -> String {
  let mut buffer = String::new();

  match io::stdin().read_line(&mut buffer) {
    Ok(_) => {},
    Err(e) => panic!("Failed to read line. Error: {:#?}", e),
  }

  return buffer;
}

fn main() {
  println!("Enter the image path:");
  let path = read_line();
  let trimmed_path = path.trim(); // Just in case extra spaces get lost in there.
  let source = image::open(trimmed_path).unwrap();

  println!("Enter the panel ratio (width:height, for example, a Wide LCD Panel would be 2:1):");
  let aspect_ratios_raw = read_line();
  let aspect_ratios: Vec<&str> = aspect_ratios_raw.trim().split(':').collect();
  let width_aspect_ratio: u32 = aspect_ratios[0].parse().unwrap();
  let height_aspect_ratio: u32 = aspect_ratios[1].parse().unwrap();

  // Let's assume that we want to dither and don't want to stretch the image.
  let resized = image_to_space_engineers_lcd::resized(&source, width_aspect_ratio, height_aspect_ratio, true);
  let output_string = image_to_space_engineers_lcd::image_to_se_string(&resized, true, true);
  let output_path = trimmed_path.to_owned() + "_space_engineers_text.txt";
  let mut file = File::create(&output_path).unwrap();

  file.write_all(output_string.as_ref()).unwrap();
  file.flush().unwrap();

  println!("The output file can be found at {}. Open it and select everything and copy it (CTRL + A, CTRL + C). Press enter to exit...", output_path);
  read_line();

```

## How it works

Space Engineers panels can only display text or a limited selection of built-in textures,
but if you set the font to `Monospace`, special characters will be rendered with a specific color.
You can find the method for converting a color to this special character in the source of the [`to_se_char`](https://docs.rs/image_to_space_engineers_lcd/0.1.0/image_to_space_engineers_lcd/fn.to_se_char.html) function.

That is what allows us to render images instead of text, but Space Engineers also has a limited color palette and panel size.
We need to resize the image first, specifically to a width of `178 * width_aspect_ratio` and a height of `178 * height_aspect_ratio`.

When it comes to the color palette, it means that if simply take each pixel of an image and then convert it to that special character with no other processing,
the end result will be affected by [color banding](https://en.wikipedia.org/wiki/Color_banding), and pretty badly too.
You can see for yourself in the [example results](#example-results), just compare the ugly undithered image and the dithered image.

That is why we need to also apply [dithering](https://en.wikipedia.org/wiki/Dither#Digital_photography_and_image_processing) to the image.
In this case, I chose the [Floyd-Steinberg](https://en.wikipedia.org/wiki/Floyd%E2%80%93Steinberg_dithering) dithering method,
as it is one that produces excellent results while keeping things simple.
So, the process goes like this:

1. Take image
2. Resize image
3. Dither image
4. Create string
5. Get pixels in image and push a Space Engineers-compatible version to the string
6. Return string

Unless of course you choose the [`image_to_se_string`](https://docs.rs/image_to_space_engineers_lcd/0.1.0/image_to_space_engineers_lcd/fn.image_to_se_string.html)
function which doesn't dither the image, resulting in the aforementioned horrible looking image in the [example results](#example-results).
It is faster, yes, but the difference is miniscule.
I only left it public because there ***might*** some ***very special*** case when you might not want to dither the image.