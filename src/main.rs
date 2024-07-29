use clap::{Parser, Subcommand};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

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
        #[arg(short, long)]
        protocol: Option<String>,
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
        Commands::Extract { file, folder, output, protocol } => {
            let app_folder = output.clone().unwrap_or_else(|| ".app".to_string());

            if let Some(file) = file {
                match extract_code_from_markdown(file) {
                    Ok(extracted_code) => {
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
                    let src_folder = PathBuf::from(&app_folder).join("src");
                    let private_folder = PathBuf::from(&app_folder).join("private");
                    let public_folder = PathBuf::from(&app_folder).join("public");
                    combine_folders(&[private_folder.clone(), public_folder.clone()], &src_folder).unwrap();

                    // Remove the private and public folders
                    if private_folder.exists() {
                        fs::remove_dir_all(private_folder).unwrap();
                    }
                    if public_folder.exists() {
                        fs::remove_dir_all(public_folder).unwrap();
                    }
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

fn combine_folders(folders: &[PathBuf], dest_folder: &PathBuf) -> io::Result<()> {
    for folder in folders {
        if folder.exists() && folder.is_dir() {
            for entry in fs::read_dir(&folder)? {
                let entry = entry?;
                let entry_path = entry.path();
                let dest_path = dest_folder.join(entry_path.file_name().unwrap());

                if entry_path.is_dir() {
                    combine_folders(&[entry_path], &dest_path)?;
                } else {
                    fs::create_dir_all(dest_folder)?;
                    fs::copy(entry_path, dest_path)?;
                }
            }
        }
    }
    Ok(())
}
