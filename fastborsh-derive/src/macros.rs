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

    // Generate serialization logic
    let serialize_body = fields.iter().enumerate().map(|(index, field)| {
        let ty = &field.ty;
        let field_access = match &field.ident {
            Some(ident) => quote! { self.#ident },
            None => {
                let idx = syn::Index::from(index);
                quote! { self.#idx }
            }
        };
        quote! {
            {
                let serialized = #field_access.fast_serialize();
                buf[offset..offset + <#ty as FastSerialize>::SIZE].copy_from_slice(&serialized);
                offset += <#ty as FastSerialize>::SIZE;
            }
        }
    });

    // Generate the implementation
    let expanded = quote! {
        impl FastSerialize for #struct_name {
            const SIZE: usize = 0 #(+ #size_expr)*;

            fn fast_serialize(&self) -> [u8; Self::SIZE] {
                let mut buf = [0u8; Self::SIZE];
                let mut offset = 0;
                #(#serialize_body)*
                buf
            }
        }
    };

    TokenStream::from(expanded)
}
