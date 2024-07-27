# File Structure Generator

This Rust application generates a file and directory structure based on a JSON configuration file. It allows you to quickly set up a predefined project structure with specified directories and files.

## Features

- Create directories and files based on a JSON structure.
- Supports nested directories and files.
- Can specify file contents in the JSON configuration.

## Requirements

- Rust (1.60 or higher)
- `serde` and `serde_json` crates

## Installation

1. **Clone the repository:**

   ```sh
   git clone <repository-url>
   cd <repository-directory>``

   ```

2. **Build the project:**

   ```sh
   cargo build --release
   ```

   This will produce an executable in the `target/release` directory.

## Usage

1. **Prepare a JSON configuration file:**

   Create a JSON file (e.g., `file_structure.json`) with the desired directory and file structure. The JSON should follow the format:

   ```json
   [
     {
       "path": "path/to/directory",
       "type": "dir",
       "children": [
         {
           "path": "file.txt",
           "type": "file",
           "content": "File content here"
         }
       ]
     }
   ]
   ```

   See the example JSON below for a basic test setup:

   ```json
   [
     {
       "path": "src",
       "type": "dir",
       "children": [
         {
           "path": "main.rs",
           "type": "file",
           "content": "// Main entry point of the Rust application"
         },
         {
           "path": "lib.rs",
           "type": "file",
           "content": "// Library code"
         }
       ]
     },
     {
       "path": "tests",
       "type": "dir",
       "children": [
         {
           "path": "unit_test.rs",
           "type": "file",
           "content": "// Unit tests"
         }
       ]
     },
     {
       "path": "Cargo.toml",
       "type": "file",
       "content": "[package]\nname = \"my_project\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n"
     }
   ]
   ```

2. **Run the application:**

   Execute the binary and provide the base path for the project and the path to the JSON configuration file.

   ```sh
   ./file_structure_generator
   ```

   When prompted, enter the base path for the project and the path to the JSON file.

3. **Example:**

   ```sh
   Enter the base path for the project: /path/to/project
   Enter the path to the file structure JSON: /path/to/file_structure.json
   ```

   The application will create the directories and files as specified in the JSON configuration.

## Notes

- Ensure the base path and JSON file paths are correct.
- The application will overwrite files if they already exist at the specified paths.

## Contributing

Feel free to submit issues or pull requests. Contributions are welcome!
