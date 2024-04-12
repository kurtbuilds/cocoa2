use proc_macro::TokenStream;
use syn::DeriveInput;
use quote::quote;

#[proc_macro_derive(Bridge)]
pub fn bridge_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast: DeriveInput = syn::parse(input).unwrap();

    // Build the trait implementation
    let gen = impl_bridge(&ast);

    // Return the generated impl
    gen.into()
}

// create a derive implementation that just prints the name struct and the id
#[proc_macro_derive(Display)]
pub fn display_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast: DeriveInput = syn::parse(input).unwrap();

    // Build the trait implementation
    let gen = impl_display(&ast);

    // Return the generated impl
    gen.into()
}

fn impl_bridge(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Bridge for #name {
            fn class() -> &'static Class {
                ::objc::class!(#name)
            }

            fn id(&self) -> id {
                self.0
            }

            unsafe fn from_id(id: id) -> Self {
                Self(id)
            }
        }
    };
    gen.into()
}

fn impl_display(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(stringify!(#name))
                    .field(&self.0)
                    .finish()
            }
        }
    };
    gen.into()
}