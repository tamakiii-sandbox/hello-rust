extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn hoge(_args: TokenStream, _item: TokenStream) -> TokenStream {
    let stream = quote! {
      {
          println!("hogehoge");
      }
    };

    stream.into()
}