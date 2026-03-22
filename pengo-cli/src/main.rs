use clap::Parser;
use pengo_cli::commands::{ChapterOp, CharacterOp, Cli, Commands, SceneOp};
use pengo_core::novel;
// use std::env::args;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { title } => match novel::new_novel(&title) {
            Ok(_) => println!("Successfully initialized novel: {}", title),
            Err(e) => eprintln!("Error: Failed to create novel. {}", e),
        },

        Commands::Chapter { volume, command } => {
            match command {
                ChapterOp::New { name } => match novel::chapter_new(volume, name) {
                    Ok(_) => println!("Successfully created new chapter."),
                    Err(e) => eprintln!("Error: Failed to create chapter. {}", e),
                },
                ChapterOp::List => match novel::chapter_ls(volume) {
                    Ok(l) => println!("{:#?}", l),
                    Err(e) => eprintln!("Error: Failed to list chapters. {}", e),
                },
            };
        }

        Commands::Character { command } => match command {
            CharacterOp::New { name } => match name {
                Some(name) => match novel::character_new(name.clone()) {
                    Ok(_) => println!("Successfully created character: {}", name),
                    Err(e) => eprintln!("Error: Failed to create character. {}", e),
                },
                None => eprintln!("Error: Please provide a character name."),
            },
            CharacterOp::List => match novel::character_ls() {
                Ok(characters) => println!("{:#?}", characters),
                Err(e) => eprintln!("Error: Failed to list characters. {}", e),
            },
        },

        Commands::Scene { command } => match command {
            SceneOp::New { name } => match name {
                Some(name) => match novel::scene_new(name.clone()) {
                    Ok(_) => println!("Successfully created scene: {}", name),
                    Err(e) => eprintln!("Error: Failed to create scene. {}", e),
                },
                None => eprintln!("Error: Please provide a scene name."),
            },
            SceneOp::List => match novel::scene_ls() {
                Ok(scenes) => println!("{:#?}", scenes),
                Err(e) => eprintln!("Error: Failed to list scenes. {}", e),
            },
        },

        Commands::Idea { idea } => match novel::idea_add(&idea) {
            Ok(_) => println!("Successfully added idea to idea.md: '{}'", idea),
            Err(e) => eprintln!("Error: Failed to add idea. {}", e),
        },

        _ => println!("Command not implemented yet. Coming soon!"),
    };
}
