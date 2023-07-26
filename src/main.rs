use anyhow::Result;
use printpdf::{BuiltinFont, Mm, PdfDocument};
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{BufWriter, Read},
};

use print::print;

mod job;
mod parse;
mod print;

fn main() -> Result<()> {
    let mut dir = env::current_exe()?;
    dir.pop();
    let csvs: Vec<_> = fs::read_dir(dir.clone())?
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
    let mut pdf_path = dir;
    pdf_path.push("letters");
    let (full_doc, _, _) = PdfDocument::new("Fair Schedule", Mm(215.9), Mm(279.4), "Layer 1");
    let full_font = full_doc.add_builtin_font(BuiltinFont::Helvetica)?;

    for (person, shifts) in person_shifts {
        let (page, layer) = full_doc.add_page(Mm(215.9), Mm(279.4), "Layer 1");
        let curr_layer = full_doc.get_page(page).get_layer(layer);
        print(curr_layer, &full_font, person.clone(), shifts.clone());

        let (doc, page, layer) = PdfDocument::new(
            person.clone() + "Fair Schedule",
            Mm(215.9),
            Mm(279.4),
            "Layer 1",
        );
        let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;
        let curr_layer = doc.get_page(page).get_layer(layer);
        print(curr_layer, &font, person.clone(), shifts);
        pdf_path.push(person.clone() + " fair schedule.pdf");
        doc.save(&mut BufWriter::new(File::create(pdf_path.clone())?))?;
        pdf_path.pop();
    }

    pdf_path.pop();
    pdf_path.push("All letters.pdf");
    full_doc.save(&mut BufWriter::new(File::create(pdf_path)?))?;

    Ok(())
}
