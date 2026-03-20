use clap::Parser;
use pengo_cli::commands::{Cli, Commands, PengoOp};
use pengo_core::{error, novel};
use std::env::args;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::New { title } => match novel::new_novel(&title) {
            Ok(_) => println!("{} create successed!", title),
            Err(e) => println!("Create failed because: {}", e),
        },
        Commands::Chapter { volume, command } => {
            match command {
                PengoOp::New { name } => match novel::chapter_new(volume, name) {
                    Ok(_) => println!("Create successed!"),
                    Err(e) => println!("Create failed because: {}", e),
                },
                PengoOp::List => match novel::chapter_ls(volume) {
                    Ok(l) => println!("{:?}", l),
                    Err(e) => println!("Find list failed because: {}", e),
                },
            };
        }
        // Commands::Character { command } => {}
        // Commands::Scene { command } => {}
        Commands::Idea { idea } => match novel::idea_add(&idea) {
            Ok(_) => println!("{} has added to idea.md", idea),
            Err(e) => println!("Idea failed because: {}", e),
        },

        _ => println!("Please wait for update"),
    };
}
