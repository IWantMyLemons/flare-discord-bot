use super::lexer;

pub struct Ast {
    token: lexer::Token,
    children: Vec<Ast>,
}

impl Ast {
    pub fn new(tokens: lexer::TokenStream) -> Ast {
        todo!()
    }
}
