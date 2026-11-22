use super::{Clippy, Group, LintLevel, Lints, Setting};

fn group(level: LintLevel, group: Group) -> Setting {
    Setting {
        level,
        lints: Lints::Group(group),
    }
}

fn one(level: LintLevel, name: &str) -> Setting {
    Setting {
        level,
        lints: Lints::One(name.to_owned()),
    }
}

#[test]
fn a_new_clippy_keeps_clippys_own_levels() {
    assert!(Clippy::new().settings().is_empty());
}

#[test]
fn levels_keep_the_order_they_were_set() {
    let clippy = Clippy::new()
        .warn(Group::Pedantic)
        .allow_lint("must_use_candidate")
        .deny(Group::Correctness)
        .warn_lint("unwrap_used")
        .allow(Group::Style)
        .deny_lint("let_and_return");

    assert_eq!(
        *clippy.settings(),
        [
            group(LintLevel::Warn, Group::Pedantic),
            one(LintLevel::Allow, "must_use_candidate"),
            group(LintLevel::Deny, Group::Correctness),
            one(LintLevel::Warn, "unwrap_used"),
            group(LintLevel::Allow, Group::Style),
            one(LintLevel::Deny, "let_and_return"),
        ]
    );
}

#[test]
fn a_lint_is_named_alike_with_or_without_clippy_before_it() {
    let cases = [
        ("unwrap_used", "unwrap_used"),
        ("clippy::unwrap_used", "unwrap_used"),
        ("clippy::clippy::x", "clippy::x"),
        ("", ""),
        ("clippy::", ""),
    ];
    for (given, kept) in cases {
        let clippy = Clippy::new().deny_lint(given);
        assert_eq!(*clippy.settings(), [one(LintLevel::Deny, kept)], "{given}");
    }
}

#[test]
fn a_group_displays_as_clippy_names_it() {
    let cases = [
        (Group::All, "all"),
        (Group::Correctness, "correctness"),
        (Group::Suspicious, "suspicious"),
        (Group::Style, "style"),
        (Group::Complexity, "complexity"),
        (Group::Perf, "perf"),
        (Group::Pedantic, "pedantic"),
        (Group::Restriction, "restriction"),
        (Group::Nursery, "nursery"),
        (Group::Cargo, "cargo"),
    ];
    for (group, name) in cases {
        assert_eq!(group.to_string(), name);
    }
}
