use anyhow::{bail, Context, Error};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use rand::prelude::*;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

pub fn fix_the_pix(img: &DynamicImage) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, Error> {
    let (imgx, imgy) = img.dimensions();
    let mut img_buff = RgbaImage::new(imgx, imgy);

    let mut rng: ThreadRng = rand::thread_rng();
    let mut source: Vec<Rgba<u8>> = img.pixels().map(|px| px.2).collect();
    source.shuffle(&mut rng);

    for (_, _, px) in img_buff.enumerate_pixels_mut() {
        *px = match source.pop() {
            Some(px) => px,
            None => bail!("No pixels left to pop in buffer"),
        };
    }
    Ok(img_buff)
}

pub fn get_new_path(path: &Path) -> Result<PathBuf, Error> {
    // initialising the PathBuf
    let mut path_buf = match path.parent() {
        Some(p) => p.to_path_buf(),
        None => PathBuf::new(),
    };

    let file_stem: &OsStr = path.file_stem().with_context(|| "No file stem found")?;
    let file_extension: &OsStr = path
        .extension()
        .with_context(|| "No file extension found")?;

    // creating the new file name
    let mut file_name: String = String::new();
    file_name.push_str(file_stem.to_str().with_context(|| "OsStr to &str failed")?);
    file_name.push_str("_fixel");
    file_name.push('.');
    file_name.push_str(
        file_extension
            .to_str()
            .with_context(|| "OsStr to str failed")?,
    );

    path_buf.push(file_name);
    Ok(path_buf)
}
