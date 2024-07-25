use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct MarkdownMeta {
    pub output_filename: String,
}

pub fn extract_code_from_markdown(file_path: &str) -> io::Result<()> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut meta_data = String::new();
    let mut in_front_matter = false;
    let mut code_blocks: HashMap<String, String> = HashMap::new();
    let mut current_lang = String::new();

    for line in reader.lines() {
        let line = line?;

        if line.trim() == "---" && !in_front_matter {
            // Start of YAML front matter
            in_front_matter = true;
        } else if line.trim() == "---" && in_front_matter {
            // End of YAML front matter
            in_front_matter = false;
        } else if in_front_matter {
            // Inside YAML front matter
            meta_data.push_str(&line);
            meta_data.push('\n');
        } else if line.trim().starts_with("```") && !current_lang.is_empty() {
            // End of a code block
            current_lang.clear();
        } else if line.trim().starts_with("```") {
            // Start of a code block, determine its language
            if line.contains(".python") {
                current_lang = "python".to_string();
            } else if line.contains(".rust") {
                current_lang = "rust".to_string();
            }

            if !code_blocks.contains_key(&current_lang) {
                code_blocks.insert(current_lang.clone(), String::new());
            }
        } else if !current_lang.is_empty() {
            // Inside a code block
            if let Some(code) = code_blocks.get_mut(&current_lang) {
                code.push_str(&line);
                code.push('\n');
            }
        }
    }

    println!("Extracted YAML metadata:\n{}", meta_data);

    // Deserialize YAML front matter, removing trailing '---' if present
    let cleaned_meta_data = meta_data.trim_end_matches("---").trim();
    let meta: MarkdownMeta = serde_yaml::from_str(cleaned_meta_data).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("YAML parsing error: {}", e),
        )
    })?;

    for (lang, code) in &code_blocks {
        let extension = match lang.as_str() {
            "python" => "py",
            "rust" => "rs",
            _ => continue,
        };

        let mut output_filename = meta.output_filename.clone();
        output_filename.push_str(&format!(".{}", extension));

        // Write the extracted code to the output file
        let mut output_file = File::create(&output_filename)?;
        output_file.write_all(code.as_bytes())?;

        println!("{} code extracted to {}", lang, output_filename);
    }

    Ok(())
}
