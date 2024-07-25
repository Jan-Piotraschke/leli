use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::io::Write;

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
        #[arg(short, long)]
        output: Option<String>,
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
        Commands::Extract { file, folder, output } => {
            let app_folder = output.clone().unwrap_or_else(|| "app".to_string());
            let doc_folder = "doc".to_string();  // Fixed output folder for HTML documentation

            if let Some(file) = file {
                match extract_code_from_markdown(file) {
                    Ok(extracted_code) => {
                        for (filename, code) in extracted_code {
                            let output_path = PathBuf::from(&app_folder).join(filename);
                            if let Some(parent) = output_path.parent() {
                                std::fs::create_dir_all(parent).unwrap();
                            }
                            let mut output_file = std::fs::File::create(&output_path).unwrap();
                            output_file.write_all(code.as_bytes()).unwrap();
                            println!("Code extracted to {}", output_path.display());
                        }
                    }
                    Err(e) => {
                        eprintln!("Error extracting code: {}", e);
                    }
                }
            } else if let Some(folder) = folder {
                if let Err(e) = extract_code_from_folder(folder, &app_folder, &doc_folder) {
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
