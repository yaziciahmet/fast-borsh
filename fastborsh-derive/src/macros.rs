use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub fn derive_borsh_size(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    // Generate the size computation based on the fields
    let size_calc = match input.data {
        Data::Struct(ref data) => match &data.fields {
            Fields::Named(ref fields) => {
                let field_sizes = fields.named.iter().map(|field| {
                    let field_name = &field.ident;
                    quote! { BorshSize::borsh_size(&self.#field_name) }
                });
                quote! { 0 #(+ #field_sizes)* }
            }
            Fields::Unnamed(ref fields) => {
                let field_sizes = fields.unnamed.iter().enumerate().map(|(i, _)| {
                    let index = syn::Index::from(i);
                    quote! { BorshSize::borsh_size(&self.#index) }
                });
                quote! { 0 #(+ #field_sizes)* }
            }
            Fields::Unit => quote! { 0 },
        },
        _ => {
            return TokenStream::from(quote! {
                compile_error!("BorshSize can only be derived for structs.");
            });
        }
    };

    // Generate the implementation
    let expanded = quote! {
        impl BorshSize for #struct_name {
            #[inline(always)]
            fn borsh_size(&self) -> usize {
                #size_calc
            }
        }
    };

    TokenStream::from(expanded)
}

