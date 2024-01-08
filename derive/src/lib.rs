use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, DeriveInput, Fields};

/// Input is Rust source code that the user wrote
/// Output is the Rust source code that our program generates.
/// It should output a block like `impl MyTrait for <user's type>`.
#[proc_macro_derive(ListOfValues)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse2(input.into()).unwrap();
    TokenStream::from(impl_derive(input))
}

fn impl_derive(input: DeriveInput) -> TokenStream2 {
    match input.data {
        syn::Data::Struct(struct_fields) => {
            let name = input.ident;
            impl_derive_on_struct(name, struct_fields)
        }
        syn::Data::Enum(_) => todo!("we don't support enums yet"),
        syn::Data::Union(_) => todo!("we don't support unions yet"),
    }
}

fn impl_derive_on_struct(name: proc_macro2::Ident, data: syn::DataStruct) -> TokenStream2 {
    let Fields::Named(ref fields) = data.fields else {
        return quote! {
            compile_error!("This macro only supports named fields right now")
        };
    };

    let each_field: Vec<_> = fields
        .named
        .iter()
        .filter_map(|field| field.ident.as_ref().map(|ident| (ident, field.ty.span())))
        .map(|(field_name, field_span)| {
            quote_spanned! {field_span=>
                (stringify!(#field_name).to_owned(), self.#field_name.to_string()),
            }
        })
        .collect();

    quote! {
        impl rust_club_types::ListOfValues for #name {
            fn get(&self) -> std::collections::HashMap<String, String> {
                std::collections::HashMap::from([
                   #(#each_field)*
                ])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn test_struct() {
        let input = quote! {
            struct Adam {
                height: u8,
                eye_color: String,
            }
        };
        let input: DeriveInput = syn::parse2(input).unwrap();
        let output = get_text_fmt(&impl_derive(input));
        println!("{output}");
    }

    fn clean_text(s: &str) -> String {
        // Add newlines after end-braces at <= two levels of indentation.
        if cfg!(not(windows)) {
            let regex = regex::Regex::new(r"(})(\n\s{0,8}[^} ])").unwrap();
            regex.replace_all(s, "$1\n$2").to_string()
        } else {
            let regex = regex::Regex::new(r"(})(\r\n\s{0,8}[^} ])").unwrap();
            regex.replace_all(s, "$1\r\n$2").to_string()
        }
    }

    fn get_text_fmt(unformatted: &TokenStream2) -> String {
        let content = rustfmt_wrapper::rustfmt(unformatted).unwrap();
        clean_text(&content)
    }
}
