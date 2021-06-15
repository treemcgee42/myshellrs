use std::path::Path;
use std::fs;
use std::error::Error;
use std::time::SystemTime;

use stdout_tables::tables::Table;
use stdout_tables::themes::Theme;

pub fn ls(dir: &Path) -> Result<(), Box<Error>> {
    if dir.is_dir() {
        let headers: Vec<(Option<usize>,String)> = vec![
            (Some(3),String::from("#")), (Some(15),String::from("name")), 
            (Some(6),String::from("type")), (Some(7),String::from("size")), 
            (Some(20),String::from("modified"))
        ];

        let mut data: Vec<Vec<String>> = Vec::new();

        for (i,entry) in fs::read_dir(dir)?.enumerate() {
            let entry = entry?;
            let metadata = entry.metadata().unwrap();

            let name = entry
                .file_name()
                .into_string()
                .or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
            let file_type: String;
            let file_size: u64 = metadata.len();
            let last_modified: String = 
                match metadata.modified() {
                    Ok(n) => format!("{}",SystemTime::now().duration_since(n).unwrap().as_secs()),
                    _ => String::from(""),
                };
            
            if entry.file_type()?.is_dir() {
                file_type = String::from("dir");
            } else {
                file_type = String::from("file"); 
            }

            data.push(
                vec![
                    format!("{}",i), 
                    name, 
                    file_type,
                    format!("{}",file_size),
                    last_modified
                ]
            );
        }

        // make the table
        let t: Table = Table::make(headers,data);
        t.draw(Theme::heavy());
    }
    Ok(())
}