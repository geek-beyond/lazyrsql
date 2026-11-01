use crate::db::ConnectionConfigError;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid Database Connection Config: {0}")]
    ConnectionConfig(#[from] ConnectionConfigError),
}
