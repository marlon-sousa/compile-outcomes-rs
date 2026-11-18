//! Policy: the crate's README, which crates.io and docs.rs show, written from the root README without its `github-only` blocks and with its `site-only` blocks shown. A marker is a whole line, even inside a fenced code block.

use std::path::Path;

/// Where the crate's README is written, from the workspace's root.
pub(crate) const FILE: &str = "compile-outcomes/README.md";

/// The comment that starts the crate's README, which a reader of its source sees and crates.io and docs.rs hide.
const HEADER: &str = "<!-- Written by `cargo xtask readme --write` from the README at the repository's root. Edit that one, then run it again. -->\n\n";

const GITHUB_ONLY: &str = "<!-- github-only -->";
const END_GITHUB_ONLY: &str = "<!-- end github-only -->";
const SITE_ONLY: &str = "<!-- site-only";
const END_SITE_ONLY: &str = "-->";

/// The kind of block a line of the root README is in, with the number of the line that opened it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, strum::Display)]
enum Block {
    #[default]
    #[strum(to_string = "shared")]
    Shared,
    #[strum(to_string = "github-only")]
    GithubOnly(usize),
    #[strum(to_string = "site-only")]
    SiteOnly(usize),
}

impl Block {
    /// The block that `marker`, a line opening one, opens on line `number`.
    fn opened_by(marker: &str, number: usize) -> Self {
        if marker == GITHUB_ONLY {
            Self::GithubOnly(number)
        } else {
            Self::SiteOnly(number)
        }
    }

    /// The number of the line that opened the block, which a shared line has none of.
    fn opened(self) -> Option<usize> {
        match self {
            Self::Shared => None,
            Self::GithubOnly(opened) | Self::SiteOnly(opened) => Some(opened),
        }
    }
}

/// The crate's README so far, while the root README is read line by line.
#[derive(Debug, Default)]
struct Reading {
    block: Block,
    kept: String,
    problems: Vec<String>,
}

/// The crate's README that `readme` gives, or a sentence for each marker out of place.
pub(crate) fn from_root(readme: &str) -> Result<String, Vec<String>> {
    let reading = readme
        .lines()
        .enumerate()
        .fold(Reading::default(), read_line);
    let problems = [
        reading.problems,
        unclosed(reading.block),
        empty(&reading.kept),
    ]
    .concat();
    if problems.is_empty() {
        Ok(format!("{HEADER}{}", reading.kept))
    } else {
        Err(problems)
    }
}

/// A sentence saying the crate's README at `file` is not what the root README gives, unless it is.
pub(crate) fn breach(file: &Path, written: Result<String, String>, wanted: &str) -> Option<String> {
    let file = file.display();
    match written {
        Ok(written) if written == wanted => None,
        Ok(_) => Some(format!(
            "{file} is not what the root README gives; run cargo xtask readme --write."
        )),
        Err(reason) => Some(format!(
            "{file} could not be read: {reason}. Run cargo xtask readme --write."
        )),
    }
}

/// `reading` after the line numbered `at + 1` of the root README.
fn read_line(mut reading: Reading, (at, line): (usize, &str)) -> Reading {
    let number = at + 1;
    match (reading.block, line.trim()) {
        (Block::Shared, marker @ (GITHUB_ONLY | SITE_ONLY)) => {
            reading.block = Block::opened_by(marker, number);
        }
        (Block::GithubOnly(_), END_GITHUB_ONLY) | (Block::SiteOnly(_), END_SITE_ONLY) => {
            reading.block = Block::Shared;
        }
        (open, marker @ (GITHUB_ONLY | SITE_ONLY)) => reading.problems.push(format!(
            "README.md opens a {} block on line {number}, inside the {open} block opened on line {}.",
            Block::opened_by(marker, number),
            open.opened().unwrap_or_default()
        )),
        (_, END_GITHUB_ONLY) => reading.problems.push(format!(
            "README.md closes a github-only block on line {number}, where none is open."
        )),
        (_, END_SITE_ONLY) => reading.problems.push(format!(
            "README.md closes a site-only block on line {number}, where none is open."
        )),
        (Block::SiteOnly(_), _) if line.contains(END_SITE_ONLY) => reading.problems.push(format!(
            "README.md has --> on line {number}, inside a site-only block, which would end it early on GitHub."
        )),
        (Block::GithubOnly(_), _) => {}
        (_, _) => {
            reading.kept.push_str(line);
            reading.kept.push('\n');
        }
    }
    reading
}

fn unclosed(block: Block) -> Vec<String> {
    block
        .opened()
        .map(|opened| {
            format!("README.md never closes the {block} block it opens on line {opened}.")
        })
        .into_iter()
        .collect()
}

fn empty(kept: &str) -> Vec<String> {
    if kept.trim().is_empty() {
        vec!["README.md leaves nothing for the crate's README.".to_owned()]
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests;
