extern crate image;
extern crate zip;

use image::DynamicImage;
use image::ImageError;

use std::io;
use std::io::Error;
use std::io::Read;
use std::fs::File;
use std::vec::Vec;

use zip::result::ZipError;
use std::num::{ParseFloatError, ParseIntError};


#[derive(Debug)]
pub enum LightfieldError {
    IoError(Error),
    ImageError(ImageError),
    ZipError(ZipError),
    ParseError(String),
}
impl From<io::Error> for LightfieldError {
    fn from(err: io::Error) -> Self {
        LightfieldError::IoError(err)
    }
}
impl From<ParseFloatError> for LightfieldError {
    fn from(e: ParseFloatError) -> Self {
        LightfieldError::ParseError(format!("Invalid float: {}", e))
    }
}
impl From<ParseIntError> for LightfieldError {
    fn from(e: ParseIntError) -> Self {
        LightfieldError::ParseError(format!("Invalid int: {}", e))
    }
}
impl From<ZipError> for LightfieldError {
    fn from(err: ZipError) -> Self {
        LightfieldError::ZipError(err)
    }
}
impl From<ImageError> for LightfieldError {
    fn from(err: ImageError) -> Self {
        LightfieldError::ImageError(err)
    }
}

pub struct LightfieldView {
    pub image: DynamicImage,
    pub ix: i32,
    pub iy: i32,
    pub x: f32,
    pub y: f32,
}

pub struct Lightfield {
    pub views: Vec<LightfieldView>,
}

impl Lightfield {
    /// load lightfield from a zip file in the format used by the
    /// rectified files from the (new) stanford lightfield archive, found at
    /// http://lightfield.stanford.edu/lfs.html
    pub fn from_zip(zip_filename: &str) -> Result<Lightfield, LightfieldError> {
        let zipfile = try!(File::open(zip_filename));
        let mut archive = try!(zip::ZipArchive::new(zipfile));

        let mut views = Vec::with_capacity(archive.len());
        for i in 0..archive.len()
        {
            let mut file = &mut try!(archive.by_index(i));
            let name = String::from(file.name());
            println!("loading {}", name);
            let parts: Vec<&str> = name.split("_").collect();
            if parts.len() < 5 {
                return Err(LightfieldError::ParseError(format!("Invalid filename '{}'", name)));
            }
            let ix:i32 = try!(parts[1].parse());
            let iy:i32 = try!(parts[2].parse());
            let  x:f32 = try!(parts[3].parse());
            let  y:f32 = try!(parts[4].parse());
            let mut contents = Vec::new();
            try!(file.read_to_end(&mut contents));

            let image = try!(image::load_from_memory(&contents));
            views.push(LightfieldView {
                image: image,
                ix: ix,
                iy: iy,
                x: x,
                y: y,
            });
        }
        Ok(Lightfield {views: views})
    }
}