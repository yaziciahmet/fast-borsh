use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub(crate) fn fast_serialize_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    // Handle struct fields
    let fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => &fields.named,
            Fields::Unnamed(ref fields) => &fields.unnamed,
            Fields::Unit => {
                return TokenStream::from(quote! {
                    compile_error!("FastSerialize cannot be derived for unit structs.");
                });
            }
        },
        _ => {
            return TokenStream::from(quote! {
                compile_error!("FastSerialize can only be derived for structs.");
            });
        }
    };

    // Generate SIZE computation
    let size_expr = fields.iter().map(|field| {
        let ty = &field.ty;
        quote! { <#ty as FastSerialize>::SIZE }
    });

    // Generate the implementation
    let expanded = quote! {
        impl FastSerialize for #struct_name {
            const SIZE: usize = 0 #(+ #size_expr)*;
        }
    };

    TokenStream::from(expanded)
}
