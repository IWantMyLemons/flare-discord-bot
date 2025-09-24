use std::{collections::HashMap, fmt::Debug, str::FromStr};

use serenity::all::CreateMessage;

#[derive(Debug)]
/// A state relating to arguments of a command
pub struct ArgState {
    positionals: Vec<String>,
    named: HashMap<String, String>,
}

impl ArgState {
    /// Parses a message to get it's arguments
    pub fn from_message(s: &str) -> Self {
        let args = &split_quotes(s)[1..];
        let mut res = Self {
            positionals: Vec::new(),
            named: HashMap::new(),
        };

        for arg in args {
            if let Some(arg_text) = arg.strip_prefix("--") {
                if let Some((key, value)) = arg_text.split_once('=') {
                    res.named.insert(key.to_string(), value.to_string());
                } else {
                    res.named.insert(arg_text.to_string(), "true".to_string());
                }
            } else {
                res.positionals.push(arg.to_string());
            }
        }
        
        res
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
fn split_quotes(s: &str) -> Vec<String> {
    #[derive(Debug, Default)]
    struct CharState {
        last_word: String,
        was_escaped: bool,
        is_quote: bool,
        was_whitespace: bool,
    }

    (s.to_string() + " ")
        .chars()
        .scan(CharState::default(), move |char_state, c| {
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
                char_state.last_word.push(c);
                char_state.was_whitespace = false;
                Some(None)
            }
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::handlers::message_binder::{split_quotes, ArgState};

    #[test]
    fn split_quotes_download() {
        let blocks = split_quotes(
            r##";download https://www.youtube.com/watch?v=8he5TcZ4Bn8 "#pooltoy #suit #toothless" --title="toothless suit i carnally want" --extract-audio"##,
        );

        assert_eq!(
            blocks,
            vec![
                ";download",
                "https://www.youtube.com/watch?v=8he5TcZ4Bn8",
                "#pooltoy #suit #toothless",
                r#"--title=toothless suit i carnally want"#,
                "--extract-audio",
            ]
        );
    }
    
    #[test]
    fn argstate_parsing() {
        let arg_state = ArgState::from_message(
            r##";download https://www.youtube.com/watch?v=8he5TcZ4Bn8 "#pooltoy #suit #toothless" --title="toothless suit i carnally want" --extract-audio"##,
        );

        println!("{arg_state:?}");

        assert_eq!(
            arg_state.positionals,
            vec![
                "https://www.youtube.com/watch?v=8he5TcZ4Bn8",
                "#pooltoy #suit #toothless",
            ]
        );

        assert_eq!(arg_state.named.get("title").unwrap(), "toothless suit i carnally want");
        assert_eq!(arg_state.named.get("extract-audio").unwrap(), "true");
    }
}
