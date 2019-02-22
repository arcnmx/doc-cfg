//#![deny(missing_docs)] // https://github.com/rust-lang/rust/issues/42008
#![doc(html_root_url = "http://docs.rs/doc-cfg/0.1.0")]

//! The [`doc_cfg`] attribute is a convenience that removes the boilerplate
//! involved with using [`#[doc(cfg(..))]`](https://doc.rust-lang.org/unstable-book/language-features/doc-cfg.html)
//! in stable crates.
//!
//! Add the following to `Cargo.toml` to get started:
//!
//! ```toml,ignore
//! [dependencies]
//! doc-cfg = { version = "0.1" }
//!
//! [features]
//! unstable-doc-cfg = []
//!
//! [package.metadata.docs.rs]
//! features = ["unstable-doc-cfg"]
//! ```
//!
//! The name of the feature is important and should not be changed. Check out
//! [the full example for how to use it](http://arcnmx.github.io/doc-cfg/doc_cfg_example).
//!
//! The `unstable-doc-cfg` feature should only be turned on when documenting, the
//! `#[doc_cfg(..)]` attribute is otherwise identical to `#[cfg(..)]` when built
//! without it.

extern crate proc_macro;

use std::iter::FromIterator;
use proc_macro2::{TokenStream, TokenTree, Delimiter, Ident, Spacing};
use quote::quote;

/// The `#[doc_cfg(..)]` attribute works much like `#[cfg(..)]`, but it allows
/// the item being documented to show up in the crate documentation when built
/// on a platform or configuration that doesn't match the predicate.
///
/// It can be used like so:
///
/// ```no_run
/// #![cfg_attr(feature = "unstable-doc-cfg", feature(doc_cfg))]
///
/// use doc_cfg::doc_cfg;
///
/// #[doc_cfg(windows)]
/// pub fn cool_nonportable_fn() { }
/// ```
///
/// Check out [the full example to see how it looks](http://arcnmx.github.io/doc-cfg/doc_cfg_example).
///
/// ## `#[doc(cfg(..))]`
///
/// In cases where the predicate contains irrelevant implementation details or
/// private dependency names you can specify an alternate condition to be
/// included in the documentation:
///
/// ```no_run
/// #![cfg_attr(feature = "unstable-doc-cfg", feature(doc_cfg))]
///
/// use doc_cfg::doc_cfg;
///
/// #[doc_cfg(feature = "__private_feature")]
/// #[doc(cfg(feature = "cargo-feature-flag"))]
/// pub fn cool_fn() { }
/// ```
#[proc_macro_attribute]
pub fn doc_cfg(attr: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    doc_cfg_(attr.into(), input.into(), needs_cfg_attr()).into()
}

fn needs_cfg_attr() -> bool {
    // TODO: when doc_cfg is stable, use rustc_version to decide whether to return false here
    true
}

