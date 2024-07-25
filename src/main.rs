use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::Command;
use std::io::Write;

mod extract;
mod translate;
use extract::{extract_code_from_markdown, extract_code_from_folder};
use translate::translate_markdown_folder;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
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
        folder: String,
        #[arg(short, long)]
        output: Option<String>,
        #[arg(short, long)]
        css: Option<String>,
    },
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Extract { file, folder, output } => {
            let app_folder = output.clone().unwrap_or_else(|| "app".to_string());

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
                if let Err(e) = extract_code_from_folder(folder, &app_folder) {
                    eprintln!("Error extracting code: {}", e);
                }
            }
        }
        Commands::Translate { folder, output, css } => {
            let doc_folder = output.clone().unwrap_or_else(|| "doc".to_string());
            let css_path = css.clone().unwrap_or_else(|| "src/css/style.css".to_string());

            if !ensure_pandoc_installed() {
                eprintln!("Pandoc is not installed. Please install Pandoc to use this tool.");
                std::process::exit(1);
            }

            if let Err(e) = translate_markdown_folder(&folder, &doc_folder, &css_path) {
                eprintln!("Error translating markdown: {}", e);
            }
        }
    }
}

fn ensure_pandoc_installed() -> bool {
    let output = Command::new("pandoc")
        .arg("--version")
        .output();

    match output {
        Ok(output) if output.status.success() => true,
        _ => false,
    }
}
