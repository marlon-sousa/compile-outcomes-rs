use super::SnippetName;

#[test]
fn a_name_is_valid_only_when_it_keeps_the_rules_of_a_name() {
    let cases = [
        ("missing_field", true),
        ("missing-field", true),
        ("Field2", true),
        ("_private", true),
        ("-dashed", true),
        ("2fields", false),
        ("", false),
        ("missing field", false),
        ("missing.field", false),
        ("champ_é", false),
    ];

    for (name, valid) in cases {
        assert_eq!(SnippetName::parse(name).is_some(), valid, "{name:?}");
    }
}

#[test]
fn a_crate_is_named_after_its_snippet_with_hyphens_as_underscores() {
    let cases = [
        ("missing_field", "snippet_missing_field"),
        ("missing-field", "snippet_missing_field"),
        ("-x", "snippet__x"),
    ];

    for (name, crate_name) in cases {
        let name = SnippetName::parse(name).unwrap();
        assert_eq!(name.crate_name(), crate_name);
    }
}

#[test]
fn a_name_displays_as_its_file_gives_it() {
    let name = SnippetName::parse("missing-field").unwrap();

    assert_eq!(name.to_string(), "missing-field");
    assert_eq!(name.as_str(), "missing-field");
}
