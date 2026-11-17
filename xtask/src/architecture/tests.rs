use std::path::PathBuf;

use super::{Source, breaches};

fn source(path: &str, text: &str) -> Source {
    Source {
        path: PathBuf::from(path),
        text: Ok(text.to_owned()),
    }
}

#[test]
fn input_or_output_in_a_pure_module_is_a_breach_with_its_line() {
    let cases = [
        (
            "use std::path::Path;\nuse std::fs::read;\n",
            "line 2: names std::fs",
        ),
        ("use std::{fs, path::Path};\n", "line 1: names std::fs"),
        (
            "use std::{path::Path, io::Read};\n",
            "line 1: names std::io",
        ),
        ("use std::os::unix::fs::symlink;\n", "line 1: names std::os"),
        ("use toml;\n", "line 1: names toml"),
        (
            "fn f() {\n    std::env::var(\"X\");\n}\n",
            "line 2: names std::env",
        ),
        (
            "fn f() {\n    let _ = cargo_metadata::Message::parse_stream;\n}\n",
            "line 2: names cargo_metadata",
        ),
        (
            "fn f(p: &std::path::Path) -> bool {\n    p.exists()\n}\n",
            "line 2: calls exists",
        ),
        (
            "fn f(p: &Path) -> bool {\n    Path::exists(p)\n}\n",
            "line 2: calls exists",
        ),
        (
            "fn f() {\n    let _ = format!(\"{:?}\", std::env::var(\"X\"));\n}\n",
            "line 2: names std::env",
        ),
        (
            "fn f() {\n    let _ = vec![std::fs::read(\"x\")];\n}\n",
            "line 2: names std::fs",
        ),
        (
            "fn f(p: &Path) {\n    assert!(p.exists());\n}\n",
            "line 2: calls exists",
        ),
        ("fn f() {\n    println!(\"out\");\n}\n", "line 2: prints"),
        ("use std as s;\n", "line 1: imports std whole"),
        ("use std::*;\n", "line 1: imports std whole"),
    ];

    for (text, breach) in cases {
        let found = breaches(source("compile-outcomes/src/domain/snippet.rs", text));
        assert_eq!(found.len(), 1, "{text}: {found:?}");
        assert!(
            found[0].starts_with(&format!("compile-outcomes/src/domain/snippet.rs: {breach}")),
            "{text}: {found:?}"
        );
    }
}

#[test]
fn input_or_output_named_in_a_comment_or_a_string_is_not_a_breach() {
    let text =
        "//! Reads nothing with std::fs.\n/// Nor std::process.\nconst S: &str = \"std::fs\";\n";

    assert!(breaches(source("compile-outcomes/src/use_cases/naming.rs", text)).is_empty());
}

#[test]
fn only_the_domain_and_use_cases_modules_are_pure() {
    let text = "use std::fs;\n";
    let cases = [
        ("compile-outcomes/src/domain.rs", true),
        ("compile-outcomes/src/domain/errors/gathered.rs", true),
        ("compile-outcomes/src/use_cases.rs", true),
        ("compile-outcomes/src/use_cases/naming/tests.rs", true),
        ("compile-outcomes/src/adapters/disk.rs", false),
        ("compile-outcomes/src/lib.rs", false),
        ("xtask/src/domain.rs", false),
    ];

    for (path, pure) in cases {
        assert_eq!(!breaches(source(path, text)).is_empty(), pure, "{path}");
    }
}

#[test]
fn a_mod_rs_and_a_long_file_are_both_reported() {
    let long = "\n".repeat(201);

    assert_eq!(
        breaches(source("xtask/src/steps/mod.rs", &long)),
        [
            "xtask/src/steps/mod.rs: it is a mod.rs; name the file after its module instead.",
            "xtask/src/steps/mod.rs: it has 201 lines, more than 200; split it along its seams.",
        ]
    );
}

#[test]
fn a_file_of_200_lines_not_named_mod_rs_breaks_nothing() {
    assert!(breaches(source("xtask/src/steps.rs", &"\n".repeat(200))).is_empty());
}

#[test]
fn a_file_that_could_not_be_read_is_a_breach_with_the_reason() {
    let unreadable = Source {
        path: PathBuf::from("xtask/src"),
        text: Err("No such file or directory (os error 2)".to_owned()),
    };

    assert_eq!(
        breaches(unreadable),
        ["xtask/src could not be read: No such file or directory (os error 2)"]
    );
}
