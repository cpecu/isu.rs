#![windows_subsystem = "windows"]

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwg::{NativeUi, Button, ButtonBuilder, TextInput};
use nwd::NwgUi;

fn main() { 
    nwg::init().expect("Failed to init NWG");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set font");
    let ui: ui::AppUi = App::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}

#[derive(Default, NwgUi)]
pub struct App {

    #[nwg_control(size: (600, 480), position: (300, 300), title: "Impresys Utils", flags: "WINDOW|VISIBLE")]
    #[nwg_events(OnWindowClose: [App::close])]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    layout: nwg::GridLayout,

    #[nwg_control(text: "Demo path", size: (280, 25), position: (10, 10))]
    demo_path: nwg::TextInput,

    #[nwg_control(text: "Browse demo", size: (280, 25), position: (10, 40))]
    #[nwg_events(OnButtonClick: [App::browse_demo])]
    browse: nwg::Button,

    #[nwg_control(text: "Browse image", size: (280, 25), position: (10, 70))]
    #[nwg_events(OnButtonClick: [App::browse_img])]
    img_browse: nwg::Button,

    #[nwg_control(text: "Run", size: (280, 25), position: (10, 100))]
    #[nwg_events(OnButtonClick: [App::run_task])]
    run: nwg::Button,

    #[nwg_control(text: "Run", size: (280, 25))]
    shell_browse: nwg::Button, 
}

#[derive(Debug, NwgUi)]
pub struct Shell {
}

pub struct ShellImg {  }

/// Implementation
impl App {

    fn run(&self) {
        nwg::simple_message("Hello!", &format!("Demo {}!", self.demo_path.text()));
    }

    fn close(&self) {
        nwg::simple_message("Goodbye", &format!("Demo, {}", self.demo_path.text())); 
    }

    fn browse_demo(&self) {
        nwg::simple_message("Browse demo", &format!("Browse demo {}", self.demo_path.text()));
    }

    fn browse_img(&self) {
        nwg::simple_message("Browse demo", &format!("Browse demo {}", self.demo_path.text()));
    }

    fn run_task(&self) {
        nwg::simple_message("Browse demo", &format!("Browse demo {}", self.demo_path.text()));
    }

    fn shell(&self) {
        nwg::simple_message("Browse", &format!("Browse shell img {}", self.demo_path.text()));
    }
}


pub struct DemoFile {

}

impl DemoFile {

}

pub mod ui {

    use super::*;
    use std::{
        rc::Rc, cell::RefCell, ops::Deref,
    };

    pub struct AppUi {
        inner: Rc<App>,
        handler: RefCell<Option<nwg::EventHandler>>,
    }

    impl nwg::NativeUi<AppUi> for App {

        fn build_ui(mut data: App) -> Result<AppUi, nwg::NwgError> {
            use nwg::Event as E;

            nwg::Window::builder()
                .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
                .size((300, 115))
                .position((300, 300))
                .title("Impresys Utils")
                .build(&mut data.window)?;

            let ui = AppUi {
                inner: Rc::new(data),
                handler: Default::default(),
            };

            let evt_ui = Rc::downgrade(&ui.inner);
            let evt_handler = move |evt, _evt_data, handle| {
                if let Some(ui) = evt_ui.upgrade() {
                    match evt {
                        E::OnButtonClick => {
                            if &handle == &ui.run {
                                App::run(&ui);
                            }
                        },
                        E::OnWindowClose => {
                            if &handle == &ui.window {
                                App::close(&ui);
                            }
                        },
                        _ => {}
                    }
                }
            };
            *ui.handler.borrow_mut() = Some(nwg::full_bind_event_handler(&ui.window.handle, evt_handler));

            Ok(ui)
        }

    }

    impl Drop for AppUi {
        fn drop(&mut self) {
            let handler = self.handler.borrow();
            if handler.is_some() {
                nwg::unbind_event_handler(handler.as_ref().unwrap())
            }
        }
    }

    impl Deref for AppUi {
        type Target = App;
        fn deref(&self) -> &App {
            &self.inner
        }
    }

}