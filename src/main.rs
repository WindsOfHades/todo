use clap::Parser;

mod cli;
mod errors;
mod priority;
mod storage;
mod task;
mod todo;

fn main() -> Result<(), errors::ToDoError> {
    let cli = cli::Cli::parse();
    let mut todo_list = match todo::ToDo::load() {
        Err(_) => todo::ToDo::new(),
        Ok(todo) => todo,
    };

    match &cli.command {
        cli::Commands::List => {
            todo_list.list();
        }
        cli::Commands::Stats => {
            todo_list.stats();
        }
        cli::Commands::Remove { id } => {
            todo_list.remove(id);
        }
        cli::Commands::Done { id } => {
            todo_list.mark_done(id);
        }
        cli::Commands::Pending { id } => {
            todo_list.mark_undone(id);
        }
        cli::Commands::Add { title, priority } => {
            todo_list.add(title, priority.clone());
        }
    }
    todo_list.save()?;
    Ok(())
}
