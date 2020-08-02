use uuid::Variant::Microsoft;
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

#[derive(Serialize, Deserialize)]
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
    #[serde(rename="$value")]
    pub video_rects: Option<Vec<VideoRect>>,
    #[serde(rename="$value")]
    pub jump_rects: Option<Vec<JumpRect>>,
    #[serde(rename="$value")]
    pub text_rects: Option<Vec<TextRect>>,
    #[serde(rename="$value")]
    pub highlight_rects: Option<Vec<HighlightRect>>,
    #[serde(rename="$value")]
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
    pub transition_type: Option<String>,
    #[serde(rename="$value")]
    pub instructions_orientation: String,
    #[serde(rename="$value")]
    pub step_delay: f32,
}

#[derive(Serialize, Deserialize)]
pub struct MouseEnterPicture {
    pub picture_file: String,
    pub time: String,
    pub mouse_coordinates: FloatCoords,
}

#[derive(Serialize, Deserialize)]
pub struct Hotspot {
    pub top_left: IntCoords,
    pub bottom_right: IntDims,
}

#[derive(Serialize, Deserialize)]
pub struct VideoRect {
    pub coords: IntCoords,
    pub dims: IntDims,

}

#[derive(Serialize, Deserialize)]
pub struct JumpRect {
    pub coords: IntCoords,
    pub dims: IntDims,

}

#[derive(Serialize, Deserialize)]
pub struct TextRect {
    pub coords: IntCoords,
    pub dims: IntDims,

}

#[derive(Serialize, Deserialize)]
pub struct HighlightRect {
    pub coords: IntCoords,
    pub dims: IntDims,

}

