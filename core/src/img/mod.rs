use std::{
    fs::File,
    path::Path,
    rc::Weak,
};
use image::{
    GenericImageView, ImageError, RgbaImage,
    open, load, ImageResult,
    png::{PNGReader, PNGEncoder}, //FilterType
    imageops::{crop, resize, FilterType},
    
};
use super::models::{IntDims, IntCoords};

pub fn open_img(path: &str) -> Result<(), image::ImageError> {
    let img = image::open(path)?;
    Ok(())
}


pub enum ImageType {
    Asset(String),
    AssetBackground(String),
    AssetForeground(String),
    Edited, //make separate variation for bg, fg, and asset?
}

pub struct Img {
    img: RgbaImage,
    img_type: ImageType,
    path: String,
}

impl Img {
    pub fn from(path: &str, img_type: ImageType) -> ImageResult<Self> {
        let img = Img {
            path: path.to_string(), img_type, 
            img: image::open(path)?
                .as_rgba8()
                .unwrap()
                .to_owned(),
        };
        Ok(img)
    }

    pub fn as_sub_img_from(base_img: &mut Self, coords: IntCoords, dims: IntDims) -> ImageResult<Self> {
        let sub_img = Self {
            img: crop(&mut base_img.img, coords.x, coords.y, dims.w, dims.h).to_image(),
            path: base_img.path.clone(), img_type: ImageType::Edited,
        };
        Ok(sub_img)
    }

    pub fn get_dims(self) -> IntDims {
        self.img.dimensions().into()
    }

    pub fn crop(&mut self, coords: IntCoords, dims: IntDims) -> ImageResult<&mut Self> {
        let crop_img = crop(&mut self.img, 
            coords.x, coords.y, 
            dims.w, dims.h)
            .to_image();
        self.img = crop_img;
        Ok(self)
    }

    pub fn resize(&mut self, new_dims: IntDims) -> ImageResult<&mut Self> {
        let resize_img = image::imageops::resize(&mut self.img,
            new_dims.w, new_dims.h, FilterType::Nearest);
        self.img = resize_img;
        Ok(self)
    }

    pub fn save(self, path: &str) -> ImageResult<Self> {
        self.img.save(path)?;
        Ok(self)
    }
}

