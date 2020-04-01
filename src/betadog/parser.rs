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
        self.parse_bin_op_rhs(0, lhs)
    }
    
    fn parse_bin_op_rhs(&mut self, expr_prec: i8, lhs: Expr)  -> Result<Expr, Error> {
        if let Some(Tok::Op(op)) = self.toks.peek() {
            let prec = match self.binary_ops.get(op) {
                Some(p) => *p,
                None => return Err(Error{message: format!("Unknown operator {}", op)}),
            };

            if prec < expr_prec {
                return Ok(lhs);
            }

            self.toks.next(); // Eats Op
            let rhs = self.parse_primary()?;

            let next_prec = if let Some(Tok::Op(next_op)) = self.toks.peek() {
                match self.binary_ops.get(next_op) {
                    Some(p) => p,
                    None => return Err(Error{message: format!("Unknown operator {}", op)}),
                }
            } else {
                return Ok(Expr::new_binary(&op[..], lhs, rhs).unwrap());
            };

            if prec >= *next_prec {
                self.parse_bin_op_rhs(expr_prec + 1, Expr::new_binary(op, lhs, rhs).unwrap())
            } else {
                Ok(Expr::new_binary(op, lhs, self.parse_bin_op_rhs(expr_prec + 1, rhs)?).unwrap())
            }
        }
        Ok(lhs)
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
                    match Expr::new_unary(op, self.parse_primary()?) {
                        Some(v) => Ok(v),
                        None => Err(Error{message: format!("Unknown unary operator {}", op)})
                    }
                },
                Tok::LParen => self.parse_paren_expr(),
                _ => Err(Error{message: String::from("Unexpected token")}),
            }
        }
        Err(Error{message: String::from("Unexpected end of tokens while parsing")})
    }
    
}

