use regex::Regex;
use serde::Deserialize;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct MarkdownMeta {
    pub output_filename: String,
}

pub fn extract_code_from_markdown(file_path: &str) -> io::Result<()> {
    let path = Path::new(file_path);
    let mut file = File::open(&path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Extract YAML front matter
    let re = Regex::new(r"(?s)^---\n(.*?)\n---").unwrap();
    let caps = re.captures(&contents).ok_or(io::Error::new(
        io::ErrorKind::InvalidData,
        "Unable to find YAML front matter",
    ))?;

    let yaml_str = caps.get(1).unwrap().as_str();
    let meta: MarkdownMeta = serde_yaml::from_str(yaml_str).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("YAML parsing error: {}", e),
        )
    })?;

    // Extract Python code
    let re = Regex::new(r"```{\.python[^}]*}\n(.*?)\n```").unwrap();
    let caps = re.captures(&contents).ok_or(io::Error::new(
        io::ErrorKind::InvalidData,
        "Unable to find Python code block",
    ))?;

    let code = caps.get(1).unwrap().as_str();
    let output_filename = meta.output_filename.clone();

    // Write the extracted code to the output file
    let mut output_file = File::create(output_filename)?;
    output_file.write_all(code.as_bytes())?;

    println!("Python code extracted to {}", meta.output_filename);

    Ok(())
}
