use parson::grammar;

pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Error,
}

#[derive(Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

grammar! {
  pub mod grammar;
  use [crate::{Expr, Opcode}, std::str::FromStr];

  pub Expr: Box<Expr> = {
      l:Expr op:ExprOp r:Factor => Box::new(Expr::Op(l, op, r)),
      f:Factor => f,
  };

  ExprOp: Opcode = {
      q:'+' => Opcode::Add,
      q:'-' => Opcode::Sub,
  };

  Factor: Box<Expr> = {
      l:Factor op:FactorOp r:Term => Box::new(Expr::Op(l, op, r)),
      t:Term => t,
  };

  FactorOp: Opcode = {
      q:'*' => Opcode::Mul,
      q:'/' => Opcode::Div,
  };

  Term: Box<Expr> = {
      n:Num => Box::new(Expr::Number(n)),
      p:'(' e:Expr q:')' => e,
  };

  Num: i32 = {
      n:r"[0-9]+" => i32::from_str(n).unwrap()
  };
}

fn main() {}
