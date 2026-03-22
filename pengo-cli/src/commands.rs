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
    /// Initialize a new novel project
    New {
        /// The title of the novel
        title: String,
    },

    /// Manage chapters
    Chapter {
        /// Specify the target volume
        #[arg(short, long, global = true)]
        volume: Option<String>,

        #[command(subcommand)]
        command: ChapterOp,
    },

    /// Manage characters
    Character {
        #[command(subcommand)]
        command: CharacterOp,
    },

    /// Manage scenes
    Scene {
        #[command(subcommand)]
        command: SceneOp,
    },

    Save {
        message: String,
    },

    Status,

    /// Add a new idea to idea.md
    Idea {
        /// The idea content to save
        idea: String,
    },
}

#[derive(Clone, Subcommand)]
pub enum ChapterOp {
    /// Create a new chapter
    New { name: Option<String> },
    /// List all chapters
    List,
}

#[derive(Clone, Subcommand)]
pub enum CharacterOp {
    /// Create a new character
    New { name: Option<String> },
    /// List all characters
    List,
}

#[derive(Clone, Subcommand)]
pub enum SceneOp {
    /// Create a new scene
    New { name: Option<String> },
    /// List all scenes
    List,
}
