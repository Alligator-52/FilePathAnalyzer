use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::fs;
use std::cmp::Reverse;
use colored::*;

const MAX_ENTRIES: usize = 10;
fn main() -> io::Result<()>
{
    
    let args:Vec<String> = env::args().collect();
    if args.len() < 2
    {
        return Ok(());
    }
    let path = Path::new(&args[1]);
    let mut dir_paths: Vec<PathBuf> = Vec::new();
    implement_search(path, &mut dir_paths);
    return Ok(());
}

fn implement_search(path: &Path, dir_paths: &mut Vec<PathBuf>)
{
    if !path.exists() || !path.is_dir()
    {
        println!("{}","\nprovided path is not a directory".red());
        return;
    }
    search_longest_paths_dfs(path, dir_paths);
    let len = match dir_paths.len()
    {
        len if len >= MAX_ENTRIES => MAX_ENTRIES,
        len => len
    };
    dir_paths.sort_by_key(|p|Reverse(p.to_str().unwrap_or("").len()));

    println!
    (
        "\nSuccessfuilly scanned: {}\nTotal entries found: {}\n{} longest paths:\n",
        path.to_string_lossy().yellow(),
        dir_paths.len().to_string().yellow(),
        len.to_string().yellow()
    );

    for path in dir_paths.iter().take(MAX_ENTRIES)
    {
        println!("{}", path.to_string_lossy().green());
    }
}



fn search_longest_paths_dfs(path: &Path, dir_paths: &mut Vec<PathBuf>)
{
    println!("Reading directory: {}", path.to_string_lossy().cyan());
    if let Ok(entries) = fs::read_dir(path)
    {
        for entry in entries 
        {
            if let Ok(entry) = entry
            {
                let path = entry.path();
                if path.is_dir()
                {
                    search_longest_paths_dfs(&path, dir_paths);
                }
                else
                {
                    dir_paths.push(path);
                }
            }
        }
    }
}

#[allow(dead_code)]
fn user_input_method()
{
    let mut dir_paths: Vec<PathBuf> = Vec::new();
    loop
    {
        let user_input = get_user_input();
        let path = Path::new(&user_input);

        implement_search(path, &mut dir_paths);
        
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