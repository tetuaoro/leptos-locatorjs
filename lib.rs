extern crate proc_macro;
extern crate quote;
extern crate syn;

use std::str::FromStr;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use regex::{Captures, Regex};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn add_locatorjs_id(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;

    let re = Regex::new(r"<[a-z][^/>]+>");
    // Modify the block to add the `data-locatorjs-id` attribute
    let mut modified_block = TokenStream2::new();

    if let Ok(re) = re {
        // Convert the block token stream to a string
        let block = block.into_token_stream().to_string();

        // Replace all HTML opening tags with a modified version that includes the locatorjs attribute
        let new_block = re.replace_all(&block, |captures: &Captures| {
            let tag_element = &captures[0];
            let locatorjs = format!(" attr:data-locatorjs-id=\"{}::{}\"", file!(), line!());
            let new_token_str = tag_element.replace(">", &format!("{locatorjs}>"));
            new_token_str
        });

        if let Ok(new_block) = TokenStream2::from_str(&new_block) {
            // Add the modified block to the new token stream
            modified_block.extend(new_block);
        }
    }

    let output = quote! {
        #(#attrs)*
        #vis #sig
        #modified_block
    };

    // Convert the new token stream to a token stream and return it
    TokenStream::from(output)
}
