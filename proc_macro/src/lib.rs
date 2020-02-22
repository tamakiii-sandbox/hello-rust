extern crate proc_macro;

use crate::proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn trace(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let ret = item.clone();
  let ast = parse_macro_input!(item as ItemFn);
  dbg!(ast);
  ret
}