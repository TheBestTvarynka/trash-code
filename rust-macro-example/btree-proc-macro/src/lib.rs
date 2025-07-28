use proc_macro::TokenStream;

struct Pair {
    key: syn::Expr,
    _colon: syn::Token![=>],
    value: syn::Expr,
}

impl syn::parse::Parse for Pair {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            key: input.parse()?,
            _colon: input.parse()?,
            value: input.parse()?,
        })
    }
}

struct Pairs(syn::punctuated::Punctuated<Pair, syn::Token![,]>);

impl syn::parse::Parse for Pairs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse_terminated(Pair::parse)?))
    }
}

#[proc_macro]
pub fn btree_2(input: proc_macro::TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as Pairs);
    impl_btree_map_macro(ast)
}

fn impl_btree_map_macro(pairs: Pairs) -> TokenStream {
    let pairs = pairs.0.into_iter().map(|Pair { key, value, .. }| {
        quote::quote! {(#key, #value)}
    });
    quote::quote! { std::collections::BTreeMap::from([#(#pairs),*]) }.into()
}
