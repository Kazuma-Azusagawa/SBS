use std::env;
use std::fmt;
use std::fs;
use std::io::Write;
//use std::os::unix::process::CommandExt;
use std::process::Command;

#[derive(Debug, Copy, Clone)]
enum TokenType {
    None,
    Compiler,
    MainFile,
    HeaderFiles,
    Option,
    Output,
    Flags,
}
#[derive(Debug)]
struct Token {
    t_type: TokenType,
    value: String,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            t_type: TokenType::None,
            value: Default::default(),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "value: {}, token type: {:?}", self.value, self.t_type)
    }
}

fn tokenize<'a>(contents: &str) -> Vec<Token> {
    let mut token_buffer: Token = Default::default();
    let mut tokens: Vec<Token> = Default::default();
    let mut buffer: String = Default::default();
    let mut u_value: bool = false;
    for c in contents.chars() {
        match c {
            '\'' => {
                if u_value {
                    tokens.push(token_buffer);
                    token_buffer = Default::default();
                }
                u_value = !u_value;
            }
            ' ' => {
                if !u_value {
                    match buffer.to_lowercase().as_str() {
                        "compiler" => token_buffer.t_type = TokenType::Compiler,
                        "option" => token_buffer.t_type = TokenType::Option,
                        "main_file" => token_buffer.t_type = TokenType::MainFile,
                        "header_files" => token_buffer.t_type = TokenType::HeaderFiles,
                        "flags" => token_buffer.t_type = TokenType::Flags,
                        "output" => token_buffer.t_type = TokenType::Output,
                        _ => eprintln!("ERROR <{buffer}> isn`t a Key word"),
                    }
                    buffer = Default::default();
                }
            }
            _ => {
                if u_value && c != '\'' {
                    token_buffer.value.push(c);
                }
                if !c.is_whitespace() && !u_value {
                    buffer.push(c);
                }
            }
        }
    }
    return tokens;
}

fn generate(tokens: Vec<Token>) -> String {
    let mut c_code: String = "#include <stdlib.h>\nint main(void){system(\"".to_string();
    for token in tokens {
        match token.t_type {
            TokenType::Compiler => {
                c_code.push_str(token.value.as_str());
            }
            TokenType::Option => {
                c_code.push(' ');
                c_code.push_str(token.value.as_str());
            }

            TokenType::MainFile => {
                c_code.push(' ');
                c_code.push_str(token.value.as_str());
            }
            TokenType::HeaderFiles => {
                c_code.push(' ');
                c_code.push_str(token.value.as_str());
            }
            TokenType::Flags => {
                c_code.push(' ');
                c_code.push_str(token.value.as_str());
            }
            TokenType::Output => {
                c_code.push_str(" -o ");
                c_code.push_str(token.value.as_str());
            }
            _ => {
                eprintln!("ERROR {token}");
            }
        }
    }
    c_code.push_str("\");\n return 0;}");
    return c_code;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg = env::args().nth(1).expect("No given file");
    let path = std::path::PathBuf::from(arg);
    let contents: String = fs::read_to_string(&path)?;
    let c_code: String = generate(tokenize(&contents));
    let dir = path.parent().unwrap();
    let c_file = dir.join("sbs.c");
    let mut file = fs::File::create(&c_file)?;
    file.write_all(c_code.as_bytes())?;
    Command::new("gcc")
        .args(["-o", "build", c_file.to_str().unwrap()])
        .spawn()?;
    Ok(())
}
