use std::env;
use std::process;
use tokio;
mod commands;

#[tokio::main]
async fn main() {
    
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
        Some("create") => println!("Function to create a model with a model file:"),
        Some("delete") => println!("Function to delete a model:"),
        Some("list") => commands::_list::list().await,
        Some("run") => println!("Function to run a model:"),
        Some("rag") => println!("Function to edit the RAG assigned to a model:"),

        // Flags
        Some("-h") | Some("--help") => println!("Function to show all the usable arguments:"),
        Some("-v") | Some("--version") => commands::_version::version(),

        // Exhaustive checks
        Some(other) => println!("Unknown argument: {}", other),
        None => println!("No arguments passed")
        
    }

}
