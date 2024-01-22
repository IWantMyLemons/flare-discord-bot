#[derive(Debug, PartialEq, Clone)]
/// A token that usually makes up a word in a command
///
/// Normally it's formatted as
///
/// `Command Argument Argument --OptionalArgument --Named=Argument`
///
/// Additionally,
///
/// [`CmdToken::Pipe`] is derived from `:|`
///
/// [`CmdToken::FileStream`] is derived from `:>`
///
/// [`CmdToken::Seperator`] is derived from `;`
pub enum Token {
    Command(String),
    Argument(String),
    NamedArgument(String, String),
    OptionalArgument(String),
    Pipe,
    FileStream,
    Seperator,
}

#[derive(Debug, Default)]
pub struct TokenStream(Vec<Token>);

impl TokenStream {
    pub fn new(command: &str) -> TokenStream {
        let tokens = command
            .split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(lex_line)
            .reduce(|a, b| [a, b].join(&Token::Seperator))
            .unwrap_or(vec![]);

        TokenStream(tokens)
    }
}

fn lex_line(line: &str) -> Vec<Token> {
    let mut is_command_start = true;

    line.split(|c: char| c.is_whitespace())
        .filter(|s| !s.is_empty())
        .map(|word| {
            if is_command_start {
                is_command_start = false;
                return Token::Command(word.to_string());
            }
            if word == ":|" {
                is_command_start = true;
                return Token::Pipe;
            }
            if word == ":>" {
                return Token::FileStream;
            }

            if let Some(stripped_argument) = word.strip_prefix("--") {
                if word.contains(":=") {
                    let mut argument_iter = stripped_argument.splitn(2, ":=").map(String::from);
                    return Token::NamedArgument(
                        argument_iter.next().unwrap_or("".to_string()),
                        argument_iter.next().unwrap_or("".to_string()),
                    );
                }
                return Token::OptionalArgument(stripped_argument.to_string());
            }

            Token::Argument(word.to_string())
        })
        .collect::<Vec<Token>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        let tokens = TokenStream::new(";");
        assert!(tokens.0.is_empty());
    }
    #[test]
    fn ping() {
        let tokens = TokenStream::new(";ping");
        let expected_tokens = vec![Token::Command("ping".to_string())];
        assert_eq!(tokens.0, expected_tokens);
    }
    #[test]
    fn play_simple() {
        let tokens = TokenStream::new(";play amogus");
        let expected_tokens = vec![
            Token::Command("play".to_string()),
            Token::Argument("amogus".to_string()),
        ];
        assert_eq!(tokens.0, expected_tokens);
    }
    #[test]
    fn play_args() {
        let tokens = TokenStream::new(";play amogus --normalise --playback:=2.0");
        let expected_tokens = vec![
            Token::Command("play".to_string()),
            Token::Argument("amogus".to_string()),
            Token::OptionalArgument("normalise".to_string()),
            Token::NamedArgument("playback".to_string(), "2.0".to_string()),
        ];
        assert_eq!(tokens.0, expected_tokens);
    }
    #[test]
    fn play_pipes() {
        let tokens = TokenStream::new(";random 1 10 :| jump");
        let expected_tokens = vec![
            Token::Command("random".to_string()),
            Token::Argument("1".to_string()),
            Token::Argument("10".to_string()),
            Token::Pipe,
            Token::Command("jump".to_string()),
        ];
        assert_eq!(tokens.0, expected_tokens);
    }
    #[test]
    fn play_chaotic() {
        let tokens =
            TokenStream::new(";play\t amogus\t\n--normalise\t--playback:=2.0\n\n\t;\t \t;ping\n;");
        let expected_tokens = vec![
            Token::Command("play".to_string()),
            Token::Argument("amogus".to_string()),
            Token::OptionalArgument("normalise".to_string()),
            Token::NamedArgument("playback".to_string(), "2.0".to_string()),
            Token::Seperator,
            Token::Command("ping".to_string()),
        ];
        assert_eq!(tokens.0, expected_tokens);
    }
}
