use std::fs::{File, OpenOptions};
use std::io::{Seek, Write};
use std::{io::Read, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntryShape {
    pub name: String,
    pub artist: String,
    pub timestamp: u128,
}

pub struct Db<'a> {
    old_data: Vec<EntryShape>,
    new_data: Vec<EntryShape>,
    db_path: &'a Path,
}

impl Db<'_> {
    pub fn new(db_path: &Path) -> Db {
        let file = Self::get_readable_db_file(&db_path);
        let data = Self::get_db_data(&file);

        return Db { old_data: data, db_path, new_data: vec![] };
    }

    pub fn get_by_artist_name(&self, artist_name: &str) -> Option<&EntryShape> {
        self.old_data.iter().find(|e| e.artist == artist_name)
    }

    fn get_db_data(mut db: &File) -> Vec<EntryShape> {
        let mut raw_data = String::new();

        match db.read_to_string(&mut raw_data) {
            Err(why) => {
                println!("failed to read db file: {}\n defaulting to empty file", why);
                raw_data = String::from("[]\n");
            }
            Ok(_) => (),
        }

        match serde_json::from_str::<Vec<EntryShape>>(&raw_data) {
            Err(why) => {
                println!(
                    "could not parse data in db file: {}\ndefaulting to empty data",
                    why
                );
                vec![]
            }
            Ok(data) => data,
        }
    }

    fn get_readable_db_file(path: &Path) -> File {
        match OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(path)
        {
            Err(why) => panic!("couldn't open {}: {}", path.display(), why),
            Ok(file) => file,
        }
    }

    pub fn insert(&mut self, entry: EntryShape) {
        match self.new_data.iter_mut().find(|e| e.artist == entry.artist) {
            Some(old_entry) => *old_entry = entry,
            None => self.new_data.push(entry),
        };
    }

    pub fn commit(&mut self) {
        let mut db = match OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(self.db_path)
        {
            Err(why) => panic!("couldn't open {}: {}", self.db_path.display(), why),
            Ok(file) => file,
        };

        let raw_data = match serde_json::to_string_pretty(&self.new_data) {
            Err(why) => panic!("failed to convert data to string {:#?}: {}", self.new_data, why),
            Ok(str_json) => str_json,
        };

        match db.rewind() {
            Err(why) => panic!("failed to rewind DB before write: {}", why),
            Ok(_) => (),
        };

        match db.write_all(raw_data.as_bytes()) {
            Err(why) => panic!("failed to write into DB file: {}", why),
            Ok(_) => (),
        };
    }
}
