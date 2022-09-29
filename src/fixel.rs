use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use rand::prelude::*;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

pub fn fix_the_pix(img: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (imgx, imgy) = img.dimensions();
    let mut img_buff = RgbaImage::new(imgx, imgy);

    let mut rng: ThreadRng = rand::thread_rng();
    let mut source: Vec<Rgba<u8>> = img.pixels().map(|px| px.2).collect();
    source.shuffle(&mut rng);

    for (_, _, px) in img_buff.enumerate_pixels_mut() {
        *px = source.pop().unwrap();
    }
    img_buff
}

pub fn get_new_path(path: &Path) -> PathBuf {
    // initialising the PathBuf
    let mut path_buf = match path.parent() {
        Some(p) => p.to_path_buf(),
        None => PathBuf::new(),
    };

    let file_stem: &OsStr = path.file_stem().expect("No file stem found");
    let file_extension: &OsStr =
        path.extension().expect("No file extension found");

    // creating the new file name
    let mut file_name: String = String::new();
    file_name.push_str(file_stem.to_str().expect("OsStr to str failed"));
    file_name.push_str("_fixel");
    file_name.push('.');
    file_name.push_str(file_extension.to_str().expect("OsStr to str failed"));

    path_buf.push(file_name);
    path_buf
}