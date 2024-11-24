use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput, Meta, MetaList};

mod oct_data_derive;

/// Generate `OctData` implementation for structs and enums that have all fields
/// implement `OctData`.
///
/// For structs, fields are written one by one in order.
/// If the struct must pad a byte due to it being a `CPropertyObject`, the `#[property_object]`
/// attribute must be present, and must contain the value to pad with.
///
/// e.g. `#[property_object(u16, 0x01)]` will pad with 0x01 as a u16.
///
/// In the presence of these property objects, all fields must be Optional, e.g. `Option<T>`,
/// and must also have a tag attribute attached to them for marshalling and unmarshalling, of the
/// form `#[tag = <number>]`.
///
/// For enums, the structure starts with a discriminant with the type specified in the `#[repr]` of
/// the enum, followed by the fields of the enum one by one.
#[proc_macro_derive(
    OctData,
    attributes(
        property_object,
        property_blob,
        server_only,
        tag,
        root,
        base,
        polymorphic_none
    )
)]
pub fn derive_message(item: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(item as DeriveInput);
    match oct_data_derive::imp(&parsed) {
        Ok(item) => item.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro_derive(ProtocolID, attributes(id))]
pub fn derive_protocol_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let id = match input.attrs.iter().find(|attr| attr.path().is_ident("id")) {
        Some(attr) => match attr.meta {
            Meta::List(MetaList { ref tokens, .. }) => tokens.into_token_stream(),
            _ => panic!("Invalid cmdid attribute value"),
        },
        _ => panic!("Protocol ID should be specified with 'id' attribute"),
    };

    let protocol_name_str = struct_name.to_string();
    TokenStream::from(::quote::quote! {
        impl crate::ProtocolID for #struct_name {
            const PROTOCOL_ID: u16 = #id;
            const NAME: &str = #protocol_name_str;
        }
    })
}
