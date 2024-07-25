use std::fs::{self, File};
use std::io::{self, Write};
use pulldown_cmark::{Parser, html::push_html};

/// Translates a markdown file to HTML and saves it to the specified output path.
pub fn translate_markdown_to_html(input_path: &str, output_path: &str) -> io::Result<()> {
    let markdown_input = fs::read_to_string(input_path)?;
    let parser = Parser::new(&markdown_input);
    let mut html_output = String::new();
    push_html(&mut html_output, parser);

    let mut output_file = File::create(output_path)?;
    output_file.write_all(html_output.as_bytes())?;

    Ok(())
}
