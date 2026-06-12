//! Verifies the `rust-saml` crate re-exports the `opensaml` public API.

#[test]
fn reexports_constants_and_types() {
    assert_eq!(
        rust_saml::constants::Binding::Redirect.urn(),
        "urn:oasis:names:tc:SAML:2.0:bindings:HTTP-Redirect"
    );
    let _setting = rust_saml::EntitySetting::default();
    let _err = rust_saml::OpenSamlError::UndefinedBinding;
}
