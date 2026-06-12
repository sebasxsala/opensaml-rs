# rustsaml

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
- **`rustsaml`** — the same API under an alternate name.

All logic, features, and docs live in [`opensaml`](../opensaml).
