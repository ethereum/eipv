use assert_cmd::Command;
use predicates::prelude::{predicate::str::contains, PredicateBooleanExt};

pub fn test_fixture(f: &str, output: &str) {
    let path = vec!["tests/fixtures", f].join("/");
    Command::cargo_bin("eipv")
        .expect("eipv binary missing")
        .arg(path)
        .assert()
        .stdout(contains(output));
}

pub fn test_fixture_exclude_output(f: &str, not: &str) {
    let path = vec!["tests/fixtures", f].join("/");
    Command::cargo_bin("eipv")
        .expect("eipv binary missing")
        .arg(path)
        .assert()
        .stdout(contains(not).not());
}
