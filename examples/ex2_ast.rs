// A simple AST for integer expressions
use std::convert::From;

#[derive(Debug, Clone, PartialEq)]
enum Expr {
    Lit(Literal),
    BinOp(Op, Box<Expr>, Box<Expr>),
}

impl Expr {
    fn eval(&self) -> Literal {
        match self {
            Expr::Lit(literal) => literal.clone(),
            Expr::BinOp(op, left, right) => op.eval(left.eval(), right.eval()),
        }
    }

    fn bin_op(o: Op, left: Expr, right: Expr) -> Self {
        Expr::BinOp(o, Box::new(left), Box::new(right))
    }
}

impl From<Literal> for Expr {
    fn from(lit: Literal) -> Self {
        Expr::Lit(lit)
    }
}

impl From<i32> for Expr {
    fn from(i: i32) -> Self {
        Expr::Lit(Literal::Int(i))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Literal {
    Bool(bool),
    Int(i32),
}

impl From<i32> for Literal {
    fn from(i: i32) -> Self {
        Literal::Int(i)
    }
}

impl Literal {
    fn get_int(&self) -> i32 {
        match self {
            Literal::Int(i) => *i,
            _ => panic!("cannot get integer from Bool"),
        }
    }

    fn get_bool(&self) -> bool {
        unimplemented!()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
}

impl Op {
    fn eval(&self, left: Literal, right: Literal) -> Literal {
        match self {
            Op::Add => Literal::Int(left.get_int() + right.get_int()),
            Op::Sub => Literal::Int(left.get_int() - right.get_int()),
            Op::Mul => Literal::Int(left.get_int() * right.get_int()),
            _ => unimplemented!(),
        }
    }
}

fn main() {}

#[test]
fn test_add_7_8() {
    // Calculate 7 + 8;
    let e7 = Expr::Lit(Literal::Int(7));
    let e8 = Expr::Lit(Literal::Int(8));
    let e78 = Expr::BinOp(Op::Add, Box::new(e7), Box::new(e8));

    println!("{:?}, evaluates to {:?}", e78, e78.eval());
    assert_eq!(e78.eval(), Literal::Int(7 + 8));
}

// As we see it is quite inconvenient to construct expressions.
// The From trait is our saviour! (It allows us to use .into())
// We can also use the binop constructor to let it do the Box:ing.
#[test]
fn test1_into() {
    let e7 = 7.into();
    let e8 = 8.into();
    let e78 = Expr::bin_op(Op::Add, e7, e8);

    println!("{:?}, evaluates to {:?}", e78, e78.eval());
    assert_eq!(e78.eval(), Literal::Int(7 + 8));
}

#[test]
fn test_add_7_8_8() {
    // Calculate 7 + 8 + 8
    let e7 = 7.into();
    let e8: Expr = 8.into();
    let e78 = Expr::bin_op(Op::Add, e7, e8.clone());
    let e788 = Expr::bin_op(Op::Add, e78, e8);
    println!("{:?}, evaluates to {:?}", e788, e788.eval());
    assert_eq!(e788.eval(), Literal::Int(7 + 8 + 8));
}

// Now its time for you to make some tests
#[test]
fn test_ex1() {
    let e5 = 5.into();
    let e7 = 7.into();
    let e8: Expr = 8.into();

    let e78 = Expr::bin_op(Op::Add, e7, e8.clone());
    let e58 = Expr::bin_op(Op::Sub, e5, e8.clone());

    let exp = Expr::bin_op(Op::Add, e78, e58);

    // println!("{:?}, eval to {:?}",exp, exp.eval() );

    // Calculate 7 + 8 + 5 - 8
    // To do so first construct an AST representation
    // for the expression 7 + 8 + 5 - 8
    //
    // Your test should pass the below assertion
    assert_eq!(exp.eval(), Literal::Int(7 + 8 + 5 - 8));
}

#[test]
fn test_ex2() {
    let e1 = 1.into();
    let e2 = 2.into();
    let e3 = 3.into();

    let e12 = Expr::bin_op(Op::Sub, e1, e2);
    let exp = Expr::bin_op(Op::Sub, e12, e3);

    // Calculate 1 - 2 - 3
    // To do so first construct an AST representation
    // for the expression 1 - 2 - 3
    // Beware of associativity!
    //
    // Your test should pass the below assertion
    assert_eq!(exp.eval(), Literal::Int(1 - 2 - 3));
}

#[test]
fn test_ex3() {
    let e1: Expr = 1.into();
    let e2: Expr = 2.into();
    let e3: Expr = 3.into();
    let e8: Expr = 8.into();
    let e82 = Expr::bin_op(Op::Mul, e8, e2);
    let e182 = Expr::bin_op(Op::Sub, e1, e82);
    let exp = Expr::bin_op(Op::Sub, e182, e3);
    println!("{:?}, evaluates to {:?}", exp, exp.eval());

    // Calculate 1 - 8 * 2 - 3
    // To do so first construct an AST representation
    // for the expression 1 - 8 * 2 - 3
    // Beware of associativity and priority
    //
    // Your test should pass the below assertion
    assert_eq!(exp.eval(), Literal::Int(1 - 8 * 2 - 3));
}
