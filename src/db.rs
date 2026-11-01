const URL_SCHEMES: &[&str] = &[
    "mysql://",
    // TODO: psqlやsqliteを対応するようになったら、PLANNED_URL_SCHEMESからここに移す
];

const PLANNED_URL_SCHEMES: &[&str] = &["postgres://", "sqlite://"];

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ConnectionConfigError {
    #[error("databaseURL is empty")]
    EmptyDbUrl,

    #[error("databaseURL is invalid (expected `mysql://`): {0}")]
    InvalidDbUrlScheme(String),

    #[error("databaseName is empty")]
    EmptyDbName,

    #[error("this database is not supported yet (planned for a future release): {0}")]
    PlannedDbUrlScheme(String),

    #[error("user is empty")]
    EmptyUser,
}

fn validate_db_url(db_url: &str) -> Result<(), ConnectionConfigError> {
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

fn validate_user(user: &str) -> Result<(), ConnectionConfigError> {
    if user.is_empty() {
        return Err(ConnectionConfigError::EmptyUser);
    }

    Ok(())
}

fn validate_db_name(db_name: &str) -> Result<(), ConnectionConfigError> {
    if db_name.is_empty() {
        return Err(ConnectionConfigError::EmptyDbName);
    }

    Ok(())
}

// 使用していない変数はClippyのwarningになるため、一時的に`_`をつける
pub fn connect(
    db_url: &str,
    user: &str,
    _password: &str,
    db_name: &str,
) -> Result<(), ConnectionConfigError> {
    validate_db_url(db_url)?;
    validate_user(user)?;
    validate_db_name(db_name)?;

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

    #[rstest]
    #[case::simple("database_name", Ok(()))]
    #[case::empty("", Err(ConnectionConfigError::EmptyDbName))]
    fn test_db_name_validate(
        #[case] db_name: &str,
        #[case] expect: Result<(), ConnectionConfigError>,
    ) {
        let validate_result = validate_db_name(db_name);
        assert_eq!(validate_result, expect)
    }

    #[test]
    fn test_connect_accepts_valid_config() {
        let result = connect(
            "mysql://localhost:3306",
            "user_name",
            "password",
            "database_name",
        );

        assert_eq!(result, Ok(()));
    }

    #[rstest]
    #[case::empty_db_url("", "user_name", "database_name", ConnectionConfigError::EmptyDbUrl)]
    #[case::invalid_db_url_scheme(
        "http://localhost:8080",
        "user_name",
        "database_name",
        ConnectionConfigError::InvalidDbUrlScheme("http://localhost:8080".to_string())
    )]
    #[case::empty_user(
        "mysql://localhost:3306",
        "",
        "database_name",
        ConnectionConfigError::EmptyUser
    )]
    #[case::empty_db_name(
        "mysql://localhost:3306",
        "user_name",
        "",
        ConnectionConfigError::EmptyDbName
    )]
    #[case::db_url_checked_before_user("", "", "database_name", ConnectionConfigError::EmptyDbUrl)]
    #[case::user_checked_before_db_name(
        "mysql://localhost:3306",
        "",
        "",
        ConnectionConfigError::EmptyUser
    )]
    fn test_connect_returns_error(
        #[case] db_url: &str,
        #[case] user: &str,
        #[case] db_name: &str,
        #[case] expect: ConnectionConfigError,
    ) {
        let result = connect(db_url, user, "password", db_name);

        assert_eq!(result, Err(expect));
    }
}
