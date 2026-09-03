use crate::errors;
use crate::priority;
use crate::storage;
use crate::task::Task;
use uuid;

#[derive(Debug)]
pub struct ToDo {
    tasks: Vec<Task>,
}

impl ToDo {
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

    pub fn list(&self) {
        for t in self.tasks.iter() {
            println!("{t}");
        }
    }

    pub fn save(&self) -> Result<(), errors::ToDoError> {
        storage::save(&self.tasks)?;
        Ok(())
    }

    pub fn load() -> Result<Self, errors::ToDoError> {
        let tasks = storage::load()?;
        Ok(Self { tasks: tasks })
    }

    pub fn stats(&self) {
        let total = self.tasks.len();
        let done = self.tasks.iter().filter(|task| task.is_done()).count();
        let pending = total - done;
        let high = self
            .tasks
            .iter()
            .filter(|task| *task.priority() == priority::Priority::High)
            .count();
        let medium = self
            .tasks
            .iter()
            .filter(|task| *task.priority() == priority::Priority::Medium)
            .count();
        let low = self
            .tasks
            .iter()
            .filter(|task| *task.priority() == priority::Priority::Low)
            .count();
        println!("📊 Task Statistics");
        println!("   Total:   {total}");
        println!("   Done:    {done} ✅");
        println!("   Pending: {pending}");
        println!("   🔴 High:   {high}");
        println!("   🟡 Medium: {medium}");
        println!("   🟢 Low:    {low}");
    }

    fn get_task_by_id(&mut self, id: &str) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.id() == id)
    }
}
