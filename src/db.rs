use std::fs::OpenOptions;
use std::{fs::File, io::Read, path::Path};
use std::io::{prelude::*, SeekFrom};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntryShape {
    pub name: String,
    pub band: String,
    pub timestamp: u128
}

pub struct Db {
    data: Vec<EntryShape>,
    db_path: Path
}

impl Db {
    pub fn new(db_path: &Path) -> Db {
        let mut file = match OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(db_path)
            {
                Err(why) => panic!("couldn't open {}: {}", db_path.display(), why),
                Ok(file) => file,
            };

        let mut raw_data = String::new();
        match file.read_to_string(&mut raw_data) {
            Err(why) => {println!("failed to read {}: {}\n defaulting to empty file", db_path.display(), why); raw_data = String::from("[]\n");},
            Ok(_) => ()
        }

        let data = match serde_json::from_str::<Vec<EntryShape>>(&raw_data) {
            Err(why) => {println!("could not parse data in {}: {}\ndefaulting to empty data", db_path.display(), why); vec![]},
            Ok(data) => data
        };

        file = match OpenOptions::new()
            .write(true)
            .truncate(truncate)
            .create(true)
            .open(db_path)
            {
                Err(why) => panic!("couldn't open {}: {}", db_path.display(), why),
                Ok(file) => file,
            };

        return Db {
            data,
            db_path
        };
    }

    fn read(&self) {

    }

    pub fn insert(&mut self, entry: EntryShape) {
        let x = EntryShape {
            name: String::from("a"),
            band: String::from("b"),
            timestamp: 123
        };

        match self.data.iter_mut().find(|e| e.band == entry.band) {
            Some(old_entry) => *old_entry = x,
            None => self.data.push(entry)
        };
    }

    pub fn commit(&mut self) {
        let raw_data = match serde_json::to_string_pretty(&self.data) {
            Err(why) => panic!("failed to convert data to string {:#?}: {}", self.data, why),
            Ok(str_json) => str_json
        };

        match self.db.seek(SeekFrom::Start(0)) {
            Err(why) => panic!("failed to flush DB before write: {}", why),
            Ok(_) => ()
        };

        println!("{}", raw_data);

        match self.db.write_all(raw_data.as_bytes()) {
            Err(why) => panic!("failed to write into DB file: {}", why),
            Ok(_) => ()
        };

    }
}
