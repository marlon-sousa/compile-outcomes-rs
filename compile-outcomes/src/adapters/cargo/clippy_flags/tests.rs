use super::flags;
use crate::domain::{Clippy, Group};

#[test]
fn clippys_own_levels_pass_no_flag() {
    assert!(flags(&Clippy::new()).is_empty());
}

#[test]
fn each_setting_is_a_flag_and_its_lints_after_the_separator_in_order() {
    let clippy = Clippy::new()
        .warn(Group::Pedantic)
        .allow_lint("must_use_candidate")
        .deny_lint("clippy::unwrap_used")
        .allow(Group::Style)
        .deny(Group::Nursery)
        .warn_lint("let_and_return");

    assert_eq!(
        flags(&clippy),
        [
            "--",
            "-W",
            "clippy::pedantic",
            "-A",
            "clippy::must_use_candidate",
            "-D",
            "clippy::unwrap_used",
            "-A",
            "clippy::style",
            "-D",
            "clippy::nursery",
            "-W",
            "clippy::let_and_return",
        ]
    );
}
