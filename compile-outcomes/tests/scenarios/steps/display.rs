//! The steps about how an outcome displays.

use cucumber::then;

use super::Compiling;

#[then(
    expr = "the snippet {string} displays every error the compiler rendered, starting {string}, then every warning, starting {string}"
)]
fn displays_as_rendered(world: &mut Compiling, snippet: String, error: String, warning: String) {
    let outcome = world.outcome(&snippet);
    let rendered: String = outcome
        .errors()
        .iter()
        .chain(outcome.warnings())
        .map(|diagnostic| diagnostic.rendered.as_str())
        .collect();
    let display = outcome.to_string();

    assert_eq!(display, rendered);
    let error_at = display.find(&error).expect("the display holds the error");
    let warning_at = display
        .find(&warning)
        .expect("the display holds the warning");
    assert!(error_at < warning_at, "{display}");
}
