use std::path::Path;
use std::fs;
use std::error::Error;
use std::time::SystemTime;

use stdout_tables::tables::Table;
use stdout_tables::themes::Theme;

pub fn list_contents(dir: &Path) -> Result<(), Box<dyn Error>> {
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
            let file_size: String = parse_bytes(&metadata.len());
            let last_modified: String = 
                match metadata.modified() {
                    Ok(n) => parse_secs(
                        &SystemTime::now().duration_since(n).unwrap().as_secs()
                    ),
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
                    file_size,
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

fn parse_secs(secs: &u64) -> String {
    // values in seconds
    let minute:u64 = 60;
    let hour:u64 = minute*60;
    let day = hour*24;
    let week = day*27;
    let year = week*52;

    // by minutes (0-59 mins)
    if secs<&hour {
        format!("{} minutes ago", secs/&minute)
    }
    // by hours (1-23 hrs)
    else if secs<&day {
        format!("{} hours ago", secs/&hour)
    }
    // by days (1-7 days)
    else if secs<&week {
        format!("{} days ago", secs/&day)
    }
    // by weeks (1-51 weeks)
    else if secs<&year {
        format!("{} weeks ago", secs/&week)
    }
    // by years (1+ yrs)
    else {
        format!("{} years ago", secs/&year)
    }
}

fn parse_bytes(bs: &u64) -> String {
    // in terms of bytes
    let kb: u64 = 1000;
    let mb: u64 = 1000*kb;
    let gb: u64 = 1000*mb;

    // by bytes (0-999 b)
    if bs<&kb {
        format!("{} b", bs)
    }
    // by kilobytes (1-999 kb)
    else if bs<&mb {
        format!("{} kb", bs/&kb)
    }
    // by megabytes (1-999 mb)
    else if bs<&gb {
        format!("{} mb", bs/&mb)
    }
    // by gigabytes (1+ gb)
    else {
        format!("{} gb", bs/&gb)
    }
}