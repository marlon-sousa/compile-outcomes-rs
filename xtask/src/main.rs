//! Handler: runs the quality gate, `cargo xtask <mode>`, as docs/WORKFLOW.md, section 7, lists its modes.

mod architecture;
mod crate_readme;
mod modes;
mod readme;
mod run;

use std::env;
use std::path::Path;
use std::process::ExitCode;

use modes::{CrateReadme, Mode, Step, requested, step};
use strum::VariantNames;

fn main() -> ExitCode {
    let arguments: Vec<String> = env::args().skip(1).collect();
    let words: Vec<&str> = arguments.iter().map(String::as_str).collect();
    let Some(request) = requested(&words) else {
        eprintln!(
            "The gate does not know {}. The modes are ci, {}, and readme --write writes the crate's README.",
            words.join(" "),
            Mode::VARIANTS.join(", ")
        );
        return ExitCode::FAILURE;
    };
    let mut failed = Vec::new();
    for mode in request.modes {
        if !passes(step(mode, request.crate_readme)) {
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
        Step::Readme(crate_readme) => match run::text(Path::new("README.md")).map_err(unreadable) {
            Ok(readme) => readme_holds(&readme) & crate_readme_holds(&readme, crate_readme),
            Err(breach) => {
                eprintln!("{breach}");
                false
            }
        },
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

fn readme_holds(readme: &str) -> bool {
    println!("\n>>> README examples check");
    let breaches: Vec<String> = readme::examples(readme)
        .into_iter()
        .filter_map(example_breach)
        .collect();
    breaches.iter().for_each(|breach| eprintln!("{breach}"));
    breaches.is_empty()
}

fn crate_readme_holds(readme: &str, crate_readme: CrateReadme) -> bool {
    println!("\n>>> crate README check");
    let file = Path::new(crate_readme::FILE);
    let breaches: Vec<String> = match (crate_readme::from_root(readme), crate_readme) {
        (Ok(wanted), CrateReadme::Write) => run::write(file, &wanted).err().into_iter().collect(),
        (Ok(wanted), CrateReadme::Check) => crate_readme::breach(file, run::text(file), &wanted)
            .into_iter()
            .collect(),
        (Err(problems), _) => problems,
    };
    breaches.iter().for_each(|breach| eprintln!("{breach}"));
    breaches.is_empty()
}

fn unreadable(reason: String) -> String {
    format!("The README could not be read: {reason}")
}

fn example_breach(example: Result<readme::Example, String>) -> Option<String> {
    match example {
        Ok(example) => readme::breach(&example, run::text(&example.file)),
        Err(breach) => Some(breach),
    }
}