fn doc_cfg_(attr: TokenStream, input: TokenStream, needs_cfg_attr: bool) -> TokenStream {
    if attr.clone().into_iter().next().is_none() {
        panic!("#[doc_cfg(..)] conditional missing");
    }

    let parsed = parse_item(input);

    let cfg = if needs_cfg_attr {
        quote! {
            #[cfg_attr(feature = "unstable-doc-cfg", cfg(any(#attr, rustdoc)))]
            #[cfg_attr(not(feature = "unstable-doc-cfg"), cfg(#attr))]
        }
    } else {
        quote! {
            #[cfg(any(#attr, rustdoc))]
        }
    };

    let doc_cfg = if parsed.doc_cfg.is_empty() {
        vec![quote! { #[doc(cfg(#attr))] }]
    } else {
        parsed.doc_cfg
    };

    let doc_cfg = doc_cfg.into_iter()
        .map(|doc_cfg| parse_cfg(doc_cfg).expect("internal doc_cfg parse error"))
        .map(|doc_cfg| if needs_cfg_attr {
            quote! {
                #[cfg_attr(feature = "unstable-doc-cfg", doc(cfg(#doc_cfg)))]
            }
        } else {
            quote! {
                #[doc(cfg(#doc_cfg))]
            }
        }).collect::<TokenStream>();

    let body = parsed.body;

    quote! {
        #doc_cfg
        #cfg
        #body
    }
}

fn parse_cfg_fn<I: IntoIterator<Item=TokenTree>>(cfg: I) -> Option<(Ident, TokenStream)> {
    let mut cfg = cfg.into_iter();

    if let TokenTree::Ident(id) = cfg.next()? {
        if let TokenTree::Group(group) = cfg.next()? {
            if group.delimiter() == Delimiter::Parenthesis && cfg.next().is_none() {
                return Some((id, group.stream()))
            }
        }
    }

    None
}

/// Extracts the inner expression from a #[doc(cfg(..))] attribute
fn parse_cfg<I: IntoIterator<Item=TokenTree>>(cfg: I) -> Option<TokenStream> {
    let mut cfg = cfg.into_iter();

    // Skip leading #
    let token = if let TokenTree::Punct(ref punct) = cfg.next()? {
        if punct.as_char() == '#' && punct.spacing() == Spacing::Alone {
            cfg.next()
        } else {
            None
        }
    } else {
        None
    }?;

    let group = if let TokenTree::Group(group) = token {
        Some(group)
    } else {
        None
    }?.stream();

    let (id, stream) = parse_cfg_fn(group)?;

    let (id, stream) = if &id == "doc" {
        parse_cfg_fn(stream)
    } else {
        None
    }?;

    if &id == "cfg" && cfg.next().is_none() {
        Some(stream)
    } else {
        None
    }
}

struct DocCfg {
    doc_cfg: Vec<TokenStream>,
    body: TokenStream,
}

fn parse_item(input: TokenStream) -> DocCfg {
    let mut doc_cfg_attrs = Vec::new();
    let mut output = Vec::new();
    let mut tokens = input.into_iter();

    let mut peek = tokens.next();
    while let Some(token) = peek.take() {
        peek = tokens.next();

        let is_doc_cfg = match (&token, &peek) {
            (TokenTree::Punct(ref punct), Some(TokenTree::Group(ref g)))
                if punct.as_char() == '#' => parse_cfg(vec![TokenTree::from(punct.clone()), g.clone().into()]).is_some(),
            _ => false,
        };

        if is_doc_cfg {
            if let Some(group) = peek.take() {
                doc_cfg_attrs.push(TokenStream::from_iter(vec![token, group]));
                peek = tokens.next();
            } else {
                unreachable!()
            }
        } else {
            output.push(token);
        }
    }

    DocCfg {
        doc_cfg: doc_cfg_attrs,
        body: TokenStream::from_iter(output),
    }
}

#[cfg(test)]
mod test {
    use quote::quote;
    use proc_macro2::{TokenStream, TokenTree};

    #[test]
    fn basic() {
        test(quote! {
            #[inline]
            fn test() { }
        }, quote! {
            #[cfg_attr(feature = "unstable-doc-cfg", doc(cfg(feature = "something")))]
            #[cfg_attr(feature = "unstable-doc-cfg", cfg(any(feature = "something", rustdoc)))]
            #[cfg_attr(not(feature = "unstable-doc-cfg"), cfg(feature = "something"))]
            #[inline]
            fn test() { }
        }, quote! {
            #[doc(cfg(feature = "something"))]
            #[cfg(any(feature = "something", rustdoc))]
            #[inline]
            fn test() { }
        }, quote! { feature = "something" });
    }

    #[test]
    fn custom_doc_cfg() {
        test(quote! {
            #[doc(cfg(feature = "somethingelse"))]
            fn test() { }
        }, quote! {
            #[cfg_attr(feature = "unstable-doc-cfg", doc(cfg(feature = "somethingelse")))]
            #[cfg_attr(feature = "unstable-doc-cfg", cfg(any(feature = "something", rustdoc)))]
            #[cfg_attr(not(feature = "unstable-doc-cfg"), cfg(feature = "something"))]
            fn test() { }
        }, quote! {
            #[doc(cfg(feature = "somethingelse"))]
            #[cfg(any(feature = "something", rustdoc))]
            fn test() { }
        }, quote! { feature = "something" });
    }

    #[test]
    fn multiple() {
        test(quote! {
            #[doc(cfg(feature = "something"))]
            #[inline]
            #[doc(cfg(feature = "somethingelse"))]
            fn test() { }
        }, quote! {
            #[cfg_attr(feature = "unstable-doc-cfg", doc(cfg(feature = "something")))]
            #[cfg_attr(feature = "unstable-doc-cfg", doc(cfg(feature = "somethingelse")))]
            #[cfg_attr(feature = "unstable-doc-cfg", cfg(any(all(feature = "something", feature = "somethingelse"), rustdoc)))]
            #[cfg_attr(not(feature = "unstable-doc-cfg"), cfg(all(feature = "something", feature = "somethingelse")))]
            #[inline]
            fn test() { }
        }, quote! {
            #[doc(cfg(feature = "something"))]
            #[doc(cfg(feature = "somethingelse"))]
            #[cfg(any(all(feature = "something", feature = "somethingelse"), rustdoc))]
            #[inline]
            fn test() { }
        }, quote! { all(feature = "something", feature = "somethingelse") });
    }

    #[test]
    #[should_panic]
    fn cfg_missing() {
        let _ = super::doc_cfg_(TokenStream::new(), quote! {
            fn test() { }
        }, true);
    }

    fn test(original: TokenStream, expected: TokenStream, expected_no_cfg_attr: TokenStream, attr: TokenStream) {
        let output = TokenStream::from(super::doc_cfg_(attr.clone(), original.clone(), true));
        compare(output, expected);

        let output = TokenStream::from(super::doc_cfg_(attr, original, false));
        compare(output, expected_no_cfg_attr);
    }

    fn compare(output: TokenStream, expected: TokenStream) {
        if !stream_eq(output.clone(), expected.clone()) {
            panic!("macro output mismatch\nexpected: {}\ngot     : {}", expected, output);
        }
    }

    fn stream_eq(lhs: TokenStream, rhs: TokenStream) -> bool {
        for (lhs, rhs) in lhs.into_iter().zip(rhs) {
            if !token_eq(&lhs, &rhs) {
                return false
            }
        }

        true
    }

    fn token_eq(lhs: &TokenTree, rhs: &TokenTree) -> bool {
        match (lhs, rhs) {
            (TokenTree::Group(lhs), TokenTree::Group(rhs)) if
                lhs.delimiter() == rhs.delimiter() && stream_eq(lhs.stream(), rhs.stream()) => true,
            (TokenTree::Punct(lhs), TokenTree::Punct(rhs)) if
                lhs.as_char() == rhs.as_char() && lhs.spacing() == rhs.spacing() => true,
            (TokenTree::Ident(lhs), TokenTree::Ident(rhs)) if
                lhs == rhs => true,
            (TokenTree::Literal(lhs), TokenTree::Literal(rhs)) if
                lhs.to_string() == rhs.to_string() => true,
            _ => false,
        }
    }
}
