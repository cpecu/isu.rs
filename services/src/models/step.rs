use uuid::Variant::Microsoft;
use std::path::{PathBuf, Path};
use image::{
    DynamicImage, ImageResult,
    imageops::FilterType,
};
use uuid::Uuid;
use super::{FloatCoords, IntCoords, IntDims};
use serde::{Serialize, Deserialize};
use serde::*;

// TODO: Handle different steps with enums or trait?
pub trait DemoStep {
    
}

pub enum StepKind {
    Animated, // -> intro, slide steps, etc
    Guided,
    Click,
    Video,
    Jump
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(rename_all="PascalCase")]
pub struct Step {
    #[serde(rename="$value")]
    pub id: Uuid,
    #[serde(rename="$value")]
    pub picture_file: String,
    #[serde(rename="$value")]
    pub time: String, //for now -- try and interpret it
    #[serde(rename="$value")]
    pub mouse_coordinates: FloatCoords,
    #[serde(rename="$value")]
    pub assets_directory: String,
    #[serde(rename="$value")]
    pub hotspots: Vec<Hotspot>,
    #[serde(rename="$value", skip_serializing_if="Option::is_none")]
    pub video_rects: Option<Vec<VideoRect>>,
    #[serde(rename="$value", skip_serializing_if="Option::is_none")]
    pub jump_rects: Option<Vec<JumpRect>>,
    #[serde(rename="$value", skip_serializing_if="Option::is_none")]
    pub text_rects: Option<Vec<TextRect>>,
    #[serde(rename="$value", skip_serializing_if="Option::is_none")]
    pub highlight_rects: Option<Vec<HighlightRect>>,
    #[serde(rename="$value", skip_serializing_if="Option::is_none")]
    pub mouse_enter_picture: Option<MouseEnterPicture>,
    #[serde(rename="$value")]
    pub is_guided: bool,
    #[serde(rename="$value")]
    pub is_pointer_suppressed: bool,
    #[serde(rename="$value")]
    pub xml_script: String, //encoded talking point
    #[serde(rename="$value")]
    pub xml_instructions: String, // encoded ci
    #[serde(rename="$value")]
    pub xml_name: String,
    #[serde(rename="$value")]
    pub step_flavor: String,
    #[serde(rename="$value")]
    pub transition_type: String,
    #[serde(rename="$value")]
    pub instructions_orientation: String,
    #[serde(rename="$value")]
    pub step_delay: f32,
}

impl Step {
    pub fn get_img_paths(self) -> Vec<PathBuf> {
        walkdir::WalkDir::new(self.assets_directory)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| { 
                e.path()
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|e| e.to_lowercase().eq(".png"))
                    .unwrap()
            })
            .map(|entry| entry.unwrap().path().to_path_buf() )
            .collect::<Vec<PathBuf>>()
            .to_vec()
    }

    pub fn get_imgs(self) -> Vec<DynamicImage> {
        self.get_img_paths()
            .into_iter()
            .map(|entry| {
                image::open(entry).unwrap()
            })
            .into_iter()
            .collect::<Vec<image::DynamicImage>>()
            .to_vec()
    }

    pub fn resize_assets(self, width: u32, height: u32) -> Vec<DynamicImage> {
        self.get_imgs()
            .into_iter()
            .map(|img| {
                let res = img.resize(width, height, FilterType::Nearest);
                res
        }).collect::<Vec<DynamicImage>>()
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct MouseEnterPicture {
    pub picture_file: String,
    pub time: String,
    pub mouse_coordinates: FloatCoords,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Hotspot {
    pub top_left: IntCoords,
    pub bottom_right: IntDims,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct VideoRect {
    pub coords: IntCoords,
    pub dims: IntDims,

}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct JumpRect {
    pub coords: IntCoords,
    pub dims: IntDims,

}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct TextRect {
    pub coords: IntCoords,
    pub dims: IntDims,

}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct HighlightRect {
    pub coords: IntCoords,
    pub dims: IntDims,

}

