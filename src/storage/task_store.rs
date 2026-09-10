use crate::{errors, task::Task};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)] // added as a test config to only use via cargo test
pub trait TaskStore {
    fn save(&self, tasks: &[Task]) -> Result<(), errors::StorageError>;
    fn load(&self) -> Result<Vec<Task>, errors::StorageError>;
}
