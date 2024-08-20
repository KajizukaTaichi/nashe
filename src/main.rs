use std::{collections::HashMap, io, io::Write, process::Command};

struct Nashe {
    memory: HashMap<String, String>,
}

impl Nashe {
    fn eval_code(&mut self, code: String) {
        let mut splited: Vec<&str> = code.split("=").collect();
        if splited.len() == 1 {
            println!("{}", self.run_command(splited[0].to_string()));
        } else {
            let identify: String = splited[0].trim().to_string();
            let result = self.run_command({
                splited.remove(0);
                splited.join("=")
            });
            self.memory.insert(identify, result);
        }
    }

    fn parse_args(&mut self, source: String) -> Vec<String> {
        fn tokenize_args(input: String) -> Vec<String> {
            let mut tokens = Vec::new();
            let mut current_token = String::new();
            let mut in_parentheses: usize = 0;
            let mut in_quote = false;

            for c in input.chars() {
                match c {
                    '(' if !in_quote => {
                        if in_parentheses != 0 {
                            in_parentheses += 1;
                            current_token.push(c);
                        } else {
                            if !current_token.is_empty() {
                                tokens.push(current_token.clone());
                                current_token.clear();
                            }
                            in_parentheses += 1;
                            current_token.push(c);
                        }
                    }
                    ')' if !in_quote => {
                        if in_parentheses != 0 {
                            current_token.push(c);
                            in_parentheses -= 1;
                            if in_parentheses == 0 {
                                tokens.push(current_token.clone());
                                current_token.clear();
                            }
                        } else {
                            panic!("チノちゃん「うるさいですね...」");
                        }
                    }
                    '"' => {
                        if in_parentheses == 0 {
                            if in_quote {
                                current_token.push(c);
                                in_quote = false;
                                tokens.push(current_token.clone());
                                current_token.clear();
                            } else {
                                in_quote = true;
                                current_token.push(c);
                            }
                        } else {
                            current_token.push(c);
                        }
                    }
                    ' ' | '\n' | '\t' | '\r' | '　' => {
                        if in_parentheses != 0 || in_quote {
                            current_token.push(c);
                        } else {
                            if !current_token.is_empty() {
                                tokens.push(current_token.clone());
                                current_token.clear();
                            }
                        }
                    }
                    _ => {
                        current_token.push(c);
                    }
                }
            }

            if in_parentheses != 0 {
                panic!("チノちゃん「うるさいですね...」");
            }
            if in_quote {
                panic!("チノちゃん「うるさいですね...」");
            }

            if !current_token.is_empty() {
                tokens.push(current_token);
            }

            tokens
        }

        let tokens = tokenize_args(source);
        let mut result: Vec<String> = vec![];
        for token in tokens {
            let token = token.trim().to_string();
            let chars: Vec<char> = token.chars().collect();
            if chars[0] == '(' && chars[chars.len() - 1] == ')' {
                let inner_brace = String::from_iter(chars[1..chars.len() - 1].iter());
                result.push(self.run_command(inner_brace))
            } else if let Some(i) = self.memory.get(&token) {
                result.push(i.to_owned());
            } else {
                result.push(token);
            }
        }
        result
    }

    fn run_command(&mut self, code: String) -> String {
        let code: Vec<String> = self.parse_args(code);
        let args = if code.is_empty() {
            vec![]
        } else {
            code[1..code.len()].to_vec()
        };
        if code[0] == "cd" {
            if std::env::set_current_dir(if let Some(i) = args.get(0) {
                i.trim()
            } else {
                return "".to_string();
            })
            .is_err()
            {
                return format!("cd: {}: no such file or directory\n", args[0]);
            } else {
                return "".to_string();
            };
        }

        let result = Command::new(code[0].to_string()).args(args).output();
        if let Ok(output) = result {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                format!("{stdout}")
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                format!("{stderr}")
            }
        } else {
            code.join("\n")
        }
    }
}

/// Get standard input
fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut result = String::new();
    io::stdin().read_line(&mut result).ok();
    result.trim().to_string()
}

fn main() {
    println!("Nashe");
    let mut nashe = Nashe {
        memory: HashMap::new(),
    };
    loop {
        let user_code = input("> ");
        if user_code.is_empty() {
            continue;
        }
        nashe.eval_code(user_code)
    }
}
