use clap::{Parser, Subcommand};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

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
                        let output_path = PathBuf::from(&app_folder).join(Path::new(file).file_name().unwrap());
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

fn process_protocol_aimm(app_folder: &Path) -> io::Result<()> {
    let mut folders_to_process = Vec::new();

    // Recursively search for private and public folders
    for entry in WalkDir::new(app_folder).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_dir() {
            if path.ends_with("private") || path.ends_with("public") {
                folders_to_process.push(path.to_path_buf());
            }
        }
    }

    let mut processed_parents = std::collections::HashSet::new();

    for folder in folders_to_process {
        let parent = folder.parent().unwrap().to_path_buf();
        if processed_parents.contains(&parent) {
            continue;
        }

        let private_folder = parent.join("private");
        let public_folder = parent.join("public");
        let src_folder = parent.join("src");

        let mut sub_folders = vec![];
        if private_folder.exists() && private_folder.is_dir() {
            sub_folders.push(private_folder.clone());
        }
        if public_folder.exists() && public_folder.is_dir() {
            sub_folders.push(public_folder.clone());
        }

        if !sub_folders.is_empty() {
            println!("Combining folders into {:?}", src_folder);
            combine_folders(&sub_folders, &src_folder)?;

            // Remove the private and public folders
            if private_folder.exists() {
                fs::remove_dir_all(&private_folder)?;
            }
            if public_folder.exists() {
                fs::remove_dir_all(&public_folder)?;
            }

            processed_parents.insert(parent);
        }
    }

    Ok(())
}

fn combine_folders(folders: &[PathBuf], dest_folder: &PathBuf) -> io::Result<()> {
    for folder in folders {
        if folder.exists() && folder.is_dir() {
            println!("Processing folder: {:?}", folder);
            for entry in fs::read_dir(&folder)? {
                let entry = entry?;
                let entry_path = entry.path();
                let dest_path = dest_folder.join(entry_path.file_name().unwrap());

                if entry_path.is_dir() {
                    combine_folders(&[entry_path], &dest_path)?;
                } else {
                    fs::create_dir_all(dest_folder)?;
                    fs::copy(&entry_path, &dest_path)?;
                    println!("Copied file to {:?}", dest_path);
                }
            }
        } else {
            println!("Folder does not exist or is not a directory: {:?}", folder);
        }
    }
    Ok(())
}
