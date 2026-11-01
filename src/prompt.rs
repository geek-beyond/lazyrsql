use std::io::{self, Write, stdin, stdout};

pub fn ask(question: &str) -> io::Result<String> {
    print!("{}", question);
    stdout().flush()?;

    let mut value = String::new();
    stdin().read_line(&mut value)?;

    Ok(value.trim().to_string())
}
