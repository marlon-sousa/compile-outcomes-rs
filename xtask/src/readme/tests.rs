use std::path::PathBuf;

use super::{Example, breach, examples};

#[test]
fn a_marked_code_block_is_an_example_of_its_file() {
    let readme = "Intro.\n\n<!-- example: tests/ui.rs -->\n```rust\nfn main() {}\n```\n\n```toml\nunmarked = true\n```\n";

    assert_eq!(
        examples(readme),
        [Ok(Example {
            file: PathBuf::from("tests/ui.rs"),
            block: "fn main() {}\n".to_owned(),
        })]
    );
}

#[test]
fn a_mark_without_a_whole_code_block_after_it_is_named() {
    let readmes = [
        "<!-- example: tests/ui.rs -->\nSome prose.\n",
        "<!-- example: tests/ui.rs -->\n```rust\nfn main() {}\n",
    ];

    for readme in readmes {
        assert_eq!(
            examples(readme),
            [Err(
                "The README marks tests/ui.rs as an example, but no code block follows the mark."
                    .to_owned()
            )]
        );
    }
}

#[test]
fn an_example_its_file_holds_as_written_is_no_breach_and_any_other_is() {
    let example = Example {
        file: PathBuf::from("tests/ui.rs"),
        block: "fn main() {}\n".to_owned(),
    };
    let cases = [
        (Ok("// The steps.\nfn main() {}\n".to_owned()), None),
        (Ok("fn main() {}\n".to_owned()), None),
        (
            Ok("pub fn main() {}\n".to_owned()),
            Some(
                "The README shows an example of tests/ui.rs that the file does not hold as written; copy the file's lines into the README again.",
            ),
        ),
        (
            Ok("fn main() { run(); }\n".to_owned()),
            Some(
                "The README shows an example of tests/ui.rs that the file does not hold as written; copy the file's lines into the README again.",
            ),
        ),
        (
            Err("No such file or directory".to_owned()),
            Some(
                "The README shows an example of tests/ui.rs, which could not be read: No such file or directory",
            ),
        ),
    ];

    for (text, expected) in cases {
        assert_eq!(breach(&example, text), expected.map(str::to_owned));
    }
}
