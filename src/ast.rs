#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Lit(Literal),
    BinOp(Op, Box<Expr>, Box<Expr>),
    Par(Box<Expr>),
}

impl Expr {
    pub fn eval(&self) -> Literal {
        match self {
            Expr::Lit(literal) => literal.clone(),
            Expr::BinOp(op, left, right) => op.eval(left.eval(), right.eval()),
            Expr::Par(e) => e.eval(),
        }
    }

    pub fn bin_op(o: Op, left: Expr, right: Expr) -> Self {
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
pub enum Literal {
    Bool(bool),
    Int(i32),
}

impl From<i32> for Literal {
    fn from(i: i32) -> Self {
        Literal::Int(i)
    }
}

impl From<Expr> for Literal {
    fn from(e: Expr) -> Self {
        match e {
            Expr::Lit(l) => l,
            _ => unreachable!(),
        }
    }
}

impl Literal {
    pub fn get_int(&self) -> i32 {
        match self {
            Literal::Int(i) => *i,
            _ => panic!("cannot get integer from Bool"),
        }
    }

    pub fn get_bool(&self) -> bool {
        unimplemented!()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
}

impl Op {
    // Evaluate operator to literal
    pub fn eval(&self, left: Literal, right: Literal) -> Literal {
        match self {
            Op::Add => Literal::Int(left.get_int() + right.get_int()),
            Op::Sub => Literal::Int(left.get_int() - right.get_int()),
            Op::Mul => Literal::Int(left.get_int() * right.get_int()),
            _ => unimplemented!(),
        }
    }

    // The operator priority
    // Mul/Div has higher priority (binds harder) than Add/Sub
    pub fn priority(&self) -> u8 {
        match self {
            Op::Add => 0,
            Op::Sub => 0,
            Op::Mul => 1,
            Op::Div => 1,
            Op::And => 0,
            Op::Or => 0,
        }
    }
}
