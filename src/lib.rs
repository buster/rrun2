#[macro_use]
extern crate log;
extern crate quale;

struct Commander<'engine> {
    engines: Vec<&'engine Box<CompletionEngine>>
}

trait CompletionEngine {
    fn get_suggestions(&self, input: &str) -> Vec<Box<Suggestion>>;
}

trait Suggestion {
    fn get_text_preview(&self) -> Option<String>;
    fn execute(&self) -> Option<String>;
}

struct MathSuggestion {
    formula: String
}

impl Suggestion for MathSuggestion {
    fn execute(&self) -> Option<String> {
        let output = std::process::Command::new("sh")
                     .arg("-c")
                     .arg(format!("qalc {}", self.formula))
                     .output()
                     .expect("Expected qalc output!");
        let output_str = String::from_utf8_lossy(&output.stdout).to_string();
        info!("Got {} as Output!", output_str);
        Some(output_str)
    }
    fn get_text_preview(&self) -> Option<String> {
        unimplemented!();
    }
}

#[derive(Debug)]
struct MathCompletionEngine {
    name: String
}

impl MathCompletionEngine {
    fn init() -> Option<MathCompletionEngine> {
        match quale::which("qalc") {
            Some(_) => Some(MathCompletionEngine { name: "MathCompletionEngine".to_owned() }),
            None => None
        }
        
    }
}

impl CompletionEngine for MathCompletionEngine {
    fn get_suggestions(&self, input: &str) -> Vec<Box<Suggestion>> {
        vec![Box::new(MathSuggestion { formula: input.to_owned() })]
    }
}

#[test]
fn should_not_run_text_string_through_qalc() {
    unimplemented!();
}

#[test]
fn should_not_initialize_math_engine_if_qalc_is_missing() {
    std::env::remove_var("PATH");
    let eng = MathCompletionEngine::init();
    println!("{:?}", eng);
    assert!(eng.is_none());
}

#[test]
fn should_calculate_1_and_1() {
    let math_engine = MathCompletionEngine::init().unwrap();
    let calculation = "1+1";
    let suggestions = math_engine.get_suggestions(&calculation);

    assert!(suggestions.len() == 1);
    assert!(suggestions[0].execute().unwrap() == "1 + 1 = 2\n");
}