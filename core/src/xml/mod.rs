
use std::{str, io::BufReader, fs::File};
use quick_xml::{Reader, Writer};
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};

pub struct ETree {
    path: String,
}

pub fn new_reader(path: &str) -> quick_xml::Result<Reader<BufReader<File>>>{
    let mut reader = Reader::from_file(path)?;
    Ok(reader)
}

pub fn new_writer() -> () {}

//pub fn loop
