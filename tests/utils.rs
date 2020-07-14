use assert_cmd::Command;
use predicates::prelude::predicate::str::contains;

pub fn test_fixture(f: &str, output: &str) {
    let path = vec!["tests/fixtures", f].join("/");
    Command::cargo_bin("eipv")
        .expect("eipv binary missing")
        .arg(path)
        .assert()
        .stdout(contains(output));
}
