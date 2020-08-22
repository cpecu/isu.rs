use std::{
    collections::HashMap, 
    io::{Read, Cursor, BufReader},
    fs::File,
    borrow::Cow,
    path::Path,
};
use serde::{Deserialize, Serialize};
use quick_xml::{Reader, Writer, 
    events::{BytesStart, BytesText, BytesEnd, BytesDecl, Event},
};

pub struct Prop(&'static str);
pub struct Value(&'static str);
pub struct DemoXml<'a> {
    path: Cow<'a, str>,
    data: Option<String>,
    demo_info: Option<DemoInfo>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct DemoInfo {
    title: &'static str,
    assets_dir: &'static str,
    num_sect: usize,
    num_steps: usize,
    audio_dir: Option<&'static str>,
    script_path: Option<&'static str>,
}

impl<'a> DemoXml<'a> {

    pub fn new<S: Into<String>>(path: S) -> Self {
        let demo_path = Cow::Borrowed(path.into().as_str());
        Self { path: demo_path, data: None, demo_info: None }
    }

    pub fn read(mut self) -> quick_xml::Result<String> {
        let path = self.path;
        let mut reader = Reader::from_file(self.path.as_ref())?
            .trim_text(true)
            .trim_markup_names_in_closing_tags(true);
        let mut buf: Vec<u8> = Vec::new();
        let mut sect_count = 0;
        let mut step_count = 0;
        let mut props: HashMap<String, String> = HashMap::new();
        let get_text = |bytes: std::io::Bytes<&[u8]>| {
            String::from_utf8(bytes.fold(Vec::new(), |mut vec, byt| {
                vec.push(byt.unwrap());
                vec
            })).unwrap()
        };
        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(el) => {
                    println!("Got tag: {}", get_text(el.bytes()));
                    match el.name() {
                        b"Chapters" => {},
                        b"Steps" => {},
                        b"Chapter" => {
                            let pos = reader.buffer_position();
                            let data = self.parse_sect(reader).unwrap();
                        }
                        _ => {},
                    }
                },
                Event::Text(txt) => {
                    println!("Got tag: {}", get_text(txt.bytes()));
                },
                Event::Decl(dec) => {
                    println!("Got decl {}", get_text(dec.bytes()));
                },
                Event::End(el) => {
                    println!("Got end: {}", get_text(el.bytes()));
                },
                Event::Empty(emp) => {
                    println!("Got empty: {}", get_text(emp.bytes()));
                },
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }
        Ok(String::new())
    }

    pub fn parse_sect(&mut self, reader: &mut Reader<BufReader<File>>) -> quick_xml::Result<()> { //use position insead of passing reader?
        let mut buf: Vec<u8> = Vec::new();
        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(el) => {
                    match el.name() { //TODO general function to serialize all section specific fields to struct
                        b"ID" => { 
                            let id = reader.read_text(el.name(), &mut buf);
                        },
                        b"XmlName" => {
                            let xmlName = reader.read_text(el.name(), &mut buf);
                        }
                        b"Steps" => {},
                        b"Step" => {
                            println!("Got step!");
                            let data = self.parse_step(reader).unwrap();
                        },
                        _ => {}
                    }
                },
                Event::Text(txt) => {

                },
                Event::End(el) => {

                },
                Event::Empty(emp) => {},
                _ => {}
            }
        }
        Ok(())
    }

    pub fn parse_step(&mut self, reader: &mut Reader<BufReader<File>>) -> quick_xml::Result<()> {
        let mut buf: Vec<u8> = Vec::new();
        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(el) => {
                    match el.name() {
                        _ => {}
                    }
                },
                Event::Text(txt) => {

                },
                Event::End(el) => {

                },
                Event::Empty(emp) => {},
                _ => {}
            }
        }
        Ok(())
    }

    pub fn get_assets_dir<'a, S: Into<String>>(path: S) -> Cow<'a, str> {
        let mut dir: String = path.into(); 
        dir.push_str("_Assets");
        Cow::Borrowed(dir.as_str())
    
    }

}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    pub fn can_load_parse_demo() {
        let demo_file = r"../../assets/DEVEL.demo";
    }
}