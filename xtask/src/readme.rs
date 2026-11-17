//! Policy: the README's examples, each a code block marked with the file of a crate the gate builds, which must hold the block as written.

use std::path::PathBuf;

/// What starts the line before a marked code block, followed by the file's path from the workspace's root and ` -->`.
const MARKER: &str = "<!-- example: ";

/// A marked code block of the README, and the file it says holds it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Example {
    pub(crate) file: PathBuf,
    pub(crate) block: String,
}

/// Every marked example of `readme`, in order, or a sentence for a mark that no code block follows.
pub(crate) fn examples(readme: &str) -> Vec<Result<Example, String>> {
    let lines: Vec<&str> = readme.lines().collect();
    lines
        .iter()
        .enumerate()
        .filter_map(|(at, line)| Some((marked_file(line)?, lines.get(at + 1..)?)))
        .map(example)
        .collect()
}

/// A sentence saying the file of `example` does not hold it, given the file's text or the reason it could not be read.
pub(crate) fn breach(example: &Example, text: Result<String, String>) -> Option<String> {
    let file = example.file.display();
    match text {
        Ok(text) if text.contains(&example.block) => None,
        Ok(_) => Some(format!(
            "The README shows an example of {file} that the file does not hold as written; copy the file's lines into the README again."
        )),
        Err(reason) => Some(format!(
            "The README shows an example of {file}, which could not be read: {reason}"
        )),
    }
}

fn marked_file(line: &str) -> Option<&str> {
    line.strip_prefix(MARKER)?.strip_suffix(" -->")
}

/// The code block that opens on the first of `after`, the lines that follow the mark of `file`.
fn example((file, after): (&str, &[&str])) -> Result<Example, String> {
    let opens = after.first().is_some_and(|line| line.starts_with("```"));
    let body = after.get(1..).unwrap_or_default();
    match (opens, body.iter().position(|line| *line == "```")) {
        (true, Some(end)) => Ok(Example {
            file: PathBuf::from(file),
            block: body
                .get(..end)
                .unwrap_or_default()
                .iter()
                .map(line)
                .collect(),
        }),
        _ => Err(format!(
            "The README marks {file} as an example, but no code block follows the mark."
        )),
    }
}

fn line(text: &&str) -> String {
    format!("{text}\n")
}

#[cfg(test)]
mod tests;
