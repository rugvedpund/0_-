use super::*;

pub fn impl_buffer(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, type_generics, where_clause) =
        ast.generics.split_for_impl();
    quote! {
        impl #impl_generics Buffer for #name #type_generics #where_clause {
            fn buf_as_mut(&mut self) -> &mut BytesMut {
                &mut self.buf
            }
        }
    }
    .into()
}
