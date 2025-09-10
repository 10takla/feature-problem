use proc_macro::TokenStream;

#[proc_macro]
pub fn grammar(_input: TokenStream) -> TokenStream {
    parser::parse();

    // Let's imagine that grammar generates some code by parser::parse.
    Default::default()
}