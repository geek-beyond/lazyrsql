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
