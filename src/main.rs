use clap::{Parser, Subcommand};

mod extract;
mod translate;
use extract::{extract_code_from_markdown, extract_code_from_folder};
use translate::translate_markdown_to_html;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Hello {
        #[arg(short, long, default_value = "Jan")]
        name: String,
    },
    Extract {
        #[arg(short, long, conflicts_with = "folder")]
        file: Option<String>,
        #[arg(short, long, conflicts_with = "file")]
        folder: Option<String>,
    },
    Translate {
        #[arg(short, long)]
        input: String,
        #[arg(short, long)]
        output: String,
    },
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Hello { name } => {
            println!("Hello {}!", name);
        }
        Commands::Extract { file, folder } => {
            if let Some(file) = file {
                if let Err(e) = extract_code_from_markdown(file) {
                    eprintln!("Error extracting code: {}", e);
                }
            } else if let Some(folder) = folder {
                if let Err(e) = extract_code_from_folder(folder) {
                    eprintln!("Error extracting code: {}", e);
                }
            }
        }
        Commands::Translate { input, output } => {
            if let Err(e) = translate_markdown_to_html(input, output) {
                eprintln!("Error translating markdown: {}", e);
            }
        }
    }
}
