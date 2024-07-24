use clap::{Parser, Subcommand};

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
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Hello { name } => {
            println!("Hello {}!", name);
        }
    }
}
