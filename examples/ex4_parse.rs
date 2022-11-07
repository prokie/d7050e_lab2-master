use proc_macro2;

use d7050e_lab2::{ast::*, parse::*};
use syn;

// Study in details the `src/ast.rs` and `src/parser.rs`.

// In this part of the lab you should extend the parser
// to support the following tests.

#[test]
fn test_div() {
    let ts: proc_macro2::TokenStream = "8 / 2 / 3".parse().unwrap();
    let e: Expr = syn::parse2(ts).unwrap();
    println!("e {:?}", e);
    let e = climb(e);
    println!("e {:?}", e);
    println!("evaluation {:?}", e.eval());
    assert_eq!(e.eval(), Literal::Int(8 / 2 / 3));
}

#[test]
fn test_and_or() {
    let ts: proc_macro2::TokenStream = "true && false || false".parse().unwrap();
    let e: Expr = syn::parse2(ts).unwrap();
    println!("e {:?}", e);
    let e = climb(e);
    println!("e {:?}", e);
    println!("evaluation {:?}", e.eval());
    assert_eq!(e.eval(), Literal::Bool(true && false || false));
}

fn main() {}
