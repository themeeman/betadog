use std::iter::{Peekable};
use super::expr::Const;

#[derive(Debug, PartialEq)]
pub struct Error {
    position: i64,
    message: String,
}

#[derive(Debug, PartialEq)]
pub enum Tok {
    Iden(String),
    Op(String),
    Lit(Const),
    LParen,
    RParen,
    Inf,
    Undef,
}

pub struct Lexer<'a> {
    current: Peekable<std::str::Chars<'a>>,
    index: i64,
}

impl Lexer<'_> {
    pub fn new(c: std::str::Chars<'_>) -> Lexer<'_> {
        Lexer {
            current: c.peekable(),
            index: 0,
        }
    }
}

impl Lexer<'_> {
    fn lex(&mut self) -> Result<Vec<Tok>, Error> {
        let mut toks = Vec::new();
        while let Some(current) = self.current.peek() {
            if *current == '(' {
                self.current.next();
                self.index += 1;
                toks.push(Tok::LParen);
            } else if *current == ')' {
                self.current.next();
                self.index += 1;
                toks.push(Tok::RParen);
            } else if current.is_digit(10) || *current == '.' {
                toks.push(self.literal()?);
            } else if current.is_whitespace() {
                self.current.next();
                self.index += 1;
            } else if current.is_alphabetic() {
                toks.push(self.iden());
            } else {
                toks.push(self.operator());
            }
        }
        Ok(toks)
    }

    fn literal(&mut self) -> Result<Tok, Error> {
        let mut s = String::new();
        let mut is_float = false;
        while let Some(current) = self.current.peek() {
            self.index += 1;
            if *current == '.' {
                if !is_float {
                    s.push(*current);
                    is_float = true; 
                    self.current.next();   
                } else {
                    return Err(Error{
                        message: String::from("Invalid numeric literal"),
                        position: self.index,
                    });
                }
            } else if current.is_digit(10) {
                s.push(*current);
                self.current.next();
            } else {
                break;
            }
        }

        if s.len() == 1 && s.chars().next().unwrap() == '.' {
            return Err(Error{
                message: String::from("Invalid numeric literal"),
                position: self.index,
            });
        }

        if is_float {
            Ok(Tok::Lit(Const::Float(
                match s.parse::<f64>() {
                    Ok(f) => f,
                    Err(_) => panic!("Unreachable panic in lexer. This is a bug.")
                }
            )))
        } else {
            Ok(Tok::Lit(Const::Int(
                match s.parse::<i128>() {
                    Ok(f) => f,
                    Err(_) => panic!("Unreachable panic in lexer. This is a bug.")
                }
            )))
        }
    }

    fn operator(&mut self) -> Tok {
        let mut s = String::new();
        while let Some(c) = self.current.peek() {
            if !c.is_alphanumeric() && !c.is_digit(10) && *c != '.' && !c.is_whitespace() && *c != ')' && *c != '(' {
                s.push(*c);
                self.index += 1;
                self.current.next();
            } else {
                break;
            }           
        }
        Tok::Op(s)
    }

    fn iden(&mut self) -> Tok {
        let mut s = String::new();
        while let Some(c) = self.current.peek() {
            if c.is_alphanumeric() {
                s.push(*c);
                self.index += 1;
                self.current.next();
            } else {
                break;
            }
        }
        
        if &s[..] == "inf" {
            Tok::Inf
        } else if &s[..] == "undef" {
            Tok::Undef
        } else {
            Tok::Iden(s)
        }
    }
}

pub fn lex(s: String) -> Result<Vec<Tok>, Error> {
    Lexer::new(s.chars()).lex()
}