use uuid::Variant::Microsoft;
use uuid::Uuid;
use super::{FloatCoords, IntCoords, IntDims};

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

pub struct Step {
    pub id: Uuid,
    pub picture_file: String,
    pub time: String, //for now -- try and interpret it
    pub mouse_coordinates: FloatCoords,
    pub assets_directory: String,
    pub hotspots: Vec<Hotspot>,
    pub video_rects: Option<Vec<VideoRect>>,
    pub jump_rects: Option<Vec<JumpRect>>,
    pub text_rects: Option<Vec<TextRect>>,
    pub highlight_rects: Option<Vec<HighlightRect>>,
    pub mouse_enter_picture: Option<MouseEnterPicture>,
    pub is_guided: bool,
    pub is_pointer_suppressed: bool,
    pub xml_script: String, //encoded talking point
    pub xml_instructions: String, // encoded ci
    pub xml_name: String,
    pub step_flavor: String,
    pub transition_type: Option<String>,
    pub instructions_orientation: String,
    pub step_delay: f32,
}

pub struct MouseEnterPicture {
    pub picture_file: String,
    pub time: String,
    pub mouse_coordinates: FloatCoords,
}

pub struct Hotspot {
    pub top_left: IntCoords,
    pub bottom_right: IntDims,
}

pub struct VideoRect {
    pub coords: IntCoords,
    pub dims: IntDims,

}

pub struct JumpRect {
    pub coords: IntCoords,
    pub dims: IntDims,

}

pub struct TextRect {
    pub coords: IntCoords,
    pub dims: IntDims,

}

pub struct HighlightRect {
    pub coords: IntCoords,
    pub dims: IntDims,

}

