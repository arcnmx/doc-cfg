# doc-cfg

[![travis-badge][]][travis] [![release-badge][]][cargo] [![docs-badge][]][docs] [![license-badge][]][license]

The `#[doc_cfg(..)]` attribute is a convenience that removes the boilerplate
involved with using [`#[doc(cfg(..))]`](https://doc.rust-lang.org/unstable-book/language-features/doc-cfg.html)
in stable crates.

## Usage

Add the following to `Cargo.toml` to get started:

```toml
[dependencies]
doc-cfg = { version = "0.1" }

[features]
unstable-doc-cfg = []

[package.metadata.docs.rs]
features = ["unstable-doc-cfg"]
```

In your crate, use `#[doc_cfg(..)]` where you'd normally use `#[cfg(..)]`:

```rust
#![cfg_attr(feature = "unstable-doc-cfg", feature(doc_cfg))]

use doc_cfg::doc_cfg;

#[doc_cfg(windows)]
pub fn cool_nonportable_fn() { }
```

The name of the feature is important and should not be changed. Check out
[the full example for how to use it](http://arcnmx.github.io/doc-cfg/doc_cfg_example).
The `unstable-doc-cfg` feature should only be turned on when documenting,
`#[doc_cfg(..)]` is otherwise identical to `#[cfg(..)]` when built without it.

## [Documentation][docs]

See the [documentation][docs] and [example](http://arcnmx.github.io/doc-cfg/doc_cfg_example)
for up to date usage information.

[travis-badge]: https://img.shields.io/travis/arcnmx/doc-cfg/master.svg?style=flat-square
[travis]: https://travis-ci.org/arcnmx/doc-cfg
[release-badge]: https://img.shields.io/crates/v/doc-cfg.svg?style=flat-square
[cargo]: https://crates.io/crates/doc-cfg
[docs-badge]: https://img.shields.io/badge/API-docs-blue.svg?style=flat-square
[docs]: http://docs.rs/doc-cfg/
[license-badge]: https://img.shields.io/badge/license-MIT-ff69b4.svg?style=flat-square
[license]: https://github.com/arcnmx/doc-cfg/blob/master/COPYING
