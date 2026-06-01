# Changelog

All notable changes to the opensaml-rs workspace are documented in this file.

The format is based on Keep a Changelog, and this project follows Semantic
Versioning while the API is still pre-1.0.

## Unreleased

### Added

- SAML 2.0 protocol layer to parity with npm `samlify` v2.10.2:
  - Constants (URNs, bindings, status codes, algorithms, NameID formats).
  - XML field extraction engine over `quick-xml` (`local-name()` XPath subset)
    with DOCTYPE hardening, plus the samlify field-sets.
  - Default message templates and tag substitution.
  - Service Provider and Identity Provider metadata parsing and generation.
  - HTTP-POST, HTTP-Redirect and HTTP-POST-SimpleSign message building.
  - `ServiceProvider`/`IdentityProvider` entities, login request/response
    creation and parsing, Single Logout, and the inbound `flow` orchestration
    (status, issuer and time validation).
- `crypto-bergshamra` feature delegating XML cryptography to
  `bergshamra` (**on by default**): key/certificate loading, XML-DSig signing
  and verification with anti-wrapping (XSW) protection, XML-Enc assertion
  encrypt/decrypt, and detached redirect/SimpleSign message signatures.
- `samlify` crate forwards the `crypto-bergshamra` feature.
- Customization to parity with samlify: a `User` subject with attributes wired
  into `IdentityProvider::create_login_response` (via `LoginResponseTemplate`),
  a `customTagReplacement` hook and custom message templates, `SignatureConfig`
  (signature prefix + placement), configurable `transformationAlgorithms`, a
  configurable encrypted-assertion tag prefix, and `SessionIndex` on logout.
- `Metadata::export_metadata` / `get_support_bindings` and `util::verify_fields`.
- Inline-certificate-vs-metadata mismatch is rejected with
  `OpenSamlError::UnmatchCertificate` (samlify rolling-cert safety).
- Conformance test suite ported 1:1 from samlify v2.10.2: all 131 active
  upstream cases (flow 64, index 47, issues 11, extractor 9) reproduced as 132
  Rust tests in `tests/{extractor,issues,index,flow}_conformance.rs`, with the
  upstream key/metadata fixtures. The crate runs 206 tests in total (89 without
  `crypto-bergshamra`); the whole suite passes.
- Security hardening: `<Audience>` restriction validation (`validate_audience`,
  on by default → `UnmatchAudience`), `InResponseTo` binding via
  `ServiceProvider::parse_login_response_with_request_id` (→ `InvalidInResponseTo`),
  metadata signature verification (`crypto::verify_metadata_signature` /
  `Metadata::verify_signature`), and XSW + robustness test suites
  (`tests/{xsw,hardening,robustness}.rs`). Schema validation remains pluggable
  via `context::set_schema_validator` on top of the always-on DOCTYPE rejection.

### Changed

- `crypto-bergshamra` is now enabled by default; disable with
  `default-features = false` for the crypto-free protocol layer (operations
  requiring signing, verification or encryption then fail closed with
  `OpenSamlError::Unsupported`).
- Reworked the crate from SP-only stubs to full SP + IdP.
- Signature verification tries each metadata-declared certificate individually,
  so signatures verify against any current key (rolling-certificate support).
- When encrypting, the IdP always signs the message *after* encryption (sound
  encrypt-then-sign); signing the message then encrypting a sub-element would
  invalidate the outer signature.

### Fixed

- Decryption strips a leading XML declaration from the recovered assertion so
  it can be re-parsed in place during the inbound flow.
