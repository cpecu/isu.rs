pub mod img;
pub mod xml;
pub mod models;
pub mod fs;

use models::{Step, Section, Demo};
use image::png::PNGReader;
use walkdir::WalkDir;
use quick_xml::{Reader, Writer, Result as XMLResult,
    events::{Event, attributes::Attribute}
};
use std::{
    fs::File,
    path::Path,
};
use image::{ImageFormat, GenericImageView};

#[cfg(test)]
mod tests {

}



pub fn read_demo() -> XMLResult<()> {
    let mut sections: Vec<Section> = Vec::new();
    let mut steps: Vec<Step> = Vec::new();
    let demo = Reader::from_file("../../assets/demo_test1.xml")?
        .trim_text(false)
        .trim_markup_names_in_closing_tags(false);
    Ok(())
}

pub fn iter_img(path: String) {
    for entry in WalkDir::new(path.as_str()) {
        println!("{}", entry.unwrap().path().display());

    }
}

pub fn iter_dir(path: String) {
    for entry in std::fs::read_dir(path).unwrap() {
        let file = entry.unwrap().path();
    }
}

pub mod test {

    pub fn can_read_demo_file() {
       //let demo_path = std:fs::Path
    }

}
