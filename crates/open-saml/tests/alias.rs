//! Verifies the `open-saml` crate re-exports the `opensaml` public API.

#[test]
fn reexports_constants_and_types() {
    assert_eq!(
        open_saml::constants::Binding::Redirect.urn(),
        "urn:oasis:names:tc:SAML:2.0:bindings:HTTP-Redirect"
    );
    let _setting = open_saml::EntitySetting::default();
    let _err = open_saml::OpenSamlError::UndefinedBinding;
}
