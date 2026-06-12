# open-saml

**This crate is an official alias for [`opensaml`](https://crates.io/crates/opensaml).**
The SAML library lives in `opensaml`; this name is reserved defensively on
crates.io so unrelated third parties cannot squat it.

Full documentation: [repository README](https://github.com/sebasxsala/opensaml-rs#readme) · [docs.rs/opensaml](https://docs.rs/opensaml).

---

Thin re-export only:

```rust
pub use opensaml::*;
```

Use whichever name you prefer:

- **`opensaml`** — the real crate; depend on it directly (recommended).
- **`open-saml`** — the same API under a hyphenated alias.

## Disclaimer — no affiliation

This crate is **not** affiliated with, derived from, maintained by, endorsed
by, or sponsored by the Java
[OpenSAML](https://shibboleth.atlassian.net/wiki/spaces/OpenSAML/overview)
project, Shibboleth Consortium, or OASIS. It shares **no code** with OpenSAML.

All logic, features, and docs live in [`opensaml`](../opensaml). Any trademarks
belong to their respective owners.
