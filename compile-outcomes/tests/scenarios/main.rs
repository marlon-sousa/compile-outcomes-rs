//! The Gherkin scenarios of `tests/features/`, which compile real snippets with the real Cargo.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    reason = "a step fails by panicking, and clippy.toml allows these only in #[test] functions"
)]

mod steps;

use cucumber::World;
use futures::executor::block_on;
use steps::{Compiling, forget_earlier_project};

fn main() {
    forget_earlier_project();
    let scenarios = Compiling::cucumber()
        .max_concurrent_scenarios(1)
        .fail_on_skipped()
        .run_and_exit("tests/features");
    block_on(scenarios);
}
