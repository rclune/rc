use clap::{Parser, Subcommand};
use std::process;

/// A simple command line tool
#[derive(Parser, Debug)]
#[command(name = "rc")]
#[command(version)]
#[command(about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Verbose mode
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run a command with optional arguments
    Run {
        /// The command to run
        command: String,

        /// Optional arguments for the command
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}

fn main() {
    let args = Args::parse();

    if args.verbose {
        println!("Running in verbose mode");
        println!("Args: {:#?}", args);
    }

    match &args.command {
        Some(Commands::Run {
            command,
            args: cmd_args,
        }) => {
            println!("Running command: {}", command);
            if !cmd_args.is_empty() {
                println!("With arguments: {:?}", cmd_args);
            }
            // TODO: Implement actual command execution here
        }
        None => {
            eprintln!("Error: No command specified");
            eprintln!("Use --help to see available commands");
            process::exit(1);
        }
    }
}
