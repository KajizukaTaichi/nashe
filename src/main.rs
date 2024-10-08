use std::{collections::HashMap, io, io::Write, process::Command};

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
        println!("{}", nashe.run_command(user_code));
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

struct Nashe {
    memory: HashMap<String, String>,
}

impl Nashe {
    fn run_command(&mut self, code: String) -> String {
        let code: Vec<String> = self.parse_args(code);
        let args = if code.len() < 2 {
            vec![]
        } else {
            code[1..code.len()].to_vec()
        };

        if code[0] == "cd" {
            if std::env::set_current_dir(if let Some(i) = args.get(0) {
                i
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
        if code[0] == "var" {
            self.memory.insert(
                if let Some(i) = args.get(0) {
                    i.to_owned()
                } else {
                    return "".to_string();
                },
                if let Some(i) = args.get(1) {
                    i.to_owned()
                } else {
                    return "".to_string();
                },
            );
            return "".to_string();
        }

        let result = Command::new(code[0].to_string()).args(args).output();
        if let Ok(output) = result {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                format!("{}", stdout.trim())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                format!("{}", stderr.trim())
            }
        } else {
            code.join("\n")
        }
    }

    fn parse_args(&mut self, source: String) -> Vec<String> {
        let tokens = {
            let mut tokens = Vec::new();
            let mut current_token = String::new();
            let mut in_parentheses: usize = 0;
            let mut in_quote = false;

            for c in source.chars() {
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

            if !current_token.is_empty() {
                tokens.push(current_token);
            }

            tokens
        };

        let mut result: Vec<String> = vec![];
        for token in tokens {
            let token = token.trim().to_string();
            let chars: Vec<char> = token.chars().collect();
            if chars[0] == '(' && chars[chars.len() - 1] == ')' {
                let inner_brace = String::from_iter(chars[1..chars.len() - 1].iter());
                result.push(self.run_command(inner_brace))
            } else if chars[0] == '$' {
                if let Some(i) = self.memory.get(&token.replacen("$", "", 1)) {
                    result.push(i.to_owned());
                } else {
                    result.push(token);
                }
            } else {
                result.push(token);
            }
        }
        result
    }
}
