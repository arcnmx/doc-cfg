//! This example shows how to use the [`#[doc_cfg(..)]`](https://docs.rs/doc-cfg/)
//! attribute.
//!
//! Run `cargo doc --features unstable-doc-cfg --open` to generate. It seems to
//! work on stable rustc as of 1.32 but this is probably an unintended bug and
//! you shouldn't count on the `unstable-doc-cfg` feature working anywhere but
//! with a nightly toolchain.
//!
//! Check the [source code of this example](../src/doc_cfg_example/lib.rs.html#12)
//! to see how it all works, or [clone it from github](https://github.com/arcnmx/doc-cfg/tree/master/example)
//! and try it yourself.

#![cfg_attr(feature = "unstable-doc-cfg", feature(doc_cfg))]

// bring the attribute into scope
use doc_cfg::doc_cfg;

/// Mark something as only available on fancy platforms!
#[doc_cfg(all(nintendo64, target_feature = "sse"))] // this will add both a #[cfg(..)] and a #[doc(cfg(..))]
pub fn cool_nonportable_fn() { wont_compile }

/// You can also display a message that differs from the actual `#[cfg(..)]` conditional.
#[doc_cfg(feature = "dependency")] // applies #[cfg(feature = "dependency")]
#[doc(cfg(feature = "my-fancy-feature"))] // the #[doc(cfg(..))] suggestion presented by rustdoc
pub fn feature_gated_fn() { wont_compile }

/// A plain old normal function.
pub fn boring_old_fn() { }
