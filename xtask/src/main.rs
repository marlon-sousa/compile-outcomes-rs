//! Handler: runs the quality gate, `cargo xtask <mode>`, as docs/WORKFLOW.md, section 7, lists its modes.

mod architecture;
mod modes;
mod readme;
mod run;

use std::env;
use std::path::Path;
use std::process::ExitCode;

use modes::{Mode, Step, requested, step};
use strum::VariantNames;

fn main() -> ExitCode {
    let argument = env::args().nth(1);
    let Some(modes) = requested(argument.as_deref()) else {
        eprintln!(
            "Unknown mode {}. The modes are ci, {}.",
            argument.unwrap_or_default(),
            Mode::VARIANTS.join(", ")
        );
        return ExitCode::FAILURE;
    };
    let mut failed = Vec::new();
    for mode in modes {
        if !passes(step(mode)) {
            failed.push(mode.to_string());
        }
    }
    if failed.is_empty() {
        println!("\nThe quality gate passed.");
        ExitCode::SUCCESS
    } else {
        eprintln!("\nThe quality gate failed: {}.", failed.join(", "));
        ExitCode::FAILURE
    }
}

fn passes(step: Step) -> bool {
    match step {
        Step::Cargo(arguments) => run::cargo(arguments),
        Step::Rustdoc(arguments) => run::rustdoc(arguments),
        Step::Architecture => architecture_holds(),
        Step::Readme => readme_holds(),
    }
}

fn architecture_holds() -> bool {
    println!("\n>>> architecture check");
    let breaches: Vec<String> = run::sources(&run::root(), &architecture::FOLDERS)
        .into_iter()
        .flat_map(architecture::breaches)
        .collect();
    breaches.iter().for_each(|breach| eprintln!("{breach}"));
    breaches.is_empty()
}

fn readme_holds() -> bool {
    println!("\n>>> README examples check");
    let breaches: Vec<String> = match run::text(Path::new("README.md")) {
        Ok(readme) => readme::examples(&readme)
            .into_iter()
            .filter_map(example_breach)
            .collect(),
        Err(reason) => vec![format!("The README could not be read: {reason}")],
    };
    breaches.iter().for_each(|breach| eprintln!("{breach}"));
    breaches.is_empty()
}

fn example_breach(example: Result<readme::Example, String>) -> Option<String> {
    match example {
        Ok(example) => readme::breach(&example, run::text(&example.file)),
        Err(breach) => Some(breach),
    }
}
