use super::Edition;

#[test]
fn an_edition_displays_as_a_manifest_writes_it() {
    let cases = [
        (Edition::E2015, "2015"),
        (Edition::E2018, "2018"),
        (Edition::E2021, "2021"),
        (Edition::E2024, "2024"),
    ];

    for (edition, written) in cases {
        assert_eq!(edition.to_string(), written);
    }
}
