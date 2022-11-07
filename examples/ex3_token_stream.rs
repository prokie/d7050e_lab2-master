use proc_macro2;

// The same ast as in ex2_ast.rs, but now part of the d7050e_lab2 library.
use d7050e_lab2::ast;
use syn;

// Rust has a hand written parser (hidden from the user).
// It's hidden since the compiler implementation may change over time.
// The crate `syn` provides a stable AST and parsers for Rust code.
// It is possible to implement custom syntax in Rust using
// `proc_macro2` together with the `syn` crate, to parse
// the token stream, process it and emit Rust code.
//
// This is useful to create Domain Specific Language extensions.
// As an example we use it to implement RTIC https://crates.io/crates/cortex-m-rtic
//

fn ast_expr(e: &syn::Expr) -> ast::Expr {
    match e {
        syn::Expr::Lit(syn::ExprLit {
            attrs: _,
            lit: syn::Lit::Int(l),
        }) => {
            let i: i32 = l.base10_parse().unwrap();
            ast::Expr::from(i)
        }
        syn::Expr::Binary(b) => {
            let left = ast_expr(&*b.left);
            let right = ast_expr(&*b.right);
            let op = match b.op {
                syn::BinOp::Add(_) => ast::Op::Add,
                _ => unimplemented!(),
            };
            ast::Expr::bin_op(op, left, right)
        }
        _ => unimplemented!(),
    }
}

fn main() {
    // Parse "1 + 2" into a proc_macro2 token stream.
    // The result is [Literal { lit: 1 }, Punct { char: '+', spacing: Alone }, Literal { lit: 2 }]
    // I.e., the literal "1", the character "-" and the literal "2"
    let ts: proc_macro2::TokenStream = "1 + 2".parse().unwrap();
    println!("ts {:?}", ts);

    // Parse it into a syn::Expr (AST node)
    // Here the result is much more elaborate, as the token stream
    // Expr is a huge enum, capturing all types of Rust expressions
    // (syn is designed to capture the complete syntax of the Rust language)
    // As you see there is a LOT of information, most of we are not interested in.
    let e: syn::Expr = syn::parse2(ts).unwrap();
    println!("e {:#?}", e);

    // We can parse the syn::Expr to our own ast::Expr (with less info, easier to work with).
    let e = ast_expr(&e);
    println!("ast_e {:?}", e);

    // We can even evaluate this expression
    println!("evaluate {:?}", e.eval());
}

// Now its time for you to extend the `ast_expr` to parse
// other expressions.

#[test]
fn test_1_minus_2_times_3() {
    let ts: proc_macro2::TokenStream = "1 - 2 * 3".parse().unwrap();
    println!("ts {:?}", ts);

    let e: syn::Expr = syn::parse2(ts).unwrap();
    println!("e {:#?}", e);

    // this will panic until you have fixed the AST and parser
    let e = ast_expr(&e);
    println!("ast_e {:?}", e);

    // Once you get the parsing to work then uncomment the assertion.
    assert_eq!(e.eval(), ast::Literal::Int(1 - 2 * 3));
}

#[test]
fn test_lp_1_minus_2_rp_times_3() {
    let ts: proc_macro2::TokenStream = "(1 - 2) * 3".parse().unwrap();
    println!("ts {:?}", ts);

    let e: syn::Expr = syn::parse2(ts).unwrap();
    println!("e {:#?}", e);

    // this will panic until you have fixed the AST and parser
    //let e = ast_expr(&e);
    println!("ast_e {:?}", e);

    // Once you get the parsing to work then uncomment the assertion.
    // You also need to fix the eval function.
    //assert_eq!(e.eval(), ast::Literal::Int((1 - 2) * 3));
}
