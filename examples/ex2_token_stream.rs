use proc_macro2::TokenStream;

fn main() {
    let ts: TokenStream = "1 + 2".parse().unwrap();

    println!("token stream {:?}", ts);
}
