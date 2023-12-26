#[derive(Debug, PartialEq, Clone)]
pub enum CmdToken {
    Command(String),
    Argument(String),
    NamedArgument(String, String),
    OptionalArgument(String),
    Pipe,
    FileStream,
    Seperator,
}

pub fn tokenize(command_content: String) -> Vec<CmdToken> {
    let lines = command_content
        .strip_prefix(';')
        .unwrap_or(&command_content)
        .split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    let mut token_lines = Vec::new();

    for line in lines {
        let mut start_of_command = true;
        let words = line
            .split(|c: char| c.is_whitespace())
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(String::from);

        let token_line = words
            .map(|argument| {
                if start_of_command {
                    start_of_command = false;
                    return CmdToken::Command(argument);
                }
                if argument == ":|" {
                    start_of_command = true;
                    return CmdToken::Pipe;
                }
                if argument == ":>" {
                    return CmdToken::FileStream;
                }

                if let Some(stripped_argument) = argument.strip_prefix("--") {
                    if argument.contains(":=") {
                        let mut argument_iter = stripped_argument.splitn(2, ":=").map(String::from);
                        return CmdToken::NamedArgument(
                            argument_iter.next().unwrap(),
                            argument_iter.next().unwrap(),
                        );
                    }
                    return CmdToken::OptionalArgument(String::from(stripped_argument));
                }

                CmdToken::Argument(argument)
            })
            .collect::<Vec<_>>();
        token_lines.push(token_line);
    }

    token_lines.join(&CmdToken::Seperator)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ping() {
        let tokens = tokenize(String::from(";ping"));
        let expected_tokens = vec![CmdToken::Command(String::from("ping"))];
        assert_eq!(tokens, expected_tokens);
    }
    #[test]
    fn play_simple() {
        let tokens = tokenize(String::from(";play amogus"));
        let expected_tokens = vec![
            CmdToken::Command(String::from("play")),
            CmdToken::Argument(String::from("amogus")),
        ];
        assert_eq!(tokens, expected_tokens);
    }
    #[test]
    fn play_args() {
        let tokens = tokenize(String::from(";play amogus --normalise --playback:=2.0"));
        let expected_tokens = vec![
            CmdToken::Command(String::from("play")),
            CmdToken::Argument(String::from("amogus")),
            CmdToken::OptionalArgument(String::from("normalise")),
            CmdToken::NamedArgument(String::from("playback"), String::from("2.0")),
        ];
        assert_eq!(tokens, expected_tokens);
    }
    #[test]
    fn play_pipes() {
        let tokens = tokenize(String::from(";random 1 10 :| jump"));
        let expected_tokens = vec![
            CmdToken::Command(String::from("random")),
            CmdToken::Argument(String::from("1")),
            CmdToken::Argument(String::from("10")),
            CmdToken::Pipe,
            CmdToken::Command(String::from("jump")),
        ];
        assert_eq!(tokens, expected_tokens);
    }
    #[test]
    fn play_chaotic() {
        let tokens = tokenize(String::from(
            "  \t;play\t amogus\t\t--normalise\t--playback:=2.0\t;\t \t;;",
        ));
        let expected_tokens = vec![
            CmdToken::Command(String::from("play")),
            CmdToken::Argument(String::from("amogus")),
            CmdToken::OptionalArgument(String::from("normalise")),
            CmdToken::NamedArgument(String::from("playback"), String::from("2.0")),
        ];
        assert_eq!(tokens, expected_tokens);
    }
}
