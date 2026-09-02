use clap::Parser;

mod cli;
mod priority;
mod storage;
mod task;
mod todo;

fn main() {
    let mut todo_list = todo::ToDo::load();
    let cli = cli::Cli::parse();

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
        cli::Commands::Add { title, priority } => {
            todo_list.add(title, priority.clone());
        }
    }
    // let mut todo_list = todo::ToDo::load();
    // todo_list.mark_done("4f909899-5f45-43b1-a68e-9f0ba1702e1a");
    // todo_list.list();
    todo_list.save();
    // todo_list.stats();
}
