use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct FileStructureEntry {
    path: String,
    #[serde(rename = "type")]
    entry_type: String,
    #[serde(default)]
    content: String,
    #[serde(default)]
    children: Option<Vec<FileStructureEntry>>,
}

fn create_dir_all<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    fs::create_dir_all(&path)?;
    Ok(())
}

fn create_file<P: AsRef<Path>>(path: P, content: &str) -> std::io::Result<()> {
    fs::write(path, content)?;
    Ok(())
}

fn process_entry(entry: &FileStructureEntry, base_path: &Path) -> std::io::Result<()> {
    let full_path = base_path.join(&entry.path);

    if entry.entry_type == "dir" {
        create_dir_all(&full_path)?;
        if let Some(children) = &entry.children {
            for child in children {
                process_entry(child, &full_path)?;
            }
        }
    } else if entry.entry_type == "file" {
        if let Some(parent) = full_path.parent() {
            create_dir_all(parent)?;
        }
        create_file(full_path, &entry.content)?;
    }

    Ok(())
}

fn print_help_message() {
    println!("The JSON file should follow this structure:");
    println!("[");
    println!("  {{");
    println!("    \"path\": \"path/to/item\",");
    println!("    \"type\": \"dir\" | \"file\",");
    println!("    \"content\": \"optional file content\",");
    println!("    \"children\": [");
    println!("      {{");
    println!("        \"path\": \"path/to/child\",");
    println!("        \"type\": \"dir\" | \"file\",");
    println!("        \"content\": \"optional file content\"");
    println!("      }}");
    println!("    ] (optional)");
    println!("  }}");
    println!("]");
}

fn main() -> std::io::Result<()> {
    // Prompt user for the base path
    print!("Enter the base path for the project: ");
    io::stdout().flush()?;

    let mut base_path = String::new();
    io::stdin().read_line(&mut base_path)?;
    let base_path = base_path.trim();

    // Ensure the base path is valid
    if base_path.is_empty() {
        eprintln!("Base path cannot be empty.");
        return Ok(());
    }

    // Prompt user for the JSON file path
    print!("Enter the path to the file structure JSON: ");
    io::stdout().flush()?;

    let mut json_path = String::new();
    io::stdin().read_line(&mut json_path)?;
    let json_path = json_path.trim();

    // Ensure the JSON file path is valid
    if json_path.is_empty() {
        eprintln!("JSON file path cannot be empty.");
        return Ok(());
    }

    // Read and parse the JSON file
    let file_content = match fs::read_to_string(json_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading JSON file: {}", err);
            print_help_message();
            return Ok(());
        }
    };

    let entries: Result<Vec<FileStructureEntry>, serde_json::Error> =
        serde_json::from_str(&file_content);

    match entries {
        Ok(entries) => {
            // Create directories and files
            let base_path = Path::new(base_path);
            for entry in entries {
                if let Err(err) = process_entry(&entry, base_path) {
                    eprintln!("Error creating files: {}", err);
                    return Ok(());
                }
            }
            println!("File structure created successfully!");
        }
        Err(err) => {
            eprintln!("Error parsing JSON file: {}", err);
            print_help_message();
        }
    }

    Ok(())
}
