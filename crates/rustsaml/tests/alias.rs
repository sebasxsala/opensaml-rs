//! Verifies the `rustsaml` crate re-exports the `opensaml` public API.

#[test]
fn reexports_constants_and_types() {
    assert_eq!(
        rustsaml::constants::Binding::Redirect.urn(),
        "urn:oasis:names:tc:SAML:2.0:bindings:HTTP-Redirect"
    );
    let _setting = rustsaml::EntitySetting::default();
    let _err = rustsaml::OpenSamlError::UndefinedBinding;
}
