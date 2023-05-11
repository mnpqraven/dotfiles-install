use dotfiles_schema::Profile;
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
    let t = match buffer.trim_end().parse::<usize>() {
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
    match t.is_ok() {
        true => t,
        false => ask_profile(profiles),
    }
}
