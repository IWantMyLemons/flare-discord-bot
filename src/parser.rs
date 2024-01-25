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

    #[test]
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

    #[test]
    fn parse_branching() {
        let tokens = TokenStream::new(";random 1 10 :|jump");
        let expected_ast = Ast {
            token: Token::Command("jump".to_string()),
            children: vec![Ast {
                token: Token::Command("random".to_string()),
                children: vec![
                    Ast {
                        token: Token::Argument("1".to_string()),
                        children: vec![],
                    },
                    Ast {
                        token: Token::Argument("10".to_string()),
                        children: vec![],
                    },
                ],
            }],
        };
        assert_eq!(Ast::build(tokens).unwrap(), expected_ast);
    }
}
