#[macro_use]
extern crate log;
extern crate quale;
mod completionengines;
use completionengines::CompletionEngine;

struct Commander<'engine> {
    engines: Vec<&'engine Box<CompletionEngine>>,
}

