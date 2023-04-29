use clap::Parser;
use dotfiles_schema::ConfigFile;
use std::error::Error;
use worker::io::{handle_path_type, PathType};

mod worker;

#[derive(Parser)]
struct Cli {
    #[arg(long = "cfg")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // NOTE: --cfg
    // handles path/url handling
    let path = handle_path_type(&cli.config)?;
    let _config: ConfigFile = match path {
        PathType::Dir => ConfigFile::from_dir(&cli.config)?,
        PathType::Url => ConfigFile::from_url(&cli.config).await?,
    };

    // manual input + testing
    { // command building
         // constring default command for each [`InstallType`], ready to accept
         // serialized inputs
    }
    { // reading through config
    }
    {
        // running command + logging + piping through stderr
    }

    Ok(())
}
