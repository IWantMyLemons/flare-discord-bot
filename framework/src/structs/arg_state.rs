use std::{collections::HashMap, fmt::Debug, str::FromStr};

use serenity::all::{Color, CreateEmbed, CreateMessage};

#[derive(Debug)]
/// A state relating to arguments of a command
pub struct ArgState {
    positionals: HashMap<usize, String>,
    named: HashMap<String, String>,
}

impl ArgState {
    /// Parses a message to get it's arguments
    pub fn from_message(s: &str, _func_args: &[&str]) -> Self {
        let args = &split_quotes(s)[1..];
        let mut res = Self {
            positionals: HashMap::new(),
            named: HashMap::new(),
        };

        let mut i_positional = 0;

        for arg in args {
            if let Some(arg_text) = arg.strip_prefix("--") {
                if let Some((key, value)) = arg_text.split_once('=') {
                    res.named.insert(key.to_string(), value.to_string());
                } else {
                    res.named.insert(arg_text.to_string(), "true".to_string());
                }
            } else {
                res.positionals.insert(i_positional, arg.to_string());
                i_positional += 1;
            }
        }

        res
    }

    /// Attempts to get an argument, will otherwise return an error message
    pub fn bind<T, E>(&mut self, name: &str, position: usize) -> Result<T, Box<CreateMessage>>
    where
        T: FromStr<Err = E>,
        E: ToString,
    {
        if let Some(arg) = self.named.remove(name) {
            arg.parse()
                .map_err(|e: E| Box::new(err_message(e.to_string())))
        } else if let Some(arg) = self.positionals.remove(&position) {
            arg.parse()
                .map_err(|e: E| Box::new(err_message(e.to_string())))
        } else {
            Err(Box::new(err_message(format!("Unmatched argument: {name}"))))
        }
    }

    /// Checks if there's no arguments left, will return an error message otherwise
    pub fn finish(&self) -> Result<(), Box<CreateMessage>> {
        if self.named.is_empty() && self.positionals.is_empty() {
            Ok(())
        } else {
            let positional_extras = self
                .positionals
                .values()
                .cloned()
                .reduce(|a, b| format!("{a} {b}"))
                .unwrap_or(String::new());
            let named_extras = self
                .named
                .iter()
                .map(|(k, v)| format!("--{k}={v}"))
                .reduce(|a, b| format!("{a} {b}"))
                .unwrap_or(String::new());
            let extras = format!("{positional_extras} {named_extras}");
            Err(Box::new(err_message(format!(
                "Too many arguments :/ ({extras})"
            ))))
        }
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

/// Generates an error message containing the string
fn err_message(s: String) -> CreateMessage {
    let embed = CreateEmbed::new()
        .color(Color::RED)
        .title("Error parsing command :/")
        .description(s);
    CreateMessage::new().embed(embed)
}

#[cfg(test)]
mod tests {
    use super::{ArgState, split_quotes};

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
            &["link", "tags", "title", "extract-audio"]
        );

        println!("{arg_state:?}");

        assert_eq!(
            arg_state.positionals.get(&0).unwrap(),
            "https://www.youtube.com/watch?v=8he5TcZ4Bn8"
        );


        assert_eq!(
            arg_state.positionals.get(&1).unwrap(),
            "#pooltoy #suit #toothless"
        );

        assert_eq!(
            arg_state.named.get("title").unwrap(),
            "toothless suit i carnally want"
        );
        assert_eq!(arg_state.named.get("extract-audio").unwrap(), "true");
    }
}
