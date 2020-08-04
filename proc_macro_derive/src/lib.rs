extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_attribute]
pub fn hello_macro_derive(_attributes: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).expect("parse error");

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::Item) -> TokenStream {
    match *ast {
        syn::Item::Impl(ref implement) => {
            let items = &implement.items;
            let code =
                quote! {
                    impl Pancakes {
                        #(#items)*
                    }
                };
            code.into()
        },
        _ => panic!("expected impl"),
    }
}
