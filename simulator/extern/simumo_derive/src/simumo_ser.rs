use proc_macro2::{Ident, TokenStream};
use syn::{Fields, Path, Type};

/// Generate a block serialized field for a
/// collection of fields
pub fn make_ser_block(fields: &Fields, struct_name: &Ident) -> Vec<TokenStream> {
    //note :: it is probably possible to use ident without cloning
    let names: Vec<Ident> = fields
        .iter()
        .map(|f| match f.ident.clone() {
            Some(i) => i,
            None => struct_name.clone(),
        })
        .collect();
    let types: Vec<String> = fields.iter().map(|f| type_to_str(&f.ty)).collect();

    let metric_tags: Vec<bool> = fields
        .iter()
        .map(|f| {
            f.attrs
                .iter()
                .any(|att| path_to_str(&att.path) == "simumo_metric")
        })
        .collect();

    let result: Vec<TokenStream> = izip!((0..names.len()), &names, &types, &metric_tags)
        .map(|(i, name, type_str, metric_tags)| {
            make_ser_statement(name, type_str, *metric_tags, i, struct_name)
        })
        .collect();

    result
}

/// Generatea serialize statement
/// if it is a metric from Dimensioned it gets unwraped
pub fn make_ser_statement(
    field_name: &Ident,
    ttype: &str,
    is_metric: bool,
    arg_number: usize,
    struct_name: &Ident,
) -> TokenStream {
    let val = if field_name == struct_name {
        quote!(self.#arg_number)
    } else {
        quote!(self.#field_name)
    };

    let (res, val) = if is_metric {
        (quote!(Some(#ttype.to_owned())), quote!(#val.value_unsafe))
    } else {
        (quote!(None), quote!(#val.clone()))
    };

    let field_name = quote!(stringify!(#field_name).to_string());

    quote! {
        seq.serialize_element(&LogDataEntry::new(
            #field_name,
            #res,
            #val
        ))?;
    }
}

/// convert type to a string only if it is a path type
/// path types are a sequence of identifier splitted with ::
pub fn type_to_str(unknown_type: &Type) -> String {
    if let syn::Type::Path(type_path) = unknown_type {
        path_to_str(&type_path.path)
    } else {
        String::from("")
    }
}

/// Get the first element in a identifier path
/// example path = path::to::selement
/// todo :: this line is unsafe when we use complex path to components
pub fn path_to_str(path: &Path) -> String {
    let segments = &path.segments;
    if segments.len() == 1 {
        //note :: it's probably possible to do it with clones
        segments[0].ident.clone().to_string()
    } else {
        String::from("")
    }
}
