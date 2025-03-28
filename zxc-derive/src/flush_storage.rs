use super::*;

pub fn impl_flush_storage(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, type_generics, where_clause) =
        ast.generics.split_for_impl();
    quote! {
        impl #impl_generics FlushStorage for #name #type_generics #where_clause {
            #[inline(always)]
            async fn flush_storage(&mut self,stream: &mut UnixStream) -> Result<(), UnixSockError> {
                Ok(())
            }
        }
    }
    .into()
}
