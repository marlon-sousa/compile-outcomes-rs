use super::Group;

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
