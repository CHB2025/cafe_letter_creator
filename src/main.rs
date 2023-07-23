use anyhow::{anyhow, Result};
use std::{collections::HashMap, fs::File, io::Read};

use print::print;

mod job;
mod parse;
mod print;

fn main() -> Result<()> {
    let mut person_jobs = HashMap::new();
    let mut file = File::open("./test.csv")?;
    let mut csv_string = String::new();
    file.read_to_string(&mut csv_string)?;
    let jobs = parse::process_file(csv_string);
    for j in jobs.clone() {
        if let Some(name) = j.name.clone() {
            person_jobs.entry(name).or_insert(vec![]).push(j);
        }
    }

    let test = jobs[0].clone();
    print(test.name.clone().expect("missing name"), vec![test])?;

    Ok(())
}
