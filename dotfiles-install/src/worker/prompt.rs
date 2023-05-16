use dotfiles_schema::{Profile, Task};
use std::{error::Error, io::stdin};

pub fn ask_profile(profiles: &Vec<Profile>) -> Result<Profile, Box<dyn Error>> {
    println!("found the following profiles {:?}", profiles);
    println!("Select the profile that you want to install");
    for (ind, profile) in profiles.iter().enumerate() {
        println!("{}: {}", ind, profile);
    }

    let mut buffer = String::new();

    // `read_line` returns `Result` of bytes read
    stdin().read_line(&mut buffer)?;
    let input = match buffer.trim_end().parse::<usize>() {
        Ok(profile_index) => match profiles.get(profile_index) {
            Some(profile) => Ok(*profile),
            None => {
                println!("Invalid input, please try again");
                Err("Invalid input".into())
            }
        },
        Err(_) => {
            println!("Invalid input, please try again");
            Err("Invalid input".into())
        }
    };

    // recursion
    match input.is_ok() {
        true => input,
        false => ask_profile(profiles),
    }
}

pub fn confirm_summary(tasks: &Vec<Task>) -> Result<bool, Box<dyn Error>> {
    println!("You are about to run the following tasks:");
    for task in tasks.iter() {
        println!("{}", task);
    }
    println!("Proceed ? y/n");

    let mut buffer = String::new();

    // `read_line` returns `Result` of bytes read
    stdin().read_line(&mut buffer)?;
    let input_text = match buffer.trim_end().parse::<char>() {
        Ok('y') | Ok('Y') => Ok(true),
        Ok('n') | Ok('N') => Ok(false),
        _ => {
            println!("Invalid input, please try again");
            Err("Invalid input, please try again".into())
        }
    };

    // recursion
    match input_text.is_ok() {
        true => input_text,
        false => confirm_summary(tasks),
    }
}

pub fn ask_config_path() -> Result<String, Box<dyn Error>> {
    println!("Please provide your config file (path or URL):");
    let mut buffer = String::new();

    stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().into())
}
