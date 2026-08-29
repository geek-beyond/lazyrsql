use std::io::{self, Write, stdin, stdout};

pub struct Config {
    pub db_url: String,
    pub user: String,
    pub password: String,
}

fn trim_answer(value: &str) -> String {
    value.trim().to_string()
}

fn ask(question: &str) -> io::Result<String> {
    print!("{}", question);
    stdout().flush()?;

    let mut value = String::new();
    stdin().read_line(&mut value)?;

    Ok(trim_answer(&value))
}

pub fn ask_config() -> io::Result<Config> {
    Ok(Config {
        db_url: ask("Database URL: ")?,
        user: ask("User: ")?,
        password: ask("Password: ")?,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn trim_answer_strips_the_trailing_newline() {
        assert_eq!(trim_answer("mysql://localhost\n"), "mysql://localhost");
    }

    #[test]
    fn trim_answer_strips_a_windows_line_ending() {
        assert_eq!(trim_answer("mysql://localhost\r\n"), "mysql://localhost");
    }

    #[test]
    fn trim_answer_strips_leading_and_ending_whitespace() {
        assert_eq!(trim_answer("  indented  \n"), "indented");
    }

    #[test]
    fn trim_answer_keeps_inner_characters_untouched() {
        assert_eq!(trim_answer("pass word!\n"), "pass word!");
    }

    #[test]
    fn trim_answer_returns_empty_string_for_a_blank_line() {
        assert_eq!(trim_answer("\n"), "");
    }

    #[test]
    fn trim_answer_returns_empty_string_for_empty_input() {
        assert_eq!(trim_answer(""), "");
    }
}
