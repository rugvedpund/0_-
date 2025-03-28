use super::*;

pub fn impl_get_path(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, type_generics, where_clause) =
        ast.generics.split_for_impl();
    let var_path = format_ident!("path");
    quote! {
        impl #impl_generics GetPath for #name #type_generics #where_clause {
            fn get_path(&mut self) -> &PathBuf {
                &self.#var_path
            }
        }
    }
    .into()
}
