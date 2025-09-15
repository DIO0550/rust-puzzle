use thiserror::Error;

#[derive(Debug, Error)]
pub enum FileNameParseError {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid filename format: {0}")]
    InvalidFormat(String),

    #[error("Missing required component: {0}")]
    MissingComponent(&'static str),

    #[error("Invalid component value: {component}: {details}")]
    InvalidComponent {
        component: &'static str,
        details: String,
    },
}

pub trait FromFileName {
    fn from_file_name(file_name: &str) -> Result<Self, FileNameParseError>
    where
        Self: Sized;
}
