#![feature(alloc_system)]
extern crate alloc_system;
extern crate gtk;

use gtk::prelude::*;
use gtk::{Builder, Window, TreeView, SearchEntry};
use std::io::Read;
use std::fs::File;

fn main() {
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
    window.set_border_width(0);
    window.set_decorated(false);
    window.show_all();
    gtk::main();
}
