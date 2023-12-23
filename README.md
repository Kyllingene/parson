# Parson: a decl macro parser generator library

## Syntax

Parson's syntax is designed to be similar to LALRPOP, with some changes both
for the sake of the macro and because I find them helpful.

```rust
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
  pub Expr: Box<Expr> = {
      l:Expr op:ExprOp r:Factor => Box::new(Expr::Op(l, op, r)),
      Factor !,
  };

  ExprOp: Opcode = { // (3)
      '+' => Opcode::Add,
      '-' => Opcode::Sub,
  };

  Factor: Box<Expr> = {
      l:Factor op:FactorOp r:Term => Box::new(Expr::Op(l, op, r)),
      t:Term => t,
  };

  FactorOp: Opcode = {
      : '*' => Opcode::Mul,
      : '/' => Opcode::Div,
  };

  Term: Box<Expr> = {
      n:Num => Box::new(Expr::Number(n)),
      : '(' e:Expr : ')' => e,
  };

  Num: i32 = {
      n:r"[0-9]+" => i32::from_str(n).unwrap()
  };
}

```
