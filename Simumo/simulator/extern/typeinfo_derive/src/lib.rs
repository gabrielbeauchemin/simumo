extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(TypeInfo)]
pub fn type_info(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let tokens = quote! {

        impl TypeInfo for #name {
            const TYPENAME : &'static str = stringify!(#name);
        }
    };
    TokenStream::from(tokens)
}
