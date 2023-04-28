use clap::Parser;

#[derive(Parser)]
struct Cli {
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    match cli.name {
        Some(name) => {
            println!("Hello {}", name);
        }
        None => {}
    }
}
