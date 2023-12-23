use std::fmt;
use std::str::FromStr;

use parson::{grammar, Parser};

pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    // Error,
}

#[derive(Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Mul => '*',
                Self::Div => '/',
                Self::Add => '+',
                Self::Sub => '-',
            }
        )
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        match self {
            Self::Number(i) => write!(f, "{i}"),
            Self::Op(l, o, r) => write!(f, "{l:?} {o:?} {r:?}"),
        }?;
        write!(f, ")")
    }
}

grammar! {
  pub mod grammar;

  pub Expr: Box<Expr> = {
      l=Expr op=ExprOp r=Factor => Box::new(Expr::Op(l, op, r)),
      f=Factor => f,
  };

  ExprOp: Opcode = {
      _='+' => Opcode::Add,
      _='-' => Opcode::Sub,
  };

  Factor: Box<Expr> = {
      l=Factor op=FactorOp r=Term => Box::new(Expr::Op(l, op, r)),
      t=Term => t,
  };

  FactorOp: Opcode = {
      _='*' => Opcode::Mul,
      _='/' => Opcode::Div,
  };

  Term: Box<Expr> = {
      n=Num => Box::new(Expr::Number(n)),
      _='(' e=Expr _=')' => e,
  };

  Num: i32 = {
      n=r"[0-9]+" => i32::from_str(n).unwrap()
  };
}

fn main() {
    let example = "4 + 2 * 42";
    println!("{:?}", grammar::Expr.parse(example));
}
