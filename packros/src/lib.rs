extern crate quote;
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::LitByteStr;
use std::convert::{From, TryInto};

#[proc_macro]
pub fn packed(input: TokenStream) -> TokenStream {
  let ast: LitByteStr = syn::parse(input).expect("Literal byte string");
  let val = ast.value();
  match val.len() {
    2 => {
      let c = u16::from_ne_bytes(val.as_slice().try_into().unwrap());
      TokenStream::from(quote! { #c })
    }
    4 => {
      let c = u32::from_ne_bytes(val.as_slice().try_into().unwrap());
      TokenStream::from(quote! { #c })
    }
    8 => {
      let c = u64::from_ne_bytes(val.as_slice().try_into().unwrap());
      TokenStream::from(quote! { #c })
    }
    16 => {
      let c = u128::from_ne_bytes(val.as_slice().try_into().unwrap());
      TokenStream::from(quote! { #c })
    }
    _ => panic!("Expected literal byte string of 2, 4, 8 or 16 bytes"),
  }
}
