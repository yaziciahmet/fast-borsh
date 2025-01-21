mod macros;

#[proc_macro_derive(BorshSize)]
pub fn derive_borsh_size(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    macros::derive_borsh_size(input)
}
