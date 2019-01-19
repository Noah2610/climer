#![recursion_limit="128"]
extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{ self, DeriveInput };

#[proc_macro_derive(TimeConversion)]
pub fn time_conversion_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_time_conversion(&ast)
}

fn impl_time_conversion(ast: &DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let gen = quote! {
        impl TimeConversion for #struct_name {
            fn h(&self)  -> u64 {
                self.hours
            }
            fn m(&self)  -> u64 {
                self.minutes
            }
            fn s(&self)  -> u64 {
                self.seconds
            }
            fn ms(&self) -> u64 {
                self.milliseconds
            }
            fn ns(&self) -> u64 {
                self.nanoseconds
            }

            fn set_h(&mut self, h: u64) {
                self.hours = h;
            }
            fn set_m(&mut self, m: u64) {
                self.minutes = m;
            }
            fn set_s(&mut self, s: u64) {
                self.seconds = s;
            }
            fn set_ms(&mut self, ms: u64) {
                self.milliseconds = ms;
            }
            fn set_ns(&mut self, ns: u64) {
                self.nanoseconds = ns;
            }
        }
    };
    gen.into()
}
