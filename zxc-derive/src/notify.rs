use super::*;

pub fn impl_notify_commander(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, type_generics, where_clause) =
        ast.generics.split_for_impl();
    quote! {
        impl #impl_generics NotifyCommander for #name #type_generics #where_clause {
            #[inline(always)]
            async fn notify_commander(&mut self) -> Result<(), <Self as HandleCommander>::Error>{
                Ok(())
            }
        }
    }
    .into()
}
