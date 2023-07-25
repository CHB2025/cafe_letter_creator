use anyhow::Result;
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::Read,
};

use print::print;

mod job;
mod parse;
mod print;

fn main() -> Result<()> {
    let dir = env::current_dir()?;
    let csvs: Vec<_> = fs::read_dir(dir)?
        .filter_map(|entry| match entry {
            Ok(e) if e.path().extension().is_some_and(|e| e == "csv") => Some(e),
            _ => None,
        })
        .collect();
    if csvs.len() == 0 {
        println!("No csvs in current directory");
        return Ok(());
    }

    let mut person_shifts = HashMap::new();
    for file in csvs {
        println!("Reading from {} ...", file.path().display());

        let mut file = File::open(file.path())?;
        let mut csv_string = String::new();
        file.read_to_string(&mut csv_string)?;
        for j in parse::process_file(csv_string) {
            if let Some(name) = j.name.clone() {
                person_shifts.entry(name).or_insert(vec![]).push(j);
            }
        }
    }

    _ = fs::create_dir("letters");

    for (person, shifts) in person_shifts {
        print(person, shifts)?;
    }

    Ok(())
}
