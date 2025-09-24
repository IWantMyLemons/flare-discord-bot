use std::{collections::HashMap, fmt::Debug, str::FromStr};

use serenity::all::CreateMessage;

#[derive(Debug)]
/// A state relating to arguments of a command
pub struct ArgState<'a> {
    _positionals: Vec<&'a str>,
    _named: HashMap<&'a str, &'a str>,
}

impl<'a> ArgState<'a> {
    /// Parses a message to get it's arguments
    pub fn from_message(_s: &str) -> Self {
        todo!()
    }

    /// Attempts to get an argument, will otherwise return an error message
    pub fn bind<T>(&mut self, _name: &str, _position: usize) -> Result<T, Box<CreateMessage>>
    where
        T: FromStr,
    {
        todo!()
    }

    /// Checks if there's no arguments left, will return an error message
    pub fn finish(&self) -> Result<(), Box<CreateMessage>> {
        todo!()
    }
}

/// Splits a string into blocks, any text within a quotes `""` is one block
fn split_quotes(s: &str) -> impl Iterator<Item = String> {
    #[derive(Debug, Default)]
    struct CharState {
        last_word: String,
        was_escaped: bool,
        is_quote: bool,
        was_whitespace: bool,
    }

    (s.to_string() + " ")
        .char_indices()
        .scan(CharState::default(), move |char_state, (i, c)| {
            if char_state.was_escaped {
                char_state.was_escaped = false;
                Some(None)
            } else if c == '\\' {
                char_state.was_escaped = true;
                Some(None)
            } else if c == '"' {
                if char_state.is_quote {
                    char_state.is_quote = false;
                    Some(None)
                } else {
                    char_state.is_quote = true;
                    Some(None)
                }
            } else if c.is_whitespace() && !char_state.is_quote && !char_state.was_whitespace {
                let word = char_state.last_word.clone();
                char_state.last_word = String::new();
                char_state.was_whitespace = true;
                Some(Some(word))
            } else {
                char_state.was_whitespace = false;
                Some(None)
            }
        })
        .flatten()
}

#[cfg(test)]
mod tests {
    use crate::handlers::message_binder::split_quotes;

    #[test]
    fn split_quotes_download() {
        let mut blocks = split_quotes(
            r##";download https://www.youtube.com/watch?v=8he5TcZ4Bn8 "#pooltoy #suit #toothless" --title="toothless suit i carnally want" --extract-audio"##,
        );

        assert_eq!(blocks.next(), Some(";download".to_string()));
        assert_eq!(
            blocks.next(),
            Some("https://www.youtube.com/watch?v=8he5TcZ4Bn8".to_string())
        );
        assert_eq!(blocks.next(), Some("#pooltoy #suit #toothless".to_string()));
        assert_eq!(
            blocks.next(),
            Some(r#"--title=toothless suit i carnally want"#.to_string())
        );
        assert_eq!(blocks.next(), Some("--extract-audio".to_string()));
    }
}
