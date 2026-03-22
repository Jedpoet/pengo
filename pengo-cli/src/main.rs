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
        Commands::Character { command } => match command {
            PengoOp::New { name } => match name {
                Some(name) => match novel::character_new(name) {
                    Ok(_) => println!("Create successed!"),
                    Err(e) => println!("Create failed because: {}", e),
                },
                None => println!("need input character name"),
            },
            PengoOp::List => match novel::character_ls() {
                Ok(characters) => println!("{:?}", characters),
                Err(e) => println!("{}", e),
            },
        },

        Commands::Scene { command } => match command {
            PengoOp::New { name } => match name {
                Some(name) => match novel::scene_new(name) {
                    Ok(_) => println!("Create successed!"),
                    Err(e) => println!("Create failed because: {}", e),
                },
                None => println!("need input scene name"),
            },
            PengoOp::List => match novel::scene_ls() {
                Ok(scenes) => println!("{:?}", scenes),
                Err(e) => println!("{}", e),
            },
        },
        Commands::Idea { idea } => match novel::idea_add(&idea) {
            Ok(_) => println!("{} has added to idea.md", idea),
            Err(e) => println!("Idea failed because: {}", e),
        },

        _ => println!("Please wait for update"),
    };
}
