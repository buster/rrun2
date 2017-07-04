# Rust Runner 2.0b1 #

## System Allocator ##

To reduce binary size, we are using the system allocator:

```rust
#![feature(alloc_system)]
extern crate alloc_system;
```

## Includes 

## Test first
```rust
#[test]
fn it_works() {
    println!("Hello World");
    println!("Goodbye World");
    assert!(yesyes() == true);
}
```

```rust
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

    window.connect_key_release_event(move |_, key| {
        let keyval = key.get_keyval();
        let keystate = key.get_state();
        // let keystate = (*key).state;
        debug!("key pressed: {}", keyval);
        match keyval {
            key::Escape => gtk::main_quit()
        };
        Inhibit(false)
    });

    window.set_border_width(0);
    window.set_decorated(false);
    window.show_all();
    // window.iconify();
    gtk::main();
}
```


```rust
fn yesyes() -> bool {
   true
}
```
