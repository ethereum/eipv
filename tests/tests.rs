mod utils;

use utils::{
    test_fixture, test_fixture_exclude_output, test_fixture_valid, test_fixture_valid_custom,
};

#[test]
fn valid() {
    test_fixture_valid("valid.md");
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
    test_fixture("preamble-missing-eip.md", "missing EIP");
    test_fixture("preamble-missing-title.md", "missing title");
    test_fixture("preamble-missing-author.md", "missing author");
    test_fixture(
        "preamble-missing-discussions-to.md",
        "missing discussions-to",
    );
    test_fixture("preamble-missing-status.md", "missing status");
    test_fixture("preamble-missing-type.md", "missing type");
    test_fixture("preamble-missing-category.md", "missing category");
    test_fixture_valid("preamble-missing-category-ok.md");
}

#[test]
fn preamble_unknown_field() {
    test_fixture("preamble-unknown-field.md", "unknown preamble field");
}

#[test]
fn preamble_malformed_field() {
    test_fixture("preamble-malformed-field.md", "malformed field");
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
    test_fixture("preamble-title-too-long.md", "exceeds max length");
}

#[test]
fn preamble_invalid_discussions_to() {
    test_fixture("preamble-invalid-discussions-to.md", "must be a URL");
}

#[test]
fn preamble_status() {
    test_fixture_valid_custom("preamble-status-draft.md", "draft: 1");
    test_fixture_valid_custom("preamble-status-review.md", "review: 1");
    test_fixture_valid_custom("preamble-status-last-call.md", "last_call: 1");
    test_fixture_valid_custom("preamble-status-final.md", "final: 1");
    test_fixture_valid_custom("preamble-status-stagnant.md", "stagnant: 1");
    test_fixture_valid_custom("preamble-status-withdrawn.md", "withdrawn: 1");
    test_fixture_valid_custom("preamble-status-living.md", "living: 1");
    test_fixture("preamble-status-invalid.md", "unknown status");
}

#[test]
fn preamble_type() {
    // TODO: ensure type is *actually* represented properly
    test_fixture_valid("preamble-type-standards.md");
    test_fixture_valid("preamble-type-informational.md");
    test_fixture_valid("preamble-type-meta.md");
    test_fixture("preamble-type-invalid.md", "unknown type");
}

#[test]
fn preamble_category() {
    // TODO: ensure category is *actually* represented properly
    test_fixture_valid("preamble-category-core.md");
    test_fixture_valid("preamble-category-networking.md");
    test_fixture_valid("preamble-category-interface.md");
    test_fixture_valid("preamble-category-erc.md");
    test_fixture("preamble-category-invalid.md", "unknown category");
}

#[test]
fn preamble_last_call_deadline() {
    test_fixture(
        "preamble-last-call-deadline-malformed.md",
        "malformed last-call-deadline",
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
    test_fixture_valid("preamble-requires-single.md");
    test_fixture_valid("preamble-requires-multiple.md");
    test_fixture(
        "preamble-requires-malformed.md",
        "EIP should be an unsigned integer",
    );
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
    test_fixture("preamble-author-email-invalid.md", "malformed email");
    test_fixture("preamble-author-handle-invalid.md", "malformed handle");
    test_fixture(
        "preamble-author-no-contact.md",
        "author has no contact details",
    );
}

#[test]
fn preamble_only_error_not_missing_field() {
    test_fixture_exclude_output("preamble-title-too-long.md", "missing");
}

#[test]
fn preamble_description_too_long() {
    test_fixture(
        "preamble-description-too-long.md",
        "description exceeds max length",
    );
}
