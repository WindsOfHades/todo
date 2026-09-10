use clap::Parser;

mod cli;
mod errors;
mod priority;
mod storage;
mod task;
mod todo;

fn main() -> Result<(), errors::ToDoError> {
    let cli = cli::Cli::parse();

    let path = std::path::PathBuf::new().join("data").join("todo.json");
    let storage = storage::json_file::JsonFileStorage::new(path);

    let mut todo_list = match todo::ToDo::load(&storage) {
        Err(_) => todo::ToDo::new(),
        Ok(todo) => todo,
    };

    match &cli.command {
        cli::Commands::List => {
            println!("{}", todo_list.list());
        }
        cli::Commands::Stats => {
            println!("{}", todo_list.stats());
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
    todo_list.save(&storage)?;
    Ok(())
}
