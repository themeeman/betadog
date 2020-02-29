pub mod eval;
pub mod expr;
pub mod lexer;
pub mod parser;
mod rat;

#[cfg(test)]
mod tests {
    use std::collections::{HashMap};
    use super::eval::*;
    use super::expr::*;
    use super::lexer::*;
    use super::parser::*;
    use super::rat::Rat;
    #[test]
    fn test_lexer() {
        let s = |x| String::from(x);
        use Tok::*;
        assert_eq!(lex(String::new()), Ok(Vec::new()));
        assert_eq!(lex(s("5")), Ok(vec![Lit(Const::from(5))]));
        assert_eq!(lex(s("3.14")), Ok(vec![Lit(Const::from(3.14))]));
        assert_eq!(lex(s("()")), Ok(vec![LParen, RParen]));
        assert!(lex(s("3.3.3.3")).is_err());
        assert!(lex(s(".")).is_err());
        assert_eq!(lex(s(".0")), Ok(vec![Lit(Const::from(0.0))]));
        assert_eq!(lex(s("0.")), Ok(vec![Lit(Const::from(0.0))]));
        assert_eq!(lex(s("+ - * / ")), Ok(vec![
            Op(s("+")), Op(s("-")), Op(s("*")), Op(s("/"))
        ]))
    }

    #[test]
    fn test_parser() {
        use self::Const::*;
        use Tok::*;
        use Expr::*;

        let ops = {
            let mut ops = HashMap::new();
            ops.insert(String::from("*"), 40);
            ops.insert(String::from("/"), 40);
            ops.insert(String::from("+"), 20);
            ops.insert(String::from("-"), 20);
            ops
        };
        let parse = |toks| parse(toks, ops.clone());
        assert!(parse(Vec::new()).is_err());
        assert_eq!(parse(vec![Lit(Int(5))]), Ok(Const(Int(5))));
    }

    #[test]
    fn test_rat() {
        let new_rat = |num, den| Rat::new(num, den);

        assert_eq!(new_rat(3, 5) + new_rat(2, 5), new_rat(1, 1));
        
        assert_eq!(new_rat(4, 9) + new_rat(9, 4), new_rat(97, 36));
        assert!(new_rat(4, 9) < new_rat(1, 2));
        assert_eq!(new_rat(1, 2), new_rat(2, 4));
        assert_eq!(new_rat(-1, 2).num(), -1);
    }
}
