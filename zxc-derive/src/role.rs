use super::*;

pub fn impl_get_role(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, type_generics, where_clause) =
        ast.generics.split_for_impl();
    quote! {
        impl #impl_generics GetRole for #name #type_generics #where_clause {
            fn role(&self) -> Role {
                self.role
            }
        }
    }
    .into()
}
