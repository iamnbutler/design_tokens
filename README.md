# design_tokens &emsp; [![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/design_tokens.svg
[crates.io]: https://crates.io/crates/design_tokens

**Utilities for working with design tokens in Rust, implementing the [Design Tokens Format Module](https://design-tokens.github.io/community-group/format/).**

This initial version simply provides structs for representing design tokens in Rust.

In the near future, this crate will provide utilities for importing and validating design tokens from JSON.

---

You may be looking for:

- [Design Tokens Format Module](https://design-tokens.github.io/community-group/format/)
- [API documentation](https://docs.rs/design_tokens)
- [Repository](https://github.com/iamnbutler/design_tokens)

## Examples

```toml
[dependencies]

design_tokens = "0.1.0"

# Optional: Preserve key order using `indexmap`.
design_tokens = { version = "0.1.0", features = ["ordered"] }

```

## Features

- **ordered**: If you want to preserve insertion order of elements, you can use the `ordered` feature which leverages the `indexmap` crate.
