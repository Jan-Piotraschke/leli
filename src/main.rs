use clap::{Parser, Subcommand};

mod extract;
use extract::extract_code_from_markdown;

/// Simple CLI for Literate Programming Microservices
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Greet a person
    Hello {
        /// Name of the person to greet
        #[arg(short, long, default_value = "Jan")]
        name: String,
    },
    /// Extract Python code from a markdown file
    Extract {
        /// Path to the markdown file
        #[arg(short, long)]
        file: String,
    },
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Hello { name } => {
            println!("Hello {}!", name);
        }
        Commands::Extract { file } => {
            if let Err(e) = extract_code_from_markdown(file) {
                eprintln!("Error extracting code: {}", e);
            }
        }
    }
}
