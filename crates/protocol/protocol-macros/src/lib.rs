use polymorphic::implement_polymorphic;
use proc_macro::TokenStream;
mod polymorphic;

#[proc_macro]
pub fn polymorphic(input: TokenStream) -> TokenStream {
    implement_polymorphic(input)
}
