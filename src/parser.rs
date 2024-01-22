use super::lexer;

#[derive(Debug, PartialEq)]
pub struct Ast {
    token: lexer::Token,
    children: Vec<Ast>,
}

impl Ast {
    pub fn new(tokens: lexer::TokenStream) -> Ast {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::Token;

    use super::*;

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
        assert_eq!(Ast::new(tokens), expected_ast);
    }
}
