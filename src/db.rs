pub const URL_SCHEMES: &[&str] = &[
    "mysql://"
    // TODO: psqlを対応するようになったら追加する
    // "postgres://"
];

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ConnectionConfigError {
    #[error("databaseURL is empty")]
    EmptyDbUrl,

    #[error(
        "databaseURL scheme is not supported yet (currently only `mysql://`; other databases are planned): {0}"
    )]
    UnsupportedDbUrlScheme(String),

    #[error("user is empty")]
    EmptyUser,
}

pub fn validate_db_url(db_url: &str) -> Result<(), ConnectionConfigError> {
    if db_url.is_empty() {
        return Err(ConnectionConfigError::EmptyDbUrl);
    }

    if !URL_SCHEMES
        .iter()
        .any(|scheme| db_url.starts_with(scheme))
    {
        return Err(ConnectionConfigError::UnsupportedDbUrlScheme(
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
    #[case::unsupported_protocol_http(
        "http://localhost:8080",
        Err(ConnectionConfigError::UnsupportedDbUrlScheme("http://localhost:8080".to_string()))
    )]
    #[case::unsupported_protocol_no_protocol(
        "localhost:3306",
        Err(ConnectionConfigError::UnsupportedDbUrlScheme("localhost:3306".to_string()))
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
    #[case::unsupported_db_url_scheme(
        "http://localhost:8080",
        "user_name",
        ConnectionConfigError::UnsupportedDbUrlScheme("http://localhost:8080".to_string())
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
