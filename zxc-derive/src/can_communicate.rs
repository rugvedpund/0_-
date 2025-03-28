use super::*;

pub fn impl_can_communicate(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, type_generics, where_clause) =
        ast.generics.split_for_impl();
    quote! {
        impl #impl_generics CanCommunicate for #name #type_generics #where_clause {
            fn sender(&mut self) -> &mut Sender<CommanderRequest> {
                &mut self.commander_sendr
            }

            fn receiver(&mut self) -> &mut Receiver<CommanderResponse> {
                &mut self.commander_recvr
            }
        }
    }.into()
}
