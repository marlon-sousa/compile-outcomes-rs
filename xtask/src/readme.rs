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
    (0..lines.len())
        .filter_map(|at| marked(&lines, at))
        .map(example)
        .collect()
}

/// A sentence saying the file of `example` does not hold it, given the file's text or the reason it could not be read.
pub(crate) fn breach(example: &Example, text: Result<String, String>) -> Option<String> {
    let file = example.file.display();
    match text {
        Ok(text) if holds(&text, &example.block) => None,
        Ok(_) => Some(format!(
            "The README shows an example of {file} that the file does not hold as written; copy the file's lines into the README again."
        )),
        Err(reason) => Some(format!(
            "The README shows an example of {file}, which could not be read: {reason}"
        )),
    }
}

/// Whether `text` holds `block` as whole lines, from the start of one of them.
fn holds(text: &str, block: &str) -> bool {
    text.starts_with(block) || text.contains(&format!("\n{block}"))
}

/// The file the line at `at` marks, and the lines after it, if that line is a mark.
fn marked<'a>(lines: &'a [&'a str], at: usize) -> Option<(&'a str, &'a [&'a str])> {
    let file = lines.get(at)?.strip_prefix(MARKER)?.strip_suffix(" -->")?;
    Some((file, lines.get(at + 1..)?))
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
