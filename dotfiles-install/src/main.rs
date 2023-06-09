use crate::worker::prompt;
use clap::Parser;
use dotfiles_schema::{ConfigFile, Profile, Task};
use std::{env, error::Error};
use tracing::{debug, info};
use tracing_subscriber::fmt::writer::MakeWriterExt;
use worker::io::{get_path_type, PathType};

mod worker;

#[derive(Parser)]
struct Cli {
    #[arg(long = "cfg")]
    config: Option<String>,
    #[arg(long)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // INFO: logging
    let cli = Cli::parse();

    let trace_level = match cli.debug {
        true => tracing::Level::DEBUG,
        false => tracing::Level::INFO,
    };
    let path = env::current_dir()?;
    let file_appender = tracing_appender::rolling::never(path, "dotfile_install.log");
    // TODO: this hogs stdout
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let tracing_writer = non_blocking.and(std::io::stdout);

    tracing_subscriber::fmt()
        .with_writer(tracing_writer)
        .with_ansi(false)
        .with_max_level(trace_level)
        .init();

    // NOTE: --cfg
    // handles path/url handling
    let config_path = match cli.config {
        Some(arg_data) => arg_data,
        None => prompt::ask_config_path()?,
    };

    let config: ConfigFile = match get_path_type(&config_path)? {
        PathType::Dir => ConfigFile::from_dir(&config_path)?,
        PathType::Url => ConfigFile::from_url(&config_path).await?,
    };

    debug!("Running in debug mode");
    println!("your config: {:?}", config);

    let profile: Profile = prompt::ask_profile(&config.profiles)?;
    println!("You selected {}", profile);

    let filtered_tasks: Vec<Task> = config
        .tasks
        .into_iter()
        .filter(|e| match &e.profile {
            Some(profiles) => profiles.contains(&profile),
            None => true,
        })
        .collect();

    if !prompt::confirm_summary(&filtered_tasks)? {
        println!("Exiting program");
        return Ok(());
    }

    for task in filtered_tasks {
        info!("Executing task {}", task.name);
        for cmd in task.cmds {
            cmd.run().await?;
        }
    }

    Ok(())
}
