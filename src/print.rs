use std::{fs::File, io::BufWriter};

use anyhow::Result;
use printpdf::{BuiltinFont, Mm, PdfDocument};

use crate::job::Job;

pub fn print(name: String, mut jobs: Vec<Job>) -> Result<()> {
    let (doc, page1, layer1) = PdfDocument::new("Fair Schedule", Mm(215.9), Mm(279.4), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;

    current_layer.begin_text_section();

    // Setup
    current_layer.set_font(&font, 18.0);
    current_layer.set_text_cursor(Mm(20.0), Mm(254.4));
    current_layer.set_line_height(20.0);

    let cap_name = name
        .clone()
        .split_whitespace()
        .map(|name| name[0..1].to_uppercase() + &name[1..])
        .collect::<Vec<_>>()
        .join(" ");
    let greeting = format!("Hi {},", cap_name);
    current_layer.write_text(greeting, &font);
    current_layer.add_line_break();
    current_layer.add_line_break();
    current_layer.add_line_break();
    let greeting = format!("Thanks for volunteering to work at the Cornerstone Café!");
    current_layer.write_text(greeting, &font);
    current_layer.add_line_break();
    let greeting = format!("Here is when you are scheduled to work at the café this week:");
    current_layer.write_text(greeting, &font);
    current_layer.add_line_break();
    current_layer.add_line_break();
    current_layer.add_line_break();

    jobs.sort_by_key(|j| j.date);
    for job in jobs {
        let text = format!(
            "{}, Aug. {}: {} from {}",
            job.day, job.date, job.role, job.time
        );
        current_layer.write_text(text, &font);
        current_layer.add_line_break();
    }

    current_layer.end_text_section();

    doc.save(&mut BufWriter::new(File::create(format!(
        "./letters/{name} fair schedule.pdf"
    ))?))?;
    Ok(())
}
