use crate::errors;
use crate::priority;
use crate::storage::task_store::TaskStore;
use crate::task::Task;
use uuid;

struct TodoStats {
    total: usize,
    done: usize,
    pending: usize,
    high: usize,
    medium: usize,
    low: usize,
}

#[derive(Debug)]
pub struct ToDo {
    tasks: Vec<Task>,
}

impl ToDo {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add(&mut self, title: &str, prio: priority::Priority) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        self.tasks.push(Task::new(&id, title.to_string(), prio));
        id
    }

    pub fn remove(&mut self, id: &str) {
        self.tasks.retain(|t| t.id() != id);
    }

    pub fn mark_done(&mut self, id: &str) {
        if let Some(task) = self.get_task_by_id(id) {
            task.set_done(true);
        }
    }

    pub fn mark_undone(&mut self, id: &str) {
        if let Some(task) = self.get_task_by_id(id) {
            task.set_done(false);
        }
    }

    pub fn list(&self) -> String {
        self.tasks
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn save(&self, storage: &dyn TaskStore) -> Result<(), errors::ToDoError> {
        storage.save(&self.tasks)?;
        Ok(())
    }

    pub fn load(storage: &dyn TaskStore) -> Result<Self, errors::ToDoError> {
        let tasks = storage.load()?;
        Ok(Self { tasks: tasks })
    }

    pub fn stats(&self) -> String {
        let stats = self.compute_stats();
        format!(
            "📊 Task Statistics\nTotal:\t{}\nDone:\t{} ✅\nPending:\t{}\n🔴 High:\t{}\n🟡 Medium:\t{}\n🟢 Low:\t{}\n",
            stats.total, stats.done, stats.pending, stats.high, stats.medium, stats.low
        )
    }

    fn get_task_by_id(&mut self, id: &str) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.id() == id)
    }

    fn compute_stats(&self) -> TodoStats {
        let total = self.tasks.len();
        let done = self.tasks.iter().filter(|task| task.is_done()).count();
        TodoStats {
            total,
            done,
            pending: total - done,
            high: self
                .tasks
                .iter()
                .filter(|task| *task.priority() == priority::Priority::High)
                .count(),
            medium: self
                .tasks
                .iter()
                .filter(|task| *task.priority() == priority::Priority::Medium)
                .count(),
            low: self
                .tasks
                .iter()
                .filter(|task| *task.priority() == priority::Priority::Low)
                .count(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::task_store::MockTaskStore;

    #[test]
    fn new_todo() {
        let todo = ToDo::new();
        assert!(todo.tasks.is_empty());
    }

    #[test]
    fn add_a_task() {
        let mut todo = ToDo::new();
        todo.add("hello", priority::Priority::High);
        assert_eq!(todo.tasks.len(), 1);
    }

    #[test]
    fn remove_a_task() {
        let mut todo = ToDo::new();
        let id = todo.add("hello", priority::Priority::High);
        todo.remove(&id);
        assert!(todo.tasks.is_empty());
    }

    #[test]
    fn remove_a_non_existing_task() {
        let mut todo = ToDo::new();
        todo.add("hello", priority::Priority::High);
        todo.remove("random_id");
        assert_eq!(todo.tasks.len(), 1);
    }

    #[test]
    fn mark_a_task_done_and_not_done() {
        let mut todo = ToDo::new();
        let id = todo.add("hello", priority::Priority::High);
        assert!(!todo.get_task_by_id(&id).unwrap().is_done());
        todo.mark_done(&id);
        assert!(todo.get_task_by_id(&id).unwrap().is_done());
        todo.mark_undone(&id);
        assert!(!todo.get_task_by_id(&id).unwrap().is_done());
    }
    #[test]
    fn save_tasks() {
        let mut todo = ToDo::new();
        let id = todo.add("hello", priority::Priority::High);
        let mut storage = MockTaskStore::new();
        storage
            .expect_save()
            .withf(move |tasks| tasks[0].id() == id)
            .times(1)
            .returning(|_| Ok(()));
        todo.save(&storage).unwrap();
    }

    #[test]
    fn save_tasks_fails() {
        let todo = ToDo::new();
        let mut storage = MockTaskStore::new();
        storage.expect_save().times(1).returning(|_| {
            Err(errors::StorageError::IO(std::io::Error::other(
                "failed to save",
            )))
        });
        let result = todo.save(&storage);
        assert!(matches!(result, Err(errors::ToDoError::Storage(_))));
    }

    #[test]
    fn load_tasks() {
        let mut storage = MockTaskStore::new();
        storage.expect_load().times(1).returning(|| {
            Ok(vec![Task::new(
                "id_0",
                "task".to_string(),
                priority::Priority::High,
            )])
        });
        let todo = ToDo::load(&storage).unwrap();
        assert_eq!(todo.tasks.len(), 1);
        assert_eq!(todo.tasks[0].id(), "id_0");
    }

    #[test]
    fn load_tasks_fails() {
        let mut storage = MockTaskStore::new();
        storage.expect_load().times(1).returning(|| {
            Err(errors::StorageError::IO(std::io::Error::other(
                "failed to load",
            )))
        });
        let todo = ToDo::load(&storage);
        assert!(matches!(todo, Err(errors::ToDoError::Storage(_))));
    }

    #[test]
    fn task_stats() {
        let mut todo = ToDo::new();
        todo.add("task_1", priority::Priority::High);
        todo.add("task_2", priority::Priority::Low);

        let result = todo.stats();
        assert!(result.contains("Total:\t2"));
        assert!(result.contains("Pending:\t2"));
        assert!(result.contains("Done:\t0"));
        assert!(result.contains("High:\t1"));
        assert!(result.contains("Low:\t1"));
        assert!(result.contains("Medium:\t0"));
    }

    #[test]
    fn task_list() {
        let mut todo = ToDo::new();
        todo.add("task_1", priority::Priority::High);
        todo.add("task_2", priority::Priority::Low);

        let result = todo.list();
        assert_eq!(result.lines().count(), 2);
    }
}
