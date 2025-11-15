mod cli;
mod db;
mod models;
mod commands {
    pub mod add;
    pub mod categories;
    pub mod export;
    pub mod list;
    pub mod remove;
    pub mod stats;
}

use clap::Parser;
use cli::{Cli, Commands};
use db::open_db;

fn main() {
    let cli = Cli::parse();
    let conn = open_db();

    match cli.cmd {
        Commands::Add {
            category,
            value,
            note,
        } => commands::add::run(&conn, category, value, note),
        Commands::List { category, days } => commands::list::run(&conn, category, days),
        Commands::Stats { category } => commands::stats::run(&conn, category),
        Commands::Categories => commands::categories::run(&conn),
        Commands::Export { path } => commands::export::run(&conn, path),
        Commands::Remove { id, category, days } => commands::remove::run(&conn, id, category, days),
    }
}
