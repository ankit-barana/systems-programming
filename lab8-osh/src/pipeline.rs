use crate::InputToken;
use crate::Result;
use std::fs::{File, OpenOptions};
use std::process::{Child, Stdio, Command};

#[derive(Clone, Debug, PartialEq)]
enum Redirection {
    Input(u32, String),
    Output(u32, String, bool), // bool = true means append output
}
#[derive(Clone, Debug, PartialEq)]
struct SimpleCommand {
    path: String,
    args: Vec<String>,
    redirections: Vec<Redirection>,
}

#[derive(Clone, Debug, PartialEq)]
pub (crate) struct Pipeline {
    commands: Vec<SimpleCommand>,
}

impl Pipeline {
    pub fn new(tokens: &[InputToken]) -> Result<Self> {
        // intialializes a vector of simple command
        let mut commands: Vec<SimpleCommand> = Vec::new();
        for command_tokens in tokens.split(|token| token == &InputToken::Pipe) {
            let mut words: Vec<String> = Vec::new();

            let mut redirections: Vec<Redirection> = Vec::new();

            for token in command_tokens {
                match token {
                    // matches word toek
                    InputToken::Word(some_string) =>  {
                        words.push(some_string.to_string());
                    }
                    InputToken::RedirectInput(fd, path) => redirections.push(Redirection::Input(*fd, path.to_string())),
                    InputToken::RedirectOutput(fd, path) => redirections.push(Redirection::Output(*fd, path.to_string(), false)),
                    InputToken::RedirectAppend(fd, path) => redirections.push(Redirection::Output(*fd, path.to_string(), true)),
                    InputToken::Pipe => return Err("Pipe encountered after splitting".into()),
                }
            }

            if words.is_empty() {
                return Err("Command missing".into());
            }

            let path = words.remove(0);
            let args = words;

            commands.push(SimpleCommand {path, args, redirections});
        }
        Ok(Pipeline {commands})
    }

