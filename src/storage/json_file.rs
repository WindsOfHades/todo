use crate::errors;
use crate::storage::task_store;
use crate::task::Task;
use serde_json;
use std::fs;
use std::path::PathBuf;

pub struct JsonFileStorage {
    path: PathBuf,
}

impl JsonFileStorage {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl task_store::TaskStore for JsonFileStorage {
    fn save(&self, tasks: &[Task]) -> Result<(), errors::StorageError> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json_string = serde_json::to_string_pretty(tasks)?;
        fs::write(&self.path, json_string)?;
        Ok(())
    }

    fn load(&self) -> Result<Vec<Task>, errors::StorageError> {
        let json_string = fs::read_to_string(&self.path)?;
        let loaded_tasks = serde_json::from_str(&json_string)?;
        Ok(loaded_tasks)
    }
}
