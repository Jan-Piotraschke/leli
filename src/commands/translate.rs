use std::fs::{self};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

/// Generates HTML from a markdown file using Pandoc and saves it to the specified output path.
pub fn generate_html_from_markdown(input_path: &str, output_path: &str, css_path: &str) -> io::Result<()> {
    let output = Command::new("pandoc")
        .arg("--standalone")
        .arg("--to=html")
        .arg("--css")
        .arg(css_path)
        .arg("--output")
        .arg(output_path)
        .arg(input_path)
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("pandoc error: {}", String::from_utf8_lossy(&output.stderr)),
        ));
    }

    println!("Generated HTML from {} to {}", input_path, output_path);
    Ok(())
}

pub fn translate_markdown_folder(folder_path: &str, doc_folder: &str, css_path: &str) -> io::Result<()> {
    let mut html_paths: Vec<String> = Vec::new();

    translate_markdown_folder_internal(folder_path, doc_folder, css_path, &mut html_paths)?;

    // Write HTML file paths to a text file
    let output_path = PathBuf::from(doc_folder).join("created_html_files.txt");
    let mut file = std::fs::File::create(&output_path)?;
    for path in html_paths {
        writeln!(file, "{}", path)?;
    }

    Ok(())
}

fn translate_markdown_folder_internal(folder_path: &str, doc_folder: &str, css_path: &str, html_paths: &mut Vec<String>) -> io::Result<()> {
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let sub_doc_folder = PathBuf::from(doc_folder).join(path.file_name().unwrap());
            fs::create_dir_all(&sub_doc_folder)?;
            translate_markdown_folder_internal(path.to_str().unwrap(), sub_doc_folder.to_str().unwrap(), css_path, html_paths)?;
        } else if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            let base_name = path.file_stem().unwrap().to_str().unwrap();
            let html_output_path = PathBuf::from(doc_folder).join(format!("{}_combined.html", base_name));
            if let Err(e) = generate_html_from_markdown(path.to_str().unwrap(), html_output_path.to_str().unwrap(), css_path) {
                eprintln!("Error generating HTML for {}: {}", path.display(), e);
            } else {
                html_paths.push(html_output_path.to_str().unwrap().to_string());
            }
        }
    }
    Ok(())
}
