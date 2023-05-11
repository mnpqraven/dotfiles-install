use crate::worker::prompt;
use clap::Parser;
use dotfiles_schema::{ConfigFile, Task};
use std::error::Error;
use tracing::info;
use worker::io::{get_path_type, PathType};

mod worker;

#[derive(Parser)]
struct Cli {
    #[arg(long = "cfg")]
    config: String,
    #[arg(long)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let tracel_level = match cli.debug {
        true => tracing::Level::DEBUG,
        false => tracing::Level::INFO,
    };

    tracing_subscriber::fmt()
        .with_max_level(tracel_level)
        .init();

    // NOTE: --cfg
    // handles path/url handling
    let path = get_path_type(&cli.config)?;
    let config: ConfigFile = match path {
        PathType::Dir => ConfigFile::from_dir(&cli.config)?,
        PathType::Url => ConfigFile::from_url(&cli.config).await?,
    };

    // let sudo_pw = ask_password("Your sudo password: ".into()).await?;

    // debug!("Running in debug mode");
    println!("your config: {:?}", config);

    let profile = prompt::ask_profile(&config.profiles)?;
    println!("You selected {}", profile);

    let filtered_tasks: Vec<Task> = config
        .tasks
        .into_iter()
        .filter(|e| match &e.profile {
            Some(profiles) => profiles.contains(&profile),
            None => true,
        })
        .collect();

    for task in filtered_tasks {
        info!("Executing task {}", task.name);
        for cmd in task.cmds {
            cmd.run().await?;
        }
    }

    Ok(())
}
