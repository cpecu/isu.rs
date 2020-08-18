#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]


mod cmd;
mod handlers;

use cmd::ImageKind;
use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};
use tauri::api::{
    path::{document_dir, cache_dir, config_dir, picture_dir},
    dialog::{save_file, pick_folder},
    file::{read_string, read_binary},
    notification::Notification,
    rpc::format_callback_result,
};
use quick_xml::{
    Reader, Writer, de::from_str, Result as XmlResult,
    events::{Event, BytesStart, BytesEnd, BytesText},
};

fn main() {
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => {
          Err(e.to_string())
        }
        Ok(command) => {
          match command {
            LoadDemo { path } => {
                parse(path);
            },
            SaveDemo { path } => {
              println!("{}", path);
            }
            OpenImage { path, kind } => {
                match kind { 
                    ImageKind::Bg => {  },
                    ImageKind::Shell => {  },
                    ImageKind::Asset => {  },
                    ImageKind::CombinedBgShell => {  },
                    _ => { },
                }
              println!("{}", path);
            },
            AttachAudio => { },
            Pace => { },
            Section => { },
            _ => {}
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}

pub enum Elements {
    Demo(String),
    Step(String),  
    Section(String),
}

pub struct Steps {
 
}

pub fn parse(path: String) -> XmlResult<()> {
    let mut demo = Reader::from_file(&path).unwrap();
    demo
        .trim_text(true)
        .trim_markup_names_in_closing_tags(false);
    Notification::new()
        .title("Demo loaded")
        .body(format!("Loaded from {}", &path))
        .show().unwrap();
    let mut buf = Vec::new();
    let get_text = |bytes: std::io::Bytes<&[u8]>| {
        String::from_utf8(bytes.fold(Vec::new(), |mut vec, byt| {
            vec.push(byt.unwrap());
            vec
        })).unwrap()
    };
    loop {
        match demo.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let txt = get_text(e.bytes());
                println!("Got start: {}", txt);
            }
            Ok(Event::Text(e)) => { 
                let txt = get_text(e.bytes());
                println!("Got text: {}", txt);
            }
            Ok(Event::End(ref e)) => {
                let txt = get_text(e.bytes());
                println!("Got end: {}", txt);
            }
            Ok(Event::Eof) => break,
            Err(e) => {

            }
            _ => {}
        }    
    }
    Ok(())
}


/*
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
*/
