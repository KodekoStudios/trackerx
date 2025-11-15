use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        category: String,
        value: f64,
        note: Option<String>,
    },
    List {
        category: Option<String>,
        days: Option<u32>,
    },
    Stats {
        category: String,
    },
    Categories,
    Export {
        path: String,
    },
    Remove {
        id: Option<i64>,
        category: Option<String>,
        days: Option<u32>,
    },
}
