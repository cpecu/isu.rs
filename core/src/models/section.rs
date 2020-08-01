use uuid::Variant::Microsoft;
use std::rc::Rc;
use uuid::Uuid;
use super::{FloatCoords, IntCoords, IntDims};
use super::step::Step;

pub struct Section {
    pub id: Uuid,
    pub xml_name: String,
    pub steps: Vec<Rc<Step>>,
    pub is_active: bool,
    pub click_anywhere: bool,
}
