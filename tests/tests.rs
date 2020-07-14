mod utils;

use utils::test_fixture;

#[test]
fn valid() {
    test_fixture("valid.md", "valid: 1");
}

#[test]
fn preamble_start_malformed() {
    test_fixture("preamble-start-malformed.md", "missing initial");
}

#[test]
fn preamble_end_malformed() {
    test_fixture("preamble-end-malformed.md", "missing trailing");
}

#[test]
fn preamble_has_required_fields() {
    test_fixture("preamble-missing-eip.md", "missing eip");
    test_fixture("preamble-missing-title.md", "missing title");
    test_fixture("preamble-missing-author.md", "missing author");
    test_fixture(
        "preamble-missing-discussions-to.md",
        "missing discussions-to",
    );
    test_fixture("preamble-missing-status.md", "missing status");
    test_fixture("preamble-missing-type.md", "missing type");
    test_fixture("preamble-missing-category.md", "missing category");
    test_fixture("preamble-missing-category-ok.md", "valid: 1");
}

#[test]
fn preamble_unknown_field() {
    test_fixture("preamble-unknown-field.md", "unknown preamble key: unknown");
}
