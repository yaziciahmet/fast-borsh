mod macros;

#[proc_macro_derive(FastBorshSerialize)]
pub fn fast_serialize_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    macros::fast_serialize_derive(input)
}
