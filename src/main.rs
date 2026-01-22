use std::env;
use std::fs;
use std::io::Write;
//use std::process::Command;

enum TokenType {
    Cmp,
    MainFile,
    Header,
    Option,
    Name,
}

struct Token {
    t_type: TokenType,
    value: String,
}

fn tokenize(contents: String) -> Vec<Token> {
    let tokens: Vec<Token>;
    let mut buffer: String = Default::default();

    return tokens;
}
fn parse(tokens: Vec<Token>) -> String {
    let mut c_code: String = Default::default();

    return c_code;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg = env::args().nth(1).expect("No given file");
    let path = std::path::PathBuf::from(arg);
    let contents: String = fs::read_to_string(&path)?;
    let c_code: String = parse(tokenize(contents));
    let dir = path.parent().unwrap();
    let mut file = fs::File::create_new(dir.to_str().unwrap().to_string() + "sbs.c")?;
    file.write_all(c_code.as_bytes())?;

    Ok(())
}
