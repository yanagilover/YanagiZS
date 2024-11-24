use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    braced, bracketed,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token, Ident, LitInt, Token, Type,
};

struct PolymorphicInput {
    name: Ident,
    _brace_token: token::Bracket,
    common_fields: Punctuated<FieldEntry, Token![,]>,
    structs: Punctuated<StructEntry, Token![,]>,
}

struct FieldEntry {
    name: Ident,
    _colon_token: Token![:],
    ty: Type,
}

struct StructEntry {
    name: Ident,
    _brace_token: token::Brace,
    fields: Punctuated<FieldEntry, Token![,]>,
    _eq_token: Token![=],
    discriminant: LitInt,
}

impl Parse for PolymorphicInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            name: input.parse()?,
            _brace_token: bracketed!(content in input),
            common_fields: content.parse_terminated(FieldEntry::parse, Token![,])?,
            structs: input.parse_terminated(StructEntry::parse, Token![,])?,
        })
    }
}

impl Parse for FieldEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            _colon_token: input.parse()?,
            ty: input.parse()?,
        })
    }
}

impl Parse for StructEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            name: input.parse()?,
            _brace_token: braced!(content in input),
            fields: content.parse_terminated(FieldEntry::parse, Token![,])?,
            _eq_token: input.parse()?,
            discriminant: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn polymorphic(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as PolymorphicInput);
    let enum_name = &input.name;

    let mut variants = quote!();
    for stru in input.structs.iter() {
        let variant_name = &stru.name;
        let variant_discr = &stru.discriminant;
        let mut fields = quote!();

        for field in input.common_fields.iter().chain(stru.fields.iter()) {
            let field_name = &field.name;
            let field_type = &field.ty;

            fields.extend(quote! {
                #field_name: #field_type,
            });
        }

        variants.extend(quote! {
            #variant_name {
                #fields
            } = #variant_discr,
        });
    }

    let mut getters_and_setters = quote!();
    for field in input.common_fields.iter() {
        let field_type = &field.ty;
        let field_name = &field.name;
        let getter_name = Ident::new(&format!("get_{}", field_name), Span::call_site());
        let setter_name = Ident::new(&format!("set_{}", field_name), Span::call_site());

        let mut getter_match_arms = quote!();
        let mut setter_match_arms = quote!();

        for stru in input.structs.iter() {
            let stru_name = &stru.name;
            getter_match_arms.extend(quote! {
                Self::#stru_name { #field_name, .. } => #field_name,
            });
            setter_match_arms.extend(quote! {
                Self::#stru_name { #field_name, .. } => { *#field_name = value; },
            });
        }

        getters_and_setters.extend(quote! {
            pub fn #getter_name(&self) -> &#field_type {
                match self {
                    #getter_match_arms
                }
            }

            pub fn #setter_name(&mut self, value: #field_type) {
                match self {
                    #setter_match_arms
                }
            }
        });
    }

    // can't use base_fields_count directly in 'quote!'
    // 'quote!' puts suffixed literal, but it's not allowed in attributes
    let base_fields_count = input.common_fields.len();
    let base_attr = format!("#[base = {base_fields_count}]")
        .parse::<proc_macro2::TokenStream>()
        .unwrap();

    quote! {
        #[derive(Debug, Clone, ::qwer::OctData)]
        #[repr(u32)]
        #base_attr
        pub enum #enum_name {
            #variants
        }

        impl #enum_name {
            #getters_and_setters
        }
    }
    .into()
}
