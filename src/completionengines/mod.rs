pub mod math;

pub trait CompletionEngine {
    fn get_suggestions(&self, input: &str) -> Vec<Box<Suggestion>>;
}

pub trait Suggestion {
    fn get_text_preview(&self) -> Option<String>;
    fn execute(&self) -> Option<String>;
}