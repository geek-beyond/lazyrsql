pub struct Config {
    pub db_url: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ConfigError {
    #[error("databaseURL is empty")]
    EmptyDbUrl,

    #[error("databaseURL is invalid (expected `mysql://`): {0}")]
    InvalidDbUrlScheme(String),

    #[error("user is empty")]
    EmptyUser,
}

impl Config {
    pub fn new(db_url: String, user: String, password: String) -> Result<Self, ConfigError> {
        validate_db_url(&db_url)?;
        validate_user(&user)?;
        Ok(Self {
            db_url,
            user,
            password,
        })
    }

    pub fn print(&self) {
        println!(
            "db_url: {}, user: {}, password: {}",
            self.db_url, self.user, self.password
        );
    }
}

fn validate_db_url(db_url: &str) -> Result<(), ConfigError> {
    if db_url.is_empty() {
        return Err(ConfigError::EmptyDbUrl);
    }

    if !db_url.starts_with("mysql://") {
        return Err(ConfigError::InvalidDbUrlScheme(db_url.to_string()));
    }

    Ok(())
}

fn validate_user(user: &str) -> Result<(), ConfigError> {
    if user.is_empty() {
        return Err(ConfigError::EmptyUser);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::simple("user_name", Ok(()))]
    #[case::empty("", Err(ConfigError::EmptyUser))]
    fn test_user_validate(#[case] user: &str, #[case] expect: Result<(), ConfigError>) {
        let validate_result = validate_user(user);
        assert_eq!(validate_result, expect)
    }

    #[rstest]
    #[case::simple("mysql://localhost:3306", Ok(()))]
    #[case::empty("", Err(ConfigError::EmptyDbUrl))]
    #[case::invalid_protocol_psql(
        "postgres://localhost:5432",
        Err(ConfigError::InvalidDbUrlScheme("postgres://localhost:5432".to_string()))
    )]
    #[case::invalid_protocol_http(
        "http://localhost:8080",
        Err(ConfigError::InvalidDbUrlScheme("http://localhost:8080".to_string()))
    )]
    #[case::invalid_protocol_no_protocol(
        "localhost:3306",
        Err(ConfigError::InvalidDbUrlScheme("localhost:3306".to_string()))
    )]
    fn test_db_url_validate(#[case] db_url: &str, #[case] expect: Result<(), ConfigError>) {
        let validate_result = validate_db_url(db_url);
        assert_eq!(validate_result, expect)
    }
}
