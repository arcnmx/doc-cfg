#[cfg(not(feature = "quote"))]
macro_rules! quote {
    (
        $($i:ident),* =>
        $($tt:tt)*
    ) => {
        <TokenStream as std::str::FromStr>::from_str(&quote_macro::quote_replace(stringify!($($tt)*), &[ $(
            (stringify!($i), &$i as &dyn std::fmt::Display),
        )* ]))
            .expect("internal parse error")
    };
    (
        $($tt:tt)*
    ) => {
        <TokenStream as std::str::FromStr>::from_str(&stringify!($($tt)*))
            .expect("internal parse error")
    };
}

#[cfg(feature = "quote")]
macro_rules! quote {
    (
        $($i:ident),* =>
        $($tt:tt)*
    ) => {
        quote::quote! { $($tt)* }
    };
    (
        $($tt:tt)*
    ) => {
        quote::quote! { $($tt)* }
    };
}

#[cfg(not(feature = "quote"))]
pub fn quote_replace(template: &'static str, replacements: &[(&'static str, &dyn std::fmt::Display)]) -> String {
    let mut s = template.to_owned();
    for &(id, disp) in replacements {
        s = s.replace(&format!("# {}", id), &disp.to_string());
    }

    s
}
