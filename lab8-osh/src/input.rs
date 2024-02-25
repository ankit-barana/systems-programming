pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    Blank,
    Unquoted,
    SingleQuoted,
    DoubleQuoted,
    Digit(u32),
}

#[derive(PartialEq, Clone, Debug)]
pub enum InputToken {
    Pipe,
    Word(String),
    RedirectInput(u32, String),
    RedirectOutput(u32, String),
    RedirectAppend(u32, String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RedirectionState {
    None, // normal parsing
    // in all of the ones below, the u32 keeps track of which file descriptor will be redirected
    Input(u32),
    Output(u32),
    Append(u32),
}

pub fn split_input(input: &str) -> Result<Vec<InputToken>> {
    let mut result: Vec<InputToken> = Vec::new();
    let mut current_word: String = String::new();
    let mut state = State::Blank;
    let mut redirection_state: RedirectionState = RedirectionState::None;

    for ch in input.chars() {
        match (state, ch) {
            // if a pipleline char is encountered, we push a new pipe token onto the result vec
            (State::Blank, '|') => result.push(InputToken::Pipe),
            (State::Blank, _) if ch.is_ascii_whitespace() => (),
            (State::Blank, '\'') => state = State::SingleQuoted,
            (State::Blank, '"') => state = State::DoubleQuoted,
            (State::Blank, _) if ch.is_ascii_digit() => state = State::Digit(ch.to_digit(10).unwrap()),

            // redirections in blank state
            (State::Blank, '<') => {
                redirection_state = RedirectionState::Input(0);
                state = State::Unquoted;
            }
            (State::Blank, '>') => {
                redirection_state = RedirectionState::Output(1);
                state = State::Unquoted;
            }
            (State::Blank, _) => {
                current_word.push(ch);
                state = State::Unquoted;
            }

            // handles the digit states. First two for when we encounter a number with redirection
            (State::Digit(num), '<') => {
                redirection_state = RedirectionState::Input(num);
                state = State::Unquoted;
            }
            (State::Digit(num), '>') => {
                redirection_state = RedirectionState::Output(num);
                println!("{num}");
                state = State::Unquoted;
            }
            (State::Digit(num), '\'') => {
                current_word.push(char::from_digit(num, 10).unwrap());
                state = State::SingleQuoted;
            }
            (State::Digit(num), '"') => {
                current_word.push(char::from_digit(num, 10).unwrap());
                state = State::DoubleQuoted;
            }
            (State::Digit(num), _) => {
                current_word.push(char::from_digit(num, 10).unwrap());
                current_word.push(ch);
                state = State::Unquoted;
            }

            (State::Unquoted, _) if ch.is_ascii_whitespace() => {
                let token = match redirection_state {
                    RedirectionState::None => InputToken::Word(current_word),
                    _ if current_word.is_empty() => return Err("Empty redirection target".into()),
                    RedirectionState::Input(num) => InputToken::RedirectInput(num, current_word),
                    RedirectionState::Output(num) => InputToken::RedirectOutput(num, current_word),
                    RedirectionState::Append(num) => InputToken::RedirectAppend(num, current_word),
                };
                result.push(token);
                current_word = String::new();
                state = State::Blank;
                redirection_state = RedirectionState::None;
            }

            (State::Unquoted, '|') => {
                let token = match redirection_state {
                    RedirectionState::None => InputToken::Word(current_word),
                    _ if current_word.is_empty() => return Err("Empty redirection target".into()),
                    RedirectionState::Input(num) => InputToken::RedirectInput(num, current_word),
                    RedirectionState::Output(num) => InputToken::RedirectOutput(num, current_word),
                    RedirectionState::Append(num) => InputToken::RedirectAppend(num, current_word),
                };
                result.push(token);
                result.push(InputToken::Pipe);
                current_word = String::new();
                state = State::Blank;
                redirection_state = RedirectionState::None;
            }

            (State::Unquoted, '>') => {
                match redirection_state {
                    RedirectionState::None => {
                        result.push(InputToken::Word(current_word));
                        redirection_state = RedirectionState::Output(1);
                        current_word = String::new();
                    }

                    RedirectionState::Output(num) if current_word.is_empty() => {
                        redirection_state = RedirectionState::Append(num);
                    }

                    _ if current_word.is_empty() => {
                        return Err("Empty redirection target".into());
                    }

                    RedirectionState::Output(num) => {
                        result.push(InputToken::RedirectOutput(num, current_word));
                        current_word = String::new();
                        redirection_state = RedirectionState::Output(1);
                    },

                    RedirectionState::Input(num) => {
                        result.push(InputToken::RedirectInput(num, current_word));
                        current_word = String::new();
                        redirection_state = RedirectionState::Output(1);
                    },
                    RedirectionState::Append(num) => {
                        result.push(InputToken::RedirectAppend(num, current_word));
                        current_word = String::new();
                        redirection_state = RedirectionState::Output(1);
                    },
                };
            }

            (State::Unquoted, '<') => {
                match redirection_state {
                    RedirectionState::None => {
                        result.push(InputToken::Word(current_word));
                        current_word = String::new();
                    }
                    _ if current_word.is_empty() => {
                        return Err("Empty redirection target".into());
                    }
                    RedirectionState::Input(num) => {
                        result.push(InputToken::RedirectInput(num, current_word));
                        current_word = String::new();
                    },
                    RedirectionState::Output(num) => {
                        result.push(InputToken::RedirectOutput(num, current_word));
                        current_word = String::new();
                    },
                    RedirectionState::Append(num) => {
                        result.push(InputToken::RedirectAppend(num, current_word));
                        current_word = String::new();
                    },
                }
                redirection_state = RedirectionState::Input(0);
            }

            (State::Unquoted, '"') => state = State::DoubleQuoted,
            (State::Unquoted, '\'') => state = State::SingleQuoted,
            (State::Unquoted, _) => current_word.push(ch),

            (State::SingleQuoted, '\'') => state = State::Unquoted,
            (State::SingleQuoted, _) => current_word.push(ch),

            (State::DoubleQuoted, '"') => state = State::Unquoted,
            (State::DoubleQuoted, _) => current_word.push(ch),
        }
    }

    // handles the final state of the input
    match state {
        // if the ending state is unquoted, we add the current word token into the vector
        State::Unquoted => {
            // and rediection state is None, we cam just push an word token
            // otherwise, we push respective redirection token
            match redirection_state {
                RedirectionState::None => result.push(InputToken::Word(current_word)),
                _ if current_word.is_empty() => return Err("Empty redirection target".into()),
                RedirectionState::Input(num) => result.push(InputToken::RedirectInput(num, current_word)),
                RedirectionState::Output(num) => result.push(InputToken::RedirectOutput(num, current_word)),
                RedirectionState::Append(num) => result.push(InputToken::RedirectAppend(num, current_word)),
            }
        },
        State::Digit(num) => {
            current_word.push(char::from_digit(num, 10).unwrap());
            result.push(InputToken::Word(current_word));
        }
        // if the state is quoted, an error is returned
        State::SingleQuoted | State::DoubleQuoted => return Err("Unclosed quote".into()),
        _ => {}
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_pipeline() {
        let input = "echo hello | wc -c";
        let tokens = split_input(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                InputToken::Word("echo".to_string()),
                InputToken::Word("hello".to_string()),
                InputToken::Pipe,
                InputToken::Word("wc".to_string()),
                InputToken::Word("-c".to_string()),
            ]
        );
    }

    #[test]
    fn mixed_quotes() {
        // This is a "raw" string. It has delimiters r#" and "#.
        // Note that we can use " inside of it.
        let input = r#"'mixing different 'quoting" styles""#;
        let tokens = split_input(input).unwrap();
        assert_eq!(
            tokens,
            vec![InputToken::Word(
                "mixing different quoting styles".to_string()
            ),]
        );
    }

    #[test]
    fn redirections() {
        use InputToken::*;

        let input = ": >foo >>bar <baz";
        let tokens = split_input(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Word(":".to_string()),
                RedirectOutput(1, "foo".to_string()),
                RedirectAppend(1, "bar".to_string()),
                RedirectInput(0, "baz".to_string()),
            ]
        );
    }

    #[test] // fails
    fn redirections_nospace() {
        use InputToken::*;

        let input = ":>foo>>bar<baz";
        let tokens = split_input(input).unwrap();
        println!("{tokens:?}");
        assert_eq!(
            tokens,
            vec![
                Word(":".to_string()),
                RedirectOutput(1, "foo".to_string()),
                RedirectAppend(1, "bar".to_string()),
                RedirectInput(0, "baz".to_string()),
                // i am getting - [Word(":"), RedirectOutput(1, "foo"), RedirectAppend(1, "bar"), RedirectInput(1, "baz")]
            ]
        );
    }

    #[test] // fails
    fn digit_redirections() {
        use InputToken::*;

        let input = ": 1word 2>foo 3>>bar 4<baz 56>abc";
        let tokens = split_input(input).unwrap();
        println!("{tokens:?}");
        assert_eq!(
            tokens,
            vec![
                Word(":".to_string()),
                Word("1word".to_string()),
                RedirectOutput(2, "foo".to_string()),
                RedirectAppend(3, "bar".to_string()),
                RedirectInput(4, "baz".to_string()),
                Word("56".to_string()),
                RedirectOutput(1, "abc".to_string()),
                // i am getting - [Word(":"), Word("1word"), RedirectOutput(50, "foo"), RedirectAppend(51, "bar"), RedirectInput(52, "baz"), Word("56"), RedirectOutput(1, "abc")]
            ]
        );
    }

    #[test]
    fn empty_redirection_targets() {
        assert!(split_input("<").is_err());
        assert!(split_input(">").is_err());
        assert!(split_input(">>>foo").is_err());
        assert!(split_input("><foo").is_err());
        assert!(split_input("<>foo").is_err());
        assert!(split_input(">><foo").is_err());
    }
}