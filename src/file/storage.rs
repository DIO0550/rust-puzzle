use bevy::utils::thiserror::{self, Error};

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Failed to save: {0}")]
    SaveFailed(String),
    #[error("Failed to load: {0}")]
    LoadFailed(String),
}

/// ファイルストレージの抽象化
pub trait Storage: Send + Sync {
    fn save(&self, file_name: &str) -> Result<(), StorageError>;
}
