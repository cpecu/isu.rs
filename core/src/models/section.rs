use uuid::Variant::Microsoft;
use uuid::Uuid;
use super::{FloatCoords, IntCoords, IntDims};
use super::step::Step;
use serde::{Serialize, Deserialize};
use serde_derive::*;

#[derive(Serialize, Deserialize)]
pub struct Section {
    pub id: Uuid,
    pub xml_name: String,
    pub steps: Vec<Step>,
    pub is_active: bool,
    pub click_anywhere: bool,
}
