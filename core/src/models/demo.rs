use uuid::Variant::Microsoft;
use std::rc::Rc;
use uuid::Uuid;
use super::{FloatCoords, IntCoords, IntDims};
use super::section::Section;

pub struct Demo {
    pub id: Uuid,
    pub demo_name: String,
    pub project: String,
    pub demo_version: f32,
    pub demo_schema_version: f32,
    pub xml_name: String,
    pub welcome_message: Option<String>,
    pub sections: Vec<Rc<Section>>,
    pub assets_directory: String,
    pub loaded_from_file_name: String,
    pub local_lock: String,
}
