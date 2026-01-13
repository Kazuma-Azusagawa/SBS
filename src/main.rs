use std::env;
use std::fs;
use std::process::Command;

fn main() -> Result<(), Box<std::error::Error>> {
    let arg = env::args().nth(1).expect("No given file");
    let path = std::path::PathBuf(arg);
    let contents: String = fs::read_to_string(path)?;
    let cmd: Command;
    Ok(())
}
