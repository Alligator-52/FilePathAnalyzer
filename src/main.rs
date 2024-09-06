use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::fs;
use colored::*;
fn main() 
{
    let mut path = String::new();
    loop
    {
        print!("{}","\nEnter the directory to scan: ".yellow());
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut path) 
        {
            Ok(_) => 
            {
                let trimmed_path = path.trim();
                if trimmed_path.is_empty()
                {
                    println!("{}", "\nFailed to read input, try again".red());
                    continue;
                }
                println!("\nEntered path: {}", trimmed_path.green());
                break;
            },
            Err(_) => 
            {
                println!("Failed to read input");
                continue;
            }
        }
    }
    print!("Working");
    
}
