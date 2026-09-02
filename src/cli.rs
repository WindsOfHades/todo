use clap;

use crate::priority::Priority;

/// CLI for managing tasks
#[derive(clap::Parser)]
#[command(name = "todo")]
#[command(version)]
pub struct Cli {
    /// commands to execute
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    /// list the existing tasks
    List,
    /// Add a task
    Add {
        /// the title of the task
        title: String,
        /// the priority of the task
        #[arg(short, long)]
        priority: String,
    },
    /// remove a task
    Remove {
        /// the string id of the task
        id: String,
    },
    /// mark as done
    Done {
        /// the string id of the task
        id: String,
    },
    /// give stats on the todo list items
    Stats,
}
