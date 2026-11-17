use std::path::PathBuf;

use super::{DefaultFeatures, Dependency, Source};

#[test]
fn a_dependency_by_path_names_its_folder() {
    let dependency = Dependency::path("greeting", "fixtures/greeting");

    assert_eq!(dependency.name(), "greeting");
    assert_eq!(
        dependency.source(),
        &Source::Path(PathBuf::from("fixtures/greeting"))
    );
}

#[test]
fn a_dependency_by_version_keeps_its_requirement() {
    let dependency = Dependency::version("serde", "1.0");

    assert_eq!(dependency.source(), &Source::Version("1.0".to_owned()));
}

#[test]
fn a_new_dependency_has_its_default_features_and_no_other() {
    let dependency = Dependency::version("serde", "1.0");

    assert!(dependency.enabled_features().is_empty());
    assert!(dependency.default_features().is_on());
}

#[test]
fn features_given_twice_are_all_enabled_in_order() {
    let dependency = Dependency::version("serde", "1.0")
        .features(["derive"])
        .features(["std", "rc"]);

    assert_eq!(*dependency.enabled_features(), ["derive", "std", "rc"]);
}

#[test]
fn default_features_can_be_turned_off() {
    let dependency = Dependency::version("serde", "1.0").without_default_features();

    assert_eq!(dependency.default_features(), DefaultFeatures::Off);
}
