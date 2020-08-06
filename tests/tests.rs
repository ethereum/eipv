mod utils;

use utils::{test_fixture, test_fixture_exclude_output};

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
fn preamble_status() {
    test_fixture("preamble-status-draft.md", "draft: 1");
    test_fixture("preamble-status-last-call.md", "last_call: 1");
    test_fixture("preamble-status-accepted.md", "accepted: 1");
    test_fixture("preamble-status-final.md", "final: 1");
    test_fixture("preamble-status-abandoned.md", "abandoned: 1");
    test_fixture("preamble-status-rejected.md", "rejected: 1");
    test_fixture("preamble-status-superseded.md", "superseded: 1");
    test_fixture("preamble-status-invalid.md", "unknown status type: Invalid");
}

#[test]
fn preamble_type() {
    // TODO: ensure type is *actually* represented properly
    test_fixture("preamble-type-standards.md", "draft: 1");
    test_fixture("preamble-type-informational.md", "draft: 1");
    test_fixture("preamble-type-meta.md", "draft: 1");
    test_fixture("preamble-type-invalid.md", "unknown type");
}

#[test]
fn preamble_category() {
    // TODO: ensure category is *actually* represented properly
    test_fixture("preamble-category-core.md", "draft: 1");
    test_fixture("preamble-category-networking.md", "draft: 1");
    test_fixture("preamble-category-interface.md", "draft: 1");
    test_fixture("preamble-category-erc.md", "draft: 1");
    test_fixture("preamble-category-invalid.md", "unknown category");
}

#[test]
fn preamble_review_period_end() {
    test_fixture(
        "preamble-review-period-end-malformed.md",
        "malformed review-period-end",
    );
}

#[test]
fn preamble_created() {
    test_fixture("preamble-created-malformed.md", "malformed created");
}

#[test]
fn preamble_updated() {
    test_fixture("preamble-updated-malformed.md", "malformed updated");
}

#[test]
fn preamble_requires() {
    test_fixture("preamble-requires-single.md", "draft: 1");
    test_fixture("preamble-requires-multiple.md", "draft: 1");
    test_fixture("preamble-requires-malformed.md", "malformed EIP number");
    test_fixture("preamble-requires-out-of-order.md", "ascending order");
    test_fixture(
        "preamble-requires-no-whitespace.md",
        "comma-separated values",
    );
    test_fixture(
        "preamble-requires-too-much-whitespace.md",
        "comma-separated values",
    );
}

#[test]
fn preamble_superseeded_by() {
    test_fixture("preamble-superseded-by-multiple.md", "draft: 1");
}

#[test]
fn preamble_replaces() {
    test_fixture("preamble-replaces-single.md", "draft: 1");
}

#[test]
fn preamble_resolution() {
    test_fixture("preamble-resolution-malformed.md", "must be a URL");
}

#[test]
fn preamble_author() {
    test_fixture(
        "preamble-author-email-unmatched.md",
        "unmatched email delimiter",
    );
    test_fixture(
        "preamble-author-handle-unmatched.md",
        "unmatched handle delimiter",
    );
    test_fixture(
        "preamble-author-email-and-handle.md",
        "can't include both an email and handle",
    );
    test_fixture(
        "preamble-author-email-trailing-info.md",
        "trailing information after email",
    );
    test_fixture(
        "preamble-author-handle-trailing-info.md",
        "trailing information after handle",
    );
}

#[test]
fn preamble_only_error_not_missing_field() {
    test_fixture_exclude_output("preamble-title-too-long.md", "missing");
}
