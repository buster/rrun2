#![feature(alloc_system)]
extern crate alloc_system;
#[macro_use]
extern crate log;
extern crate simplelog;

extern crate gtk;
extern crate gdk;

extern crate commander;

use gtk::prelude::*;
use gtk::{Builder, Window, TreeView, SearchEntry};

fn main() {
    setup_logging(simplelog::LogLevelFilter::Debug);
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));
    const GLADE_STRING: &'static str = include_str!("rrun.glade");
    let mut builder = Builder::new_from_string(GLADE_STRING);
    let window: Window = builder.get_object("rrun").unwrap();
    let completion_list: TreeView = builder.get_object("completion_view").unwrap();
    let entry: SearchEntry = builder.get_object("search_entry").unwrap();
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.connect_key_release_event(move |_, key| {
        let keyval = key.get_keyval();
        let keystate = key.get_state();
        debug!("key pressed: {}", keyval);
        match keyval {
            gdk::enums::key::Escape => {
                info!("Quitting...");
                gtk::main_quit()
            }
            _ => (),
        };
        Inhibit(false)
    });

    window.set_border_width(0);
    window.set_decorated(false);
    window.show_all();
    // window.iconify();
    gtk::main();
}

fn setup_logging(log_level: simplelog::LogLevelFilter) {
    use simplelog::{Config, TermLogger, CombinedLogger};
    CombinedLogger::init(
        vec![
            TermLogger::new(log_level, Config::default()).unwrap(),
            //WriteLogger::new(log_level, Config::default(), File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();
}