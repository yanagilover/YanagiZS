use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Fields, GenericArgument, ItemStruct, PathArguments, Type};

pub fn implement_property_accessors(input: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(input as ItemStruct);
    let struct_name = &input_struct.ident;

    let Fields::Named(fields) = &input_struct.fields else {
        panic!("only structs with named fields are supported by #[property_accessors]");
    };

    let mut get_methods = proc_macro2::TokenStream::new();
    let mut get_mut_methods = proc_macro2::TokenStream::new();

    for field in fields.named.iter() {
        let name = field.ident.as_ref().unwrap();
        let ty = get_underlying_type(&field.ty);

        get_methods.extend(quote! {
            pub fn #name(&self) -> &#ty {
                self.#name.as_ref().unwrap()
            }
        });

        let mut_getter_name = format_ident!("{}_mut", name);
        get_mut_methods.extend(quote! {
            pub fn #mut_getter_name(&mut self) -> &mut #ty {
                self.#name.as_mut().unwrap()
            }
        });
    }

    quote! {
        #input_struct

        impl #struct_name {
            #get_methods
            #get_mut_methods
        }
    }
    .into()
}

fn get_underlying_type(ty: &Type) -> &Type {
    let Type::Path(path) = ty else {
        panic!("all fields of struct for #[property_accessor] should be Option<T>");
    };

    let last_segment = path.path.segments.last();
    let last_segment = last_segment.as_ref().unwrap();

    let PathArguments::AngleBracketed(args) = &last_segment.arguments else {
        panic!("all fields of struct for #[property_accessor] should be Option<T>");
    };

    if let GenericArgument::Type(ty) = args.args.first().unwrap() {
        ty
    } else {
        panic!("all fields of struct for #[property_accessor] should be Option<T>");
    }
}
