extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro2::TokenStream;
use simumo_ser::make_ser_block;
use syn::DeriveInput;

#[macro_use]
mod izip;
mod simumo_ser;

/// Custom serialization derivation
///
/// can handle custom field name for field having the "simumo_metric" tag
#[proc_macro_derive(SimumoSerialize, attributes(simumo_metric))]
pub fn simumo_serialize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;

    if let syn::Data::Struct(body) = &ast.data {
        let ser_block = make_ser_block(&body.fields, name);
        let size = ser_block.len();
        let output = quote! {

            impl Serialize for #name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok,S::Error>
                where S: Serializer {
                    let mut seq = serializer.serialize_seq(Some(#size))?;
                    //expand serialization Tokens
                    #(#ser_block)*
                    seq.end()
                }
            }
        };
        output.into()
    } else {
        panic!("#[derive(SimumoLoggable)] is only defined for structs");
    }
}

/// Macro that provide a collection of derivation
/// for components with the role of Tag
#[proc_macro_attribute]
pub fn simucomponent_tag(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let output = quote! {

        #[derive(
            Component,
            TypeInfo,
            Debug,
            Clone,
            Deserialize
            )]
        #input

    };
    output.into()
}

/// Macro that provide a collection of derivation
/// for components that have the role of data in Simumo
#[proc_macro_attribute]
pub fn simucomponent_data(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let output = quote! {

        #[derive(
            Component,
            TypeInfo,
            Debug,
            Clone,
            SimumoSerialize,
            Deserialize
            )]
        #input
    };
    output.into()
}

/// Macro that provide a collection of derivation
/// for a basic component in simumo
#[proc_macro_attribute]
pub fn simucomponent_base(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let output = quote! {
        #[derive(
            Component,
            TypeInfo
            )]
        #input
    };
    output.into()
}

/// Macro that provide a collection of derivation
/// for a basic system in simumo
#[proc_macro_attribute]
pub fn simusystem(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let output = quote! {
        #[derive(
            TypeInfo,
            Deserialize
            )]
        #input
    };
    output.into()
}

/// Macro that provide a collection of derivation
/// for a basic simulator ressource
#[proc_macro_attribute]
pub fn simuresource(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let output = quote! {
        #[derive(
            TypeInfo,
            Default
            )]
        #input
    };
    output.into()
}
