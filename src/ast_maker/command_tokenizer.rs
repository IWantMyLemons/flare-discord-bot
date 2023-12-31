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
pub enum CmdToken {
    Command(String),
    Argument(String),
    NamedArgument(String, String),
    OptionalArgument(String),
    Pipe,
    FileStream,
    Seperator,
}

/// Turns the content of a message into a `Vec<CmdToken>`
pub fn tokenize(command_content: String) -> Vec<CmdToken> {
    let lines = command_content
        .split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty());

    lines
        .map(|line| {
            let mut is_command_start = true;
            let words = line
                .split(|c: char| c.is_whitespace())
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(String::from);
            words
                .map(|argument| convert_to_token(argument, &mut is_command_start))
                .collect::<Vec<_>>()
        })
        .reduce(|a, b| [a, b].join(&CmdToken::Seperator))
        .unwrap()
}

/// internal function to convert a word into a token, is_command_start is an external bool
fn convert_to_token(argument: String, is_command_start: &mut bool) -> CmdToken {
    if *is_command_start {
        *is_command_start = false;
        return CmdToken::Command(argument);
    }
    if argument == ":|" {
        *is_command_start = true;
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
            "  \t;play\t amogus\t\n--normalise\t--playback:=2.0\n\n\t;\t \t;\n;",
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
