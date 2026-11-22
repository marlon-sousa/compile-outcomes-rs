use super::{Clippy, Group, LintLevel, LintName, Lints, Setting};

fn group(level: LintLevel, group: Group) -> Setting {
    Setting {
        level,
        lints: Lints::Group(group),
    }
}

fn one(level: LintLevel, name: &str) -> Setting {
    Setting {
        level,
        lints: Lints::One(LintName::given(name.to_owned())),
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
