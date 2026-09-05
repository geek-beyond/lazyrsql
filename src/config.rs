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

    #[test]
    fn test_new_stores_each_field() {
        let config = Config::new(
            "mysql://localhost:3306".to_string(),
            "user_name".to_string(),
            "password".to_string(),
        )
        .expect("valid config");

        assert_eq!(config.db_url, "mysql://localhost:3306");
        assert_eq!(config.user, "user_name");
        assert_eq!(config.password, "password");
    }

    #[rstest]
    #[case::empty_db_url("", "user_name", ConfigError::EmptyDbUrl)]
    #[case::invalid_db_url_scheme(
        "http://localhost:8080",
        "user_name",
        ConfigError::InvalidDbUrlScheme("http://localhost:8080".to_string())
    )]
    #[case::empty_user("mysql://localhost:3306", "", ConfigError::EmptyUser)]
    #[case::db_url_checked_before_user("", "", ConfigError::EmptyDbUrl)]
    fn test_new_returns_error(
        #[case] db_url: &str,
        #[case] user: &str,
        #[case] expect: ConfigError,
    ) {
        let result = Config::new(db_url.to_string(), user.to_string(), "password".to_string());

        assert_eq!(result.err(), Some(expect));
    }
}
