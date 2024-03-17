use {proc_macro::TokenStream, quote::quote, syn::DeriveInput};

#[proc_macro_derive(ResourceTag)]
pub fn resource_tag_derive_macro(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl ResourceTag for #name {
            fn as_any(&self) -> &dyn core::any::Any {
                self
            }
        }
    };
    gen.into()
}
