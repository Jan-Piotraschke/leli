# leli

What is **leli**? **leli** stands for "**le**gacy **li**terate".  
Its designed primarily to empower the use of the *AImM* (AI-maintained Microservices) architecture.  
**leli** prepares everything so that an AI can maintain and inspect compliant codebases by reading their HTML output files.  

Using **leli** is about coding with the end in mind: envisioning that your project will someday be a legacy project, which you yourself will not maintain anymore. But you want to ensure that the AI can maintain, explain, and customize it, understanding your literate words and thoughts behind it.

## Installation

Compile the project using the following command:

```bash
cargo build --release
```

## Usage

```bash
./target/release/leli extract --file example/math_operations.md
```
