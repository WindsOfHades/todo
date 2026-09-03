#[derive(Debug, thiserror::Error)]
pub enum ToDoError {
    #[error(transparent)]
    Storage(#[from] StorageError),
}

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Failed to perform I/O operation")]
    IO(#[from] std::io::Error),
    #[error("Failed to parse json")]
    Json(#[from] serde_json::Error),
}
