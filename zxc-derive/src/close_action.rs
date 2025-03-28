use super::*;

pub fn impl_close_action(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, type_generics, where_clause) =
        ast.generics.split_for_impl();
    quote! {
        impl #impl_generics CloseAction for #name #type_generics #where_clause {
            #[inline(always)]
            async fn close_action(&mut self) -> Result<(), std::io::Error> {
                Ok(())
            }
        }
    }
    .into()
}
