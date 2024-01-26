use crate::lexer::{Token, TokenStream};

#[derive(Debug, PartialEq)]
pub struct Ast {
    token: Token,
    children: Vec<Ast>,
}

impl Ast {
    pub fn build(_tokens: TokenStream) -> Option<Ast> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    fn parse_simple() {
        let tokens = TokenStream::new(";play amogus");
        let expected_ast = Ast {
            token: Token::Command("play".to_string()),
            children: vec![Ast {
                token: Token::Argument("amogus".to_string()),
                children: vec![],
            }],
        };
        assert_eq!(Ast::build(tokens).unwrap(), expected_ast);
    }
}
