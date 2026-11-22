use super::LintName;

#[test]
fn a_lint_name_loses_one_leading_clippy_and_keeps_the_rest_as_given() {
    let cases = [
        ("unwrap_used", "unwrap_used"),
        ("clippy::unwrap_used", "unwrap_used"),
        ("clippy::clippy::x", "clippy::x"),
        ("", ""),
        ("clippy::", ""),
    ];
    for (given, kept) in cases {
        assert_eq!(
            LintName::given(given.to_owned()).to_string(),
            kept,
            "{given}"
        );
    }
}
