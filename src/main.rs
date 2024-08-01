// src/main.rs
use clap::Parser;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

mod commands;
mod utils;
mod schema;

use commands::{extract::*, save::*, translate::*, Args, Commands};
use utils::{ensure_pandoc_installed, process_protocol_aimm};

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Extract {
            file,
            folder,
            output,
            protocol,
        } => {
            let app_folder = output.clone().unwrap_or_else(|| ".app".to_string());

            if let Some(file) = file {
                match extract_code_from_markdown(file) {
                    Ok(Ok(extracted_code)) => {
                        for (filename, code) in extracted_code {
                            let output_path = PathBuf::from(&app_folder).join(filename);
                            if let Some(parent) = output_path.parent() {
                                fs::create_dir_all(parent).unwrap();
                            }
                            let mut output_file = File::create(&output_path).unwrap();
                            output_file.write_all(code.as_bytes()).unwrap();
                            println!("Code extracted to {}", output_path.display());
                        }
                    }
                    Ok(Err(_)) => {
                        // Copy simple markdown file to .app folder
                        let output_path =
                            PathBuf::from(&app_folder).join(Path::new(file).file_name().unwrap());
                        fs::copy(file, &output_path).unwrap();
                        println!("Copied file to {}", output_path.display());
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

            if let Some(protocol) = protocol {
                if protocol == "AImM" {
                    println!("Protocol AImM detected. Combining folders...");
                    if let Err(e) = process_protocol_aimm(&PathBuf::from(&app_folder)) {
                        eprintln!("Error processing protocol AImM: {}", e);
                    }
                } else {
                    println!("Protocol detected but not AImM.");
                }
            } else {
                println!("No protocol specified.");
            }
        }
        Commands::Translate {
            folder,
            output,
            css,
        } => {
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
        Commands::Save {
            file,
            db: _,
        } => {
            let created_files = fs::read_to_string(file).expect("Unable to read created files list");
            let html_files: Vec<String> = created_files.lines().map(|s| s.to_string()).collect();

            let mut conn = establish_connection(); // Make the connection mutable
            if let Err(e) = save_html_metadata_to_db(&html_files, &mut conn) {
                eprintln!("Error saving HTML metadata to database: {}", e);
            }
        }
    }
}
