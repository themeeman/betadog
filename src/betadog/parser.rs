use super::lexer::Tok;
use super::expr::{Expr, Const};
use std::collections::HashMap;
use std::slice::{Iter};
use std::iter::{Peekable};

#[derive(Debug, PartialEq)]
pub struct Error {
    message: String,
}

pub fn parse(toks: Vec<Tok>, ops: HashMap<String, i8>) -> Result<Expr, Error> {
    let mut parser = Parser{toks: toks.iter().peekable(), binary_ops: ops};
    let ast = parser.parse_expr();
    match ast {
        Ok(ast) => if !parser.is_done() {
                Err(Error{message: String::from("Unexpected tokens after parsing expression")})
            } else {
                Ok(ast)
            },
        Err(err) => Err(err),
    }
}

struct Parser<'a> {
    binary_ops: HashMap<String, i8>,
    toks: Peekable<Iter<'a, Tok>>,
}

impl Parser<'_> {
    fn is_done(&mut self) -> bool {
        match self.toks.peek() {
            Some(_) => false,
            None => true,
        }
    }

    fn parse_paren_expr(&mut self) -> Result<Expr, Error> {
        self.toks.next(); // eats LParen
        let expr = self.parse_expr()?;
        if let Some(tok) = self.toks.peek() {
            if let Tok::RParen = tok {
                self.toks.next(); // eats RParen
                return Ok(expr);
            }
    
        }
        Err(Error{message: String::from("Unexpected end of tokens while parsing")})
    }
    
    fn parse_expr(&mut self) -> Result<Expr, Error> {
        let lhs = self.parse_primary()?;
        if let Some(Tok::Op(op)) = self.toks.peek() {
            return match &op[..] {
                "+" | "-" => self.parse_sum_rhs(&lhs),
                "*" | "/" => { 
                    let p = self.parse_product_rhs(&lhs)?;
                    if let Some(Tok::Op(op)) = self.toks.peek() {
                        match &op[..] {
                            "+" | "-" => self.parse_sum_rhs(&p),
                            _ => Ok(p),
                        }
                    } else {
                        Ok(p)
                    }
                },
                _ => { 
                    let expr = self.parse_bin_op_rhs(&lhs)?;
                    if let Some(Tok::Op(op)) = self.toks.peek() {
                        match &op[..] {
                            "+" | "-" => self.parse_sum_rhs(&expr),
                            "*" | "/" => { 
                                let p = self.parse_product_rhs(&expr)?;
                                if let Some(Tok::Op(op)) = self.toks.peek() {
                                    match &op[..] {
                                        "+" | "-" => self.parse_sum_rhs(&p),
                                        _ => Ok(p),
                                    }
                                } else {
                                    Ok(p)
                                }
                            },
                            _ => Ok(expr),
                        }
                    } else {
                        Ok(expr)
                    }
                },
            };
        }
        Ok(lhs)
    }
    
    fn parse_sum_rhs(&mut self, lhs: &Expr) -> Result<Expr, Error> {
        let mut expr = vec![Box::new(lhs.clone())];
        macro_rules! f {
            ($rhs:expr) => {
                self.toks.next(); // Eats Op
                let rhs: Expr = $rhs;
                if let Some(Tok::Op(op)) = self.toks.peek() {
                    match &op[..] {
                        "+" | "-" => 
                            expr.push(Box::new(rhs)),
                        "*" | "/" => 
                            expr.push(Box::new(self.parse_product_rhs(&rhs)?)),
                        op => if self.binary_ops[op] >= self.binary_ops[&String::from("+")] {
                            expr.push(Box::new(self.parse_bin_op_rhs(&rhs)?));
                        } else {
                            return self.parse_bin_op_rhs(&Expr::Sum(expr));
                        },
                    }
                } else {
                    expr.push(Box::new(rhs));
                    return Ok(Expr::Sum(expr));
                };
            };
        }

        loop {
            println!("{}", Expr::Sum(expr.clone()));
            if let Some(Tok::Op(op)) = self.toks.peek() {
                match &op[..] {
                    "+" => {
                        f!(self.parse_primary()?);
                    },
                    "-" => {
                        f!(Expr::Neg(Box::new(self.parse_primary()?)));
                    },
                    _ => unreachable!(),
                };
            } else {
                return Ok(Expr::Sum(expr));
            }
        }
    }

    fn parse_product_rhs(&mut self, lhs: &Expr) -> Result<Expr, Error> {
        let mut expr = vec![Box::new(lhs.clone())];
        macro_rules! f {
            ($rhs:expr) => {
                self.toks.next(); // Eats Op
                let rhs: Expr = $rhs;
                if let Some(Tok::Op(op)) = self.toks.peek() {
                    match &op[..] {
                        "+" | "-" => {
                            expr.push(Box::new(rhs));
                            return Ok(Expr::Prod(expr));
                        },
                        "*" | "/" => 
                            expr.push(Box::new(rhs)),
                        op => if self.binary_ops[op] >= self.binary_ops[&String::from("*")] {
                            expr.push(Box::new(self.parse_bin_op_rhs(&rhs)?));
                        } else {
                            return self.parse_bin_op_rhs(&Expr::Prod(expr));
                        },
                    }
                } else {
                    expr.push(Box::new(rhs));
                    return Ok(Expr::Prod(expr));
                };
            };
        }

        loop {
            println!("{}", Expr::Prod(expr.clone()));
            if let Some(Tok::Op(op)) = self.toks.peek() {
                match &op[..] {
                    "*" => {
                        f!(self.parse_primary()?);
                    },
                    "/" => {
                        f!(Expr::Recipr(Box::new(self.parse_primary()?)));
                    },
                    _ => unreachable!(),
                };
            } else {
                return Ok(Expr::Prod(expr));
            }
        }
    }

    fn parse_bin_op_rhs(&mut self, lhs: &Expr)  -> Result<Expr, Error> {
        if let Some(Tok::Op(op)) = self.toks.peek() {
            let prec = match self.binary_ops.get(op) {
                Some(p) => *p,
                None => return Err(Error{message: format!("Unknown operator {}", op)}),
            };

            self.toks.next(); // Eats Op
            let rhs = self.parse_primary()?;

            let (next_op, next_prec) = if let Some(Tok::Op(next_op)) = self.toks.peek() {
                match self.binary_ops.get(next_op) {
                    Some(p) => (next_op, p),
                    None => return Err(Error{message: format!("Unknown operator {}", op)}),
                }
            } else {
                return Ok(Expr::new_binary(&op[..], lhs, &rhs).unwrap());
            };

            if let "+" | "-" | "*" | "/" = &next_op[..] {
                return Ok(Expr::new_binary(&op[..], lhs, &rhs).unwrap());
            }

            if prec > *next_prec {
                return self.parse_bin_op_rhs(&Expr::new_binary(op, lhs, &rhs).unwrap());
            } else {
                return Ok(Expr::new_binary(op, lhs, &self.parse_bin_op_rhs(&rhs)?).unwrap());
            }
        }
        unreachable!();
    }
    
    fn parse_primary(&mut self) -> Result<Expr, Error> {
        if let Some(tok) = self.toks.peek() {
            return match *tok {
                Tok::Inf => {
                    self.toks.next(); // Eats inf
                    Ok(Expr::Const(Const::Inf))
                },
                Tok::Undef => {
                    self.toks.next(); // Eats undef
                    Ok(Expr::Const(Const::Undef))
                }
                Tok::Lit(c) => { 
                    self.toks.next(); // Eats constant
                    Ok(Expr::Const(*c)) 
                },
                Tok::Op(op) => {
                    self.toks.next(); // Eats op
                    match Expr::new_unary(op, &self.parse_primary()?) {
                        Some(v) => Ok(v),
                        None => Err(Error{message: format!("Unknown unary operator {}", op)})
                    }
                },
                Tok::LParen => self.parse_paren_expr(),
                Tok::Iden(s) => {
                    self.toks.next(); // Eats iden
                    Ok(Expr::Var(s.clone()))
                },
                _ => Err(Error{message: String::from("Unexpected token")}),
            }
        }
        Err(Error{message: String::from("Unexpected end of tokens while parsing")})
    }
    
}

