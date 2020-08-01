use std::{str, io::BufReader, fs::File};
use quick_xml::Reader;
use quick_xml::events::Event;

pub struct ETree {
    path: String,
}

pub fn read_from_file(path: &str) -> quick_xml::Result<Reader<BufReader<File>>>{
    let mut reader = Reader::from_file(path)?;
    Ok(reader)
}

//pub fn loop
