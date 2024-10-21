use std::env;
use std::fs;
use std::process;
use std::path::PathBuf;
use tokio;
mod commands;

#[tokio::main]
async fn main() {
    
    //? [Create the needed directories and files]
    //* Create the data folder on Windows
    #[cfg(target_os = "windows")]
    {
        let mut config_dir = PathBuf::from(env::var("APPDATA").unwrap());
        config_dir.push(".ollamind");

        fs::create_dir_all(&config_dir).unwrap_or_else(|e| {
            println!("Error creating config directory: {}", e);
            process::exit(1);
        });
    }

    //* Create the data folder on Unix
    #[cfg(target_family = "unix")]
    { 
        println!("OllaMind doesn't support linux yet");
        process::exit(1);
    }


    //? [Simple arguments parser]
    let args: Vec<String> = env::args().collect();

    //* If no arguments are passed
    if args.len() < 2 {

        println!("Usage: ollamind <arguments>");
        process::exit(1);

    }

    //* If at least one orgument is passed
    match args.get(1).map(|s| s.as_str()) {

        // Arguments
        Some("config") => println!("Function to edit ollamind's configuration"),
        Some("run") => println!("Function to run a model:"),
        Some("rag") => println!("Function to edit the RAG assigned to a model:"),

        // Flags
        Some("-h") | Some("--help") => println!("Function to show all the usable arguments:"),
        Some("-v") | Some("--version") => commands::_version(),

        // Exhaustive checks
        Some(other) => println!("Unknown argument: {}", other),
        None => println!("No arguments passed")
        
    }

}
