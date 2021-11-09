use assert_cmd::Command;
use predicates::prelude::{predicate::str::contains, PredicateBooleanExt};

pub fn test_fixture(f: &str, output: &str) {
    let path = vec!["tests/fixtures", f].join("/");
    Command::cargo_bin("eipv")
        .expect("eipv binary missing")
        .arg(path)
        .assert()
        .stdout(contains(output))
        .stdout(contains(f).count(1));
}

pub fn test_fixture_exclude_output(f: &str, not: &str) {
    let path = vec!["tests/fixtures", f].join("/");
    Command::cargo_bin("eipv")
        .expect("eipv binary missing")
        .arg(path)
        .assert()
        .stdout(contains(not).not());
}

pub fn test_fixture_valid(f: &str) {
    let path = vec!["tests/fixtures", f].join("/");
    Command::cargo_bin("eipv")
        .expect("eipv binary missing")
        .arg(path)
        .assert()
        .stdout(contains("valid: 1, invalid: 0"));
}

pub fn test_fixture_valid_custom(f: &str, output: &str) {
    let path = vec!["tests/fixtures", f].join("/");
    Command::cargo_bin("eipv")
        .expect("eipv binary missing")
        .arg(path)
        .assert()
        .stdout(contains("valid: 1, invalid: 0"))
        .stdout(contains(output));
}
