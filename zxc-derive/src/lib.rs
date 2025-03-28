mod can_communicate;
mod file_ops;
mod get_path;
mod id;
mod repeater_read_from_file;
mod role;
use can_communicate::*;
use file_ops::*;
use get_path::*;
use id::*;
use proc_macro::TokenStream;
use quote::*;
use repeater_read_from_file::*;
use role::*;
mod buffer;
mod flush_storage;
use buffer::*;
use flush_storage::*;
use syn::parse_macro_input;
mod close_action;
use close_action::*;
mod notify;
use notify::*;

#[proc_macro_derive(RepeaterReadFile)]
pub fn repeaterreadfile(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    impl_repeater_read_from_file(&ast)
}

#[proc_macro_derive(GetPath)]
pub fn getpath(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    impl_get_path(&ast)
}

#[proc_macro_derive(FileOps)]
pub fn fileops(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    impl_file_ops(&ast)
}

#[proc_macro_derive(GetRole)]
pub fn get_role(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    impl_get_role(&ast)
}

#[proc_macro_derive(CanCommunicate)]
pub fn can_communicate(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    impl_can_communicate(&ast)
}

#[proc_macro_derive(Id)]
pub fn id(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    impl_id(&ast)
}

#[proc_macro_derive(Buffer)]
pub fn buffer(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    impl_buffer(&ast)
}

#[proc_macro_derive(FlushStorage)]
pub fn flush_storage(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    impl_flush_storage(&ast)
}

#[proc_macro_derive(CloseAction)]
pub fn close_action(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    impl_close_action(&ast)
}

#[proc_macro_derive(NotifyCommander)]
pub fn notify_commander(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    impl_notify_commander(&ast)
}
