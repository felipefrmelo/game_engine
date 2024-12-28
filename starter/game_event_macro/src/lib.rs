use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(IterableEnum)]
pub fn iterable_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let variants: Vec<_> = if let Data::Enum(data_enum) = input.data {
        data_enum
            .variants
            .into_iter()
            .map(|variant| variant.ident)
            .collect()
    } else {
        panic!("#[derive(IterableEnum)] is only valid for enums");
    };

    let variants_len = variants.len();

    let gen = quote! {
        impl #name {
            pub fn iter() -> impl Iterator<Item = &'static #name> {
                static EVENTS: [#name; #variants_len] = [
                    #( #name::#variants, )*
                ];
                EVENTS.iter()
            }
        }
    };

    gen.into()
}