    pub fn run(self) -> Result<()> {

        let mut children: Vec<Child>  = Vec::new();
        let mut last_stdout = Stdio::inherit();

        for (index, command) in self.commands.iter().enumerate() {
            let mut current_stdout = Stdio::piped();
            
            if index == self.commands.len() - 1 {
                current_stdout = Stdio::inherit();
            }

            // fd paramters
            let mut stdin = last_stdout;
            let mut stdout = current_stdout;
            let mut stderr = Stdio::inherit();

            for redirection in command.redirections.clone() {
                match redirection {
                    Redirection::Input(fd, path) => {
                        let file = File::open(path.clone()).map_err(|err| format!("{path}: {err}"))?;
                        match fd {
                            0 => stdin = file.into(),
                            _ => return Err(format!("Redirecting {} not supported", fd).into()),
                        }
                    }
                    Redirection::Output(fd, path, is_append) => {

                            let file = OpenOptions::new().write(true).create(true).truncate(!is_append).open(path.clone()).map_err(|err| format!("{}: {}", path, err))?;

                        match fd {
                            1 => stdout = file.into(),
                            2 => stderr = file.into(),
                            _ => return Err(format!("Redirecting {} not supported", fd).into()),
                        }
                    }
                }
            }


            let _path=command.path.clone();
            let argument=command.args.clone();

            let cmd = Command::new(_path)
            .args(argument)
            .stdin(stdin)
            .stdout(stdout )
            .stderr(stderr)
            .spawn();

            match cmd {
                Ok(mut child) => {
                    // extracts stdoutput and converts it into stdio
                    last_stdout = child.stdout.take().map_or(Stdio::null(), Stdio::from);
                    children.push(child);
                 
                }
                Err(err) => {
                    eprintln!("Error encountered during spawning: {}", err);
                    last_stdout = Stdio::null();
                }
            };
        }
        // Wait for all of the children that were created successfully.
        for mut child in children {
            child.wait()?;
        }
    Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_pipeline() {
        let tokens = [
            InputToken::Word("cat".to_string()),
            InputToken::Word("Cargo.toml".to_string()),
            InputToken::Pipe,
            InputToken::Word("wc".to_string()),
            InputToken::Word("-l".to_string()),
        ];
        let pipeline = Pipeline::new(&tokens).expect("Failed to create a Pipeline");
        assert_eq!(
            pipeline,
            Pipeline {
                commands: vec![
                    SimpleCommand {
                        path: "cat".to_string(),
                        args: vec!["Cargo.toml".to_string()],
                        redirections: vec![],
                    },
                    SimpleCommand {
                        path: "wc".to_string(),
                        args: vec!["-l".to_string()],
                        redirections: vec![],
                    },
                ]
            }
        );
    }


    #[test]
    fn output_pipeline() {
        let tokens = [
            InputToken::Word("cat".to_string()),
            InputToken::Word("Cargo.toml".to_string()),
            InputToken::RedirectOutput(2, "~/output.txt".to_string())
        ];
        let pipeline = Pipeline::new(&tokens).expect("Failed to create a Pipeline");
        assert_eq!(
            pipeline,
            Pipeline {
                commands: vec![
                    SimpleCommand {
                        path: "cat".to_string(),
                        args: vec!["Cargo.toml".to_string()],
                        redirections: vec![Redirection::Output(2, "~/output.txt".to_string(), false)],
                    },
                ]
            }
        );
    }

    #[test]
    fn append_pipeline() {
        let tokens = [
            InputToken::Word("cat".to_string()),
            InputToken::Word("Cargo.toml".to_string()),
            InputToken::RedirectAppend(2, "~/output.txt".to_string())
        ];
        let pipeline = Pipeline::new(&tokens).expect("Failed to create a Pipeline");
        assert_eq!(
            pipeline,
            Pipeline {
                commands: vec![
                    SimpleCommand {
                        path: "cat".to_string(),
                        args: vec!["Cargo.toml".to_string()],
                        redirections: vec![Redirection::Output(2, "~/output.txt".to_string(), true)],
                    },
                ]
            }
        );
    }

    #[test]
    fn io_pipeline() {
        let tokens = [
            InputToken::Word("cat".to_string()),
            InputToken::Word("Cargo.toml".to_string()),
            InputToken::RedirectAppend(2, "~/output.txt".to_string()),
            InputToken::RedirectInput(2, "~/output.txt".to_string()),
        ];
        let pipeline = Pipeline::new(&tokens).expect("Failed to create a Pipeline");
        assert_eq!(
            pipeline,
            Pipeline {
                commands: vec![
                    SimpleCommand {
                        path: "cat".to_string(),
                        args: vec!["Cargo.toml".to_string()],
                        redirections: vec![Redirection::Output(2, "~/output.txt".to_string(), true),
                                           Redirection::Input(2, "~/output.txt".to_string())],
                    },
                ]
            }
        );
    }

    #[test]
    fn redirections() {
        let tokens = [
            InputToken::Word(":".to_string()),
            InputToken::RedirectOutput(1, "foo".to_string()),
            InputToken::RedirectAppend(1, "bar".to_string()),
            InputToken::RedirectInput(0, "baz".to_string()),
        ];
        let pipeline = Pipeline::new(&tokens).expect("Failed to create a Pipeline");
        assert_eq!(
            pipeline,
            Pipeline {
                commands: vec![
                    SimpleCommand {
                        path: ":".to_string(),
                        args: vec![],
                        redirections: vec![Redirection::Output(1, "foo".to_string(), false),
                                           Redirection::Output(1, "bar".to_string(), true),
                                           Redirection::Input(0, "baz".to_string())],
                    },
                ]
            }
        );
    }

    #[test]
    fn digit_redirections() {
        let tokens = [
            InputToken::Word(":".to_string()),
            InputToken::Word("1word".to_string()),
            InputToken::RedirectOutput(2, "foo".to_string()),
            InputToken::RedirectAppend(3, "bar".to_string()),
            InputToken::RedirectInput(4, "baz".to_string()),
            InputToken::Word("56".to_string()),
            InputToken::RedirectOutput(1, "abc".to_string()),
        ];
        let pipeline = Pipeline::new(&tokens).expect("Failed to create a Pipeline");
        assert_eq!(
            pipeline,
            Pipeline {
                commands: vec![
                    SimpleCommand {
                        path: ":".to_string(),
                        args: vec!["1word".to_string(), "56".to_string()],
                        redirections: vec![Redirection::Output(2, "foo".to_string(), false),
                                           Redirection::Output(3, "bar".to_string(), true),
                                           Redirection::Input(4, "baz".to_string()),
                                           Redirection::Output(1, "abc".to_string(), false),],
                    },
                ]
            }
        );
    }
}
