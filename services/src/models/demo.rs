use uuid::Variant::Microsoft;
use std::rc::Rc;
use std::io::Cursor;
use uuid::Uuid;
use super::{FloatCoords, IntCoords, IntDims};
use super::section::Section;
use serde::{Serialize, Deserialize};
use serde_derive::*;
use quick_xml::{Reader, Writer, Result as XMLResult,
    events::{Event, attributes::Attribute}
};
use super::*;


#[derive(Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all="PascalCase", default)]
pub struct Demo {
    pub id: Uuid,
    pub demo_name: String,
    pub project: String,
    pub demo_version: f32,
    pub demo_schema_version: f32,
    pub xml_name: String,
    pub welcome_message: Option<String>,
    #[serde(rename="Chapter")]
    pub sections: Vec<Section>,
    pub assets_directory: String,
    pub loaded_from_file_name: String,
    pub local_lock: String,
}

pub struct DemoVec {

}

impl Demo {
    pub fn new(url: &'static str) -> XMLResult<()> {
        let mut demo_buf = Vec::new();
        let mut sections: Vec<Section> = Vec::new();
        let mut steps: Vec<Step> = Vec::new();
        let mut demo_out = Writer::new(Cursor::new(Vec::new()));
        let mut demo = Reader::from_file(url)?;
        let mut sect_count = 0;
        let mut step_count = 0;
        demo.trim_text(false)
            .expand_empty_elements(false)
            .trim_markup_names_in_closing_tags(false);
        loop {
            match demo.read_event(&mut demo_buf) {
                Ok(Event::Start(ref e)) => { 
                    match e.name() { 
                        b"Chapter" => { //send to new section constructor
                            sect_count += 1;
                            println!("Section {}", &sect_count);
                        },
                        b"Step" => { //in section constructor , send ot new step constr
                            step_count +=1;
                            println!("Step {}", &step_count);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() { 
                        b"Chapter" => { //send to new section constructor
                            sect_count += 1;
                            println!("Section {}", &sect_count);
                        },
                        b"Step" => { //in section constructor , send ot new step constr
                            step_count +=1;
                            println!("Step {}", &step_count);
                        },
                        _ => (),
                    }
                },
                Ok(Event::Text(ref e)) => {
                    println!("Text {}", String::from_utf8(e.to_vec()).unwrap());
                },
                Ok(Event::CData(ref e)) => {
                    println!("Cdata {}", String::from_utf8(e.to_vec()).unwrap());
                },
                Ok(Event::Eof) => {
                    println!("EOF");
                    break
                },
                Ok(e) => {
                    println!("Other");
                },
                Err(e) => panic!("Error {} {}", demo.buffer_position(), e),
            }
            demo_buf.clear();
        }
        let out = demo_out.into_inner().into_inner();
        Ok(())
    }

    pub fn write() -> XMLResult<()> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        let mut demo_buf: Vec<&[u8]> = Vec::new();
        Ok(())
    }
}
