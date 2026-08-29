use std::io::{self, Write, stdin, stdout};

pub struct Config {
    db_url: String,
    user: String,
    password: String
}

fn ask(question: &str) -> io::Result<String> {
    print!("{}", question);
    stdout().flush()?;

    let mut value = String::new();
    stdin().read_line(&mut value)?;

    Ok(String::from(value.trim_end()))
}

pub fn ask_config() -> io::Result<Config> {
    Ok(
        Config {
            db_url: ask("Database URL: ")?,
            user: ask("User: ")?,
            password: ask("Password: ")?,
        }
    )
}
