use std::path::Path;

use super::{HEADER, breach, from_root};

#[test]
fn a_github_only_block_is_left_out_and_a_site_only_block_shown() {
    let readme = "# Title\n\n<!-- github-only -->\nFor contributors.\n<!-- end github-only -->\n<!-- site-only\nFor users.\n-->\nShared.\n";

    assert_eq!(
        from_root(readme),
        Ok(format!("{HEADER}# Title\n\nFor users.\nShared.\n"))
    );
}

#[test]
fn every_marker_out_of_place_is_named() {
    let readme = "Shared.\n<!-- end github-only -->\n-->\n<!-- github-only -->\n<!-- site-only\n";

    assert_eq!(
        from_root(readme),
        Err(vec![
            "README.md closes a github-only block on line 2, where none is open.".to_owned(),
            "README.md closes a site-only block on line 3, where none is open.".to_owned(),
            "README.md opens a site-only block on line 5, inside the github-only block opened on line 4.".to_owned(),
            "README.md never closes the github-only block it opens on line 4.".to_owned(),
        ])
    );
}

#[test]
fn awkward_readmes_give_what_a_reader_expects() {
    let cases = [
        (
            "line endings of Windows",
            "Shared.\r\n<!-- github-only -->\r\nHidden.\r\n<!-- end github-only -->\r\n",
            Ok(format!("{HEADER}Shared.\n")),
        ),
        (
            "indented markers",
            "Shared.\n  <!-- site-only\nShown.\n  -->\n",
            Ok(format!("{HEADER}Shared.\nShown.\n")),
        ),
        (
            "an empty README",
            "",
            Err(vec!["README.md leaves nothing for the crate's README.".to_owned()]),
        ),
        (
            "a README all github-only",
            "<!-- github-only -->\nHidden.\n<!-- end github-only -->\n",
            Err(vec!["README.md leaves nothing for the crate's README.".to_owned()]),
        ),
        (
            "a github-only block never closed",
            "Shared.\n<!-- github-only -->\nHidden.\n",
            Err(vec!["README.md never closes the github-only block it opens on line 2.".to_owned()]),
        ),
        (
            "a site-only block never closed",
            "Shared.\n<!-- site-only\nShown.\n",
            Err(vec!["README.md never closes the site-only block it opens on line 2.".to_owned()]),
        ),
        (
            "--> inside a site-only block",
            "Shared.\n<!-- site-only\nSee <!-- a note --> here.\n-->\n",
            Err(vec!["README.md ends a comment on line 3, inside a site-only block, which would end the block early on GitHub.".to_owned()]),
        ),
    ];

    cases.into_iter().for_each(|(case, readme, wanted)| {
        assert_eq!(from_root(readme), wanted, "{case}");
    });
}

#[test]
fn a_crate_readme_that_is_not_what_the_root_gives_is_named() {
    let file = Path::new("compile-outcomes/README.md");

    assert_eq!(breach(file, Ok("same".to_owned()), "same"), None);
    assert_eq!(
        breach(file, Ok("old".to_owned()), "new"),
        Some(
            "compile-outcomes/README.md is not what the root README gives; run cargo xtask readme --write."
                .to_owned()
        )
    );
    assert_eq!(
        breach(file, Err("not found".to_owned()), "new"),
        Some(
            "compile-outcomes/README.md could not be read: not found. Run cargo xtask readme --write."
                .to_owned()
        )
    );
}
