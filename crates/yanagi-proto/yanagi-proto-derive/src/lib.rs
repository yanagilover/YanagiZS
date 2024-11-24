use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, Data, DeriveInput, Field, Fields, GenericArgument, Meta, MetaList, Path,
    PathArguments, Type, TypePath,
};

#[must_use]
fn get_type_name(path: &Path) -> String {
    path.segments.last().unwrap().ident.to_string()
}

#[must_use]
fn extract_field_type(field: &Field) -> String {
    match &field.ty {
        Type::Path(TypePath { path, .. }) => {
            let last_segment = path.segments.last().unwrap();
            match &last_segment.arguments {
                PathArguments::AngleBracketed(args) => {
                    if let Some(GenericArgument::Type(Type::Path(TypePath { path, .. }))) =
                        args.args.last()
                    {
                        get_type_name(path)
                    } else {
                        get_type_name(path)
                    }
                }
                _ => get_type_name(path),
            }
        }
        _ => panic!("Unsupported field type"),
    }
}

#[proc_macro_derive(XorFields, attributes(xor))]
pub fn xor_fields_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("XorFields only supports structs with named fields"),
        },
        Data::Enum(_) => {
            let expanded = quote! {
                impl crate::XorFields for #struct_name {
                    fn xor_fields(&mut self) {
                    }
                }
            };

            return TokenStream::from(expanded);
        }
        _ => panic!("XorFields only supports structs"),
    };

    let field_xors = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_ty = extract_field_type(field);
        let xor_attr = field.attrs.iter().find(|attr| attr.path().is_ident("xor"));

        xor_attr.map_or_else(
            || {
                if field_ty.chars().next().unwrap().is_uppercase()
                    && field_ty != "String"
                    && field_ty != "Any"
                {
                    quote! {
                        self.#field_name.xor_fields();
                    }
                } else {
                    quote! {}
                }
            },
            |attr| match attr.meta {
                Meta::List(MetaList { ref tokens, .. }) => {
                    let xor_value = tokens.into_token_stream();
                    if xor_value.to_string() != "0" {
                        quote! {
                        if self.#field_name != 0 {
                            self.#field_name ^= #xor_value;
                        }
                        }
                    } else {
                        quote! {}
                    }
                }
                _ => panic!("Invalid xor attribute value"),
            },
        )
    });

    let expanded = quote! {
        impl crate::XorFields for #struct_name {
            fn xor_fields(&mut self) {
                #(#field_xors)*
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(CmdID, attributes(cmdid))]
pub fn cmd_id_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let id = match input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("cmdid"))
    {
        Some(attr) => match attr.meta {
            Meta::List(MetaList { ref tokens, .. }) => tokens.into_token_stream(),
            _ => panic!("Invalid cmdid attribute value"),
        },
        _ => 0u16.into_token_stream(),
    };

    TokenStream::from(quote! {
        impl crate::CmdID for #struct_name {
            const CMD_ID: u16 = #id;
        }
    })
}
