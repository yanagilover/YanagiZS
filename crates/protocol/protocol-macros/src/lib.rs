use polymorphic::implement_polymorphic;
use proc_macro::TokenStream;
use property_accessors::implement_property_accessors;

mod polymorphic;
mod property_accessors;

#[proc_macro]
pub fn polymorphic(input: TokenStream) -> TokenStream {
    implement_polymorphic(input)
}

#[proc_macro_attribute]
pub fn property_accessors(_attr: TokenStream, item: TokenStream) -> TokenStream {
    implement_property_accessors(item)
}
