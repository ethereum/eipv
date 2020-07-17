# EIP validator

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](https://github.com/lightclient/eipv)
[![ci status](https://github.com/lightclient/eipv/workflows/ci/badge.svg)](https://github.com/lightclient/eipv/actions)

An engine which ensures [Ethereum Improvement
Proposals](https://eips.ethereum.org) meet certain requirements.

## Getting Started

To install `eipv` and validate the EIPs repository:

```console
git clone https://github.com/lightclient/eipv.git
cargo install --path=eipv eipv
eipv /path/to/EIPS
```

## Requirements

This tracks what `eipv` can validate.

- [x] Preamble starts with `---`
- [x] Preamble ends with `---`
- [x] Preamble includes all required fields:
    - `eip`
    - `title`
    - `author`
    - `discussions-to`
    - `created`
    - `status`
    - `type`
    - `category` (iff `type` == "Standards Track")
- [x] Preamble does not include any unknown fields
- [ ] Preamble fields are properly formed:
    - [x] Each field is of the shape `{field}: {value}\n`
    - [x] `eip` is an unsigned integer
    - [x] `title` is a string whose length is less than 44 characters
    - [ ] `author` is a comma-separated string of author information which has
      three possible shapes:
        - Name only: `John A. Doe`
        - Name and email: `John A. Doe <john@doe.com>`
        - Name and Github username: `John A. Doe (@johndoe)`
    - [x] `discussions-to` is a URL where discussions regarding the EIP should be
      directed
    - [ ] `discussions-to` does not point to a PR
    - [x] `status` is one of the following string values:
        - `draft`
        - `last call`
        - `accepted`
        - `final`
        - `abandoned`
        - `rejected`
        - `superseded`
    - [ ] `type` is one of the following string values:
        - `standards track`
        - `informational`
        - `meta`
    - [ ] `category` is one of the following string values:
        - `core`
        - `networking`
        - `interface`
        - `erc`
    - [ ] `review-period-end` is a date value 
    - [ ] `created` is a date value 
    - [ ] `updated` is a comma-separated list of date values
    - [ ] `requires` is a comma-separated list of EIP numbers in ascending order
    - [ ] `superseded-by` is a comma-separated list of EIP numbers in ascending order
    - [ ] `replaces` is a comma-separated list of EIP numbers in ascending order
    - [ ] `resolution` is a URL
- [ ] EIP numbers listed as `required` exist
- [ ] EIP numbers listed as `superseded-by` exist and list the current EIP in
  its `replaces` field
- [ ] EIP numbers listed as `replaces` exist and list the current EIP in its
  `superseded-by` field
- [ ] The EIP body includes the required sections in the following order:
    - `Abstract`
    - `Motivation`
    - `Specification`
    - `Rationale`
    - `Backwards Compatibility`
    - `Test Cases`
    - `Implementations`
    - `Security Considerations`
    - `Copyright Waiver
- [ ] The `Abstract` section is no longer than 200 words
- [ ] The `Copyright Waiver` section contains only the following string:
  `Copyright and related rights waived via CC0.`
- [ ] The EIP body does not include any unclosed brackets or parentheses
  outside of code snippets
- [ ] File name is of form `eip-N.md`, where `N` coresponds to the EIP's assigned number
