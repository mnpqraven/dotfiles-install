use clap::Parser;
use dotfiles_schema::ConfigFile;
use std::{
    error::Error,
    io::{self, Write},
    process::Command,
};
use tracing::debug;
use worker::io::{handle_path_type, PathType};

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
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let cli = Cli::parse();

    // NOTE: --cfg
    // handles path/url handling
    let path = handle_path_type(&cli.config)?;
    let _config: ConfigFile = match path {
        PathType::Dir => ConfigFile::from_dir(&cli.config)?,
        PathType::Url => ConfigFile::from_url(&cli.config).await?,
    };
    if cli.debug {
        debug!("Running debug mode");
        debug!("{:?}", _config);
    }

    // manual input + testing
    { // command building
         // constring default command for each [`InstallType`], ready to accept
         // serialized inputs
    }
    println!("running command ls -la");
    // TODO: default pacman + yay command builder
    // we need to test with a linux machine
    let ls_cmd = Command::new("ls")
        .arg("-la")
        .output()
        .expect("failed to run command");

    // normally writing command output to stdout
    // io::stdout().write_all(&ls_cmd.stdout)?;
    if !ls_cmd.stderr.is_empty() {
        // do what we need to do with error
        // logging/tracing, saving to summary
        io::stdout().write_all(&ls_cmd.stderr)?;
    }

    { // reading through config
    }
    {
        // running command + logging + piping through stderr
    }

    Ok(())
}
