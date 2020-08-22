#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
mod handlers;

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
            MyCustomCommand { argument } => {
              println!("{}", argument);
            }
          }
          Ok(())
        }
      }
    })
   .build()
   .run();
}
