pub const URL_SCHEMES: &[&str] = &[
    "mysql://",
    // TODO: psqlやsqliteを対応するようになったら、PLANNED_URL_SCHEMESからここに移す
];

pub const PLANNED_URL_SCHEMES: &[&str] = &["postgres://", "sqlite://"];

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ConnectionConfigError {
    #[error("databaseURL is empty")]
    EmptyDbUrl,

    #[error("databaseURL is invalid (expected `mysql://`): {0}")]
    InvalidDbUrlScheme(String),

    #[error("this database is not supported yet (planned for a future release): {0}")]
    PlannedDbUrlScheme(String),

    #[error("user is empty")]
    EmptyUser,
}

pub fn validate_db_url(db_url: &str) -> Result<(), ConnectionConfigError> {
    if db_url.is_empty() {
        return Err(ConnectionConfigError::EmptyDbUrl);
    }

    if PLANNED_URL_SCHEMES
        .iter()
        .any(|scheme| db_url.starts_with(scheme))
    {
        return Err(ConnectionConfigError::PlannedDbUrlScheme(
            db_url.to_string(),
        ));
    }

    if !URL_SCHEMES.iter().any(|scheme| db_url.starts_with(scheme)) {
        return Err(ConnectionConfigError::InvalidDbUrlScheme(
            db_url.to_string(),
        ));
    }

    Ok(())
}

pub fn validate_user(user: &str) -> Result<(), ConnectionConfigError> {
    if user.is_empty() {
        return Err(ConnectionConfigError::EmptyUser);
    }

    Ok(())
}

pub fn connect(db_url: &str, user: &str, password: &str) -> Result<(), ConnectionConfigError> {
    validate_db_url(db_url)?;
    validate_user(user)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::simple("user_name", Ok(()))]
    #[case::empty("", Err(ConnectionConfigError::EmptyUser))]
    fn test_user_validate(#[case] user: &str, #[case] expect: Result<(), ConnectionConfigError>) {
        let validate_result = validate_user(user);
        assert_eq!(validate_result, expect)
    }

    #[rstest]
    #[case::simple("mysql://localhost:3306", Ok(()))]
    #[case::empty("", Err(ConnectionConfigError::EmptyDbUrl))]
    #[case::planned_protocol_psql(
        "postgres://localhost:5432",
        Err(ConnectionConfigError::PlannedDbUrlScheme("postgres://localhost:5432".to_string()))
    )]
    #[case::planned_protocol_sqlite(
        "sqlite://./test.db",
        Err(ConnectionConfigError::PlannedDbUrlScheme("sqlite://./test.db".to_string()))
    )]
    #[case::invalid_protocol_http(
        "http://localhost:8080",
        Err(ConnectionConfigError::InvalidDbUrlScheme("http://localhost:8080".to_string()))
    )]
    #[case::invalid_protocol_no_protocol(
        "localhost:3306",
        Err(ConnectionConfigError::InvalidDbUrlScheme("localhost:3306".to_string()))
    )]
    fn test_db_url_validate(
        #[case] db_url: &str,
        #[case] expect: Result<(), ConnectionConfigError>,
    ) {
        let validate_result = validate_db_url(db_url);
        assert_eq!(validate_result, expect)
    }

    #[test]
    fn test_connect_accepts_valid_config() {
        let result = connect("mysql://localhost:3306", "user_name", "password");

        assert_eq!(result, Ok(()));
    }

    #[rstest]
    #[case::empty_db_url("", "user_name", ConnectionConfigError::EmptyDbUrl)]
    #[case::invalid_db_url_scheme(
        "http://localhost:8080",
        "user_name",
        ConnectionConfigError::InvalidDbUrlScheme("http://localhost:8080".to_string())
    )]
    #[case::empty_user("mysql://localhost:3306", "", ConnectionConfigError::EmptyUser)]
    #[case::db_url_checked_before_user("", "", ConnectionConfigError::EmptyDbUrl)]
    fn test_connect_returns_error(
        #[case] db_url: &str,
        #[case] user: &str,
        #[case] expect: ConnectionConfigError,
    ) {
        let result = connect(db_url, user, "password");

        assert_eq!(result, Err(expect));
    }
}
