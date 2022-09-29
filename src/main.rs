use anyhow::{Context, Result};
use clap::{value_parser, Arg, Command};
use image::{ImageBuffer, Rgba};
use std::path::{Path, PathBuf};

mod fixel;

fn main() -> Result<()> {
    let matches = Command::new("fixel")
        .version("0.1.0")
        .author("Nicholas L.")
        .about("Fixel randomly reorders the pixels of an image.")
        .arg(
            Arg::new("image")
                .required(true)
                .value_parser(value_parser!(String))
                .help("Name of image that should be used"),
        )
        .get_matches();
    // getting argument
    let image_file_name: &String = matches
        .get_one::<String>("image")
        .expect("`image` argument is required");

    // image_path is the path provided by the user
    let image_path: &Path = Path::new(image_file_name);
    let img = image::open(&image_path).with_context(|| {
        format!("could not open image file: {}", &image_path.display())
    })?;
    // creating new image path
    let path_buf: PathBuf = fixel::get_new_path(image_path);
    let new_image_path: &Path = path_buf.as_path();
    // fuzzing the pixels
    let img_buff: ImageBuffer<Rgba<u8>, Vec<u8>> = fixel::fix_the_pix(&img);

    println!(
        "Saving fuzzy image: `{}`",
        new_image_path
            .to_str()
            .expect("Could not covert path to str")
    );
    img_buff.save(new_image_path).with_context(|| {
        format!("could not save image file: {}", &new_image_path.display())
    })?;

    Ok(())
}
