use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pengo")]
#[command(author = "jadepoet")]
#[command(version = "1.0")]
#[command(about = "Cargo for Novelists.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// initializate new novel
    New {
        /// novel title
        title: String,
    },

    /// chapter operations
    Chapter {
        /// 指定要操作的卷 (Volume)
        #[arg(short, long, global = true)]
        volume: Option<String>,

        #[command(subcommand)]
        command: PengoOp,
    },

    Character {
        ///command
        #[command(subcommand)]
        command: PengoOp,
    },

    Scene {
        ///command
        #[command(subcommand)]
        command: PengoOp,
    },

    /// add your idea to idea.md
    Idea {
        /// Your Idea
        idea: String,
    },
}
#[derive(Clone, Subcommand)]
pub enum PengoOp {
    /// Create a new xxx
    New { name: Option<String> },

    /// List all xxx
    List,
}
