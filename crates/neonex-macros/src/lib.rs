extern crate proc_macro;
use std::str::FromStr;

use proc_macro::TokenStream;
use rand::Rng;

#[proc_macro]
pub fn generate_32char_seed(_item: TokenStream) -> TokenStream {
    const CHAR_COUNT: usize = 32;

    let mut seed = String::new();

    let rng = rand::rng();
    let mut iter = rng.random_iter::<char>();
    while seed.chars().count() < CHAR_COUNT {
        seed.push(iter.next().unwrap());
    }

    assert_eq!(seed.chars().count(), CHAR_COUNT);

    let output = format!("\"{}\"", seed);

    TokenStream::from_str(&output).expect("Unable to produce TokenStream from random seed")
}
