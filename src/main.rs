use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::fs;
use colored::*;


fn main() 
{
    //let mut path  = Path::new("");
    loop
    {
        let user_input = get_user_input();
        let path = Path::new(&user_input);

        if !path.is_dir()
        {
            println!("{}","\nprovided path is not a directory".red());
            continue;
        }
        println!("\nProvided Directory: {}",&path.to_string_lossy().green());
        break;
    }
}

fn get_user_input() -> String
{
    let mut input = String::new();
    loop
    {
        print!("{}","\nEnter the directory to scan: ".yellow());
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) 
        {
            Ok(_) => 
            {
                let trimmed_path = input.trim();
                if trimmed_path.is_empty()
                {
                    println!("{}", "\nFailed to read input, try again".red());
                    continue;
                }
                return trimmed_path.to_string();
            },
            Err(_) => 
            {
                println!("Failed to read input");
                continue;
            }
        }
    }
}
