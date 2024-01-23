use super::lexer;

#[derive(Debug, PartialEq)]
pub struct Ast {
    token: lexer::Token,
    children: Vec<Ast>,
}

impl Ast {
    pub fn build(mut _tokens: lexer::TokenStream) -> Option<Ast> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexer::Token;

    #[test]
    fn parse_simple() {
        let tokens = lexer::TokenStream::new(";play amogus");
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
