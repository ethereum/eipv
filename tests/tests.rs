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

#[test]
fn preamble_malformed_field() {
    test_fixture("preamble-malformed-field.md", "malformed key-value pair");
}

#[test]
fn preamble_malformed_eip() {
    test_fixture(
        "preamble-malformed-eip.md",
        "EIP should be an unsigned integer",
    );
    test_fixture(
        "preamble-malformed-eip-signed-int.md",
        "EIP should be an unsigned integer",
    );
}

#[test]
fn preamble_title_too_long() {
    test_fixture("preamble-title-too-long.md", "exceeds max length of 44");
}

#[test]
fn preamble_invalid_discussions_to() {
    test_fixture("preamble-invalid-discussions-to.md", "must be a URL");
}

#[test]
fn preamble_status_discussions_to() {
    test_fixture("preamble-status-draft.md", "draft: 1");
    test_fixture("preamble-status-last-call.md", "last_call: 1");
    test_fixture("preamble-status-accepted.md", "accepted: 1");
    test_fixture("preamble-status-final.md", "final: 1");
    test_fixture("preamble-status-abandoned.md", "abandoned: 1");
    test_fixture("preamble-status-rejected.md", "rejected: 1");
    test_fixture("preamble-status-superseded.md", "superseded: 1");
    test_fixture("preamble-status-invalid.md", "unknown status type: Invalid");
}
