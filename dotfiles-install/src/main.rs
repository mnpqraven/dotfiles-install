use clap::Parser;
use dotfiles_schema::ConfigFile;
use std::{fs::File, io::BufReader};

#[derive(Parser)]
struct Cli {
    name: Option<String>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.name {
        Some(name) => {
            println!("Hello {}", name);
        }
        None => {}
    }

    // manual input + testing
    {
        // IO
        // accept either path or url
        // UNWRAP: handle
        let reader = File::open("./template.jsonc").unwrap();
        // UNWRAP: handle
        let datastruct: ConfigFile = serde_json::from_reader(BufReader::new(reader)).unwrap();
        dbg!(&datastruct);
    }
    {
        // schema
        // let schema = schema_for!(ConfigFile);
        // println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
    { // command building
         // constring default command for each [`InstallType`], ready to accept
         // serialized inputs
    }
    { // reading through config
    }
    {
        // running command + logging + piping through stderr
    }
}
