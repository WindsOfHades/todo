use crate::errors;
use crate::task::Task;
use serde_json;
use std::fs;
use std::path::PathBuf;

pub fn save(tasks: &[Task]) -> Result<(), errors::StorageError> {
    let json_file = get_storage_path()?;
    let json_string = serde_json::to_string_pretty(tasks)?;
    fs::write(&json_file, json_string)?;
    Ok(())
}

pub fn load() -> Result<Vec<Task>, errors::StorageError> {
    let json_file = get_storage_path()?;
    let json_string = fs::read_to_string(json_file)?;
    let loaded_tasks = serde_json::from_str(&json_string)?;
    Ok(loaded_tasks)
}

fn get_storage_path() -> Result<PathBuf, errors::StorageError> {
    let json_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("todo.json");

    if let Some(parent) = json_file_path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(json_file_path)
}
