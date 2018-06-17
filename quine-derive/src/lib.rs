#![feature(proc_macro)]

extern crate proc_macro;
use proc_macro::TokenStream;
use std::str::FromStr;

#[proc_macro_attribute]
pub fn quine(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = args.to_string();
    let args = args.split(",").map(|s| s.trim().to_owned()).collect::<Vec<_>>();
    let data_name = &args.get(0).unwrap();
    let function_name = args.get(1);

    let input = input.to_string();
    let mut processed_input = String::new();
    let mut prev_char = None;
    for c in input.chars() {
        match (prev_char, c) {
            (Some(' '), ' ') => {},
            (_, _) => {
                processed_input.push(c);
            },
        }
        prev_char = Some(c);
    }
    let mut input = processed_input;
    input = input.replace("\n", "");
    for pat in &["::", "(", ")", ".", ",", "}", "{", ";", ".", "*", "/", "+", "<", ">", "-", "=", "&&", "||"] {
        input = input.replace(&format!(" {}", pat), &pat);
        input = input.replace(&format!("{} ", pat), &pat);
    }
    let mut output = String::new();

    output.push_str(&format!("const {}: &'static [u8] = &[", data_name));
    let mut input_data = input.clone();
    if let Some(function_name) = function_name {
        input_data = input_data.replace(function_name.as_str(), "main");
    }
    for b in input_data.bytes() {
        output.push_str(&format!("0x{:x}, ", b));
    }
    output.push_str("];");
    output.push_str(&input);
    //println!("{}", input);
    //println!("{}", output);
    TokenStream::from_str(&output).unwrap()
}

