use std::path::Path;
use std::sync::LazyLock;

use compile_outcomes::{Compilation, Dependency, Edition, Outcome, Outcomes, compile};
use cucumber::{World, given, then, when};

/// Every snippet of tests/ui, compiled once, the first time a scenario asks.
static OUTCOMES: LazyLock<Outcomes> = LazyLock::new(|| {
    let work_dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("ui");
    let compilation = Compilation::new(Edition::E2021, work_dir)
        .snippets_in("tests/ui")
        .dependency(Dependency::path("my_macros", "."));
    compile(&compilation).expect("the compilation is well described")
});

#[derive(Debug, Default, World)]
struct Ui {
    snippet: String,
    outcome: Option<&'static Outcome>,
}

#[given(expr = "the snippet {string}")]
fn the_snippet(ui: &mut Ui, name: String) {
    ui.snippet = name;
}

#[when("it is compiled")]
fn it_is_compiled(ui: &mut Ui) {
    ui.outcome = OUTCOMES.get(&ui.snippet);
}

#[then(expr = "it fails with an error that contains {string}")]
fn fails_with(ui: &mut Ui, words: String) {
    let outcome = ui.outcome.expect("the snippet is in tests/ui");
    assert!(outcome.is_failed(), "{outcome:#?}");
    let said = outcome
        .errors()
        .iter()
        .any(|error| error.message.contains(&words));
    assert!(said, "{outcome:#?}");
}

fn main() {
    futures::executor::block_on(Ui::run("tests/features"));
}
