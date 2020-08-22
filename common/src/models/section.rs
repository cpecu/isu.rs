use uuid::Variant::Microsoft;
use uuid::Uuid;
use super::{FloatCoords, IntCoords, IntDims};
use super::step::Step;
use serde::{Serialize, Deserialize};
use serde_derive::*;

#[derive(Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all="PascalCase", default)]
pub struct Section {
    pub id: Uuid,
    pub xml_name: String,
    #[serde(rename="Step")]
    pub steps: Vec<Step>,
    pub is_active: bool,
    pub click_anywhere: bool,
}
