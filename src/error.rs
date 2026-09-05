use crate::config::ConfigError;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid Config: {0}")]
    Config(#[from] ConfigError),
}
