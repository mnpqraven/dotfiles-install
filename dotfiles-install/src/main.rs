use crate::worker::prompt;
use clap::Parser;
use dotfiles_schema::{ConfigFile, Task};
use std::{env, error::Error};
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

    let trace_level = match cli.debug {
        true => tracing::Level::DEBUG,
        false => tracing::Level::INFO,
    };
    let path = env::current_dir()?;
    let file_appender = tracing_appender::rolling::never(path, "dotfile_install.log");
    // TODO: this hogs stdout
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_max_level(trace_level)
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
