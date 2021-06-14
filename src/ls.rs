use std::path::Path;
use std::fs;
use std::error::Error;

pub fn ls(dir: &Path) -> Result<(), Box<Error>> {
    let mut files = Vec::new();
    let mut dirs = Vec::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                dirs.push(entry);
            } else {
                files.push(entry);
            }
        }

        println!("Directories:");
        for file in dirs.iter() {
            let file_name = file
            .file_name()
            .into_string()
            .or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
            println!("{}", file_name);
        }

        println!("Files:");
        for file in files.iter() {
            let file_name = file
            .file_name()
            .into_string()
            .or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
            println!("{}", file_name);
            println!("{:?}", file.metadata())
        }
    }
    Ok(())
}