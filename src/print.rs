use printpdf::{IndirectFontRef, Mm, PdfLayerReference};

use crate::job::Job;

pub fn print(
    current_layer: PdfLayerReference,
    font: &IndirectFontRef,
    name: String,
    mut jobs: Vec<Job>,
) {
    current_layer.begin_text_section();

    // Setup
    current_layer.set_font(font, 14.0);
    current_layer.set_text_cursor(Mm(20.0), Mm(254.4));
    current_layer.set_line_height(20.0);

    let cap_name = name
        .clone()
        .split_whitespace()
        .map(|name| name[0..1].to_uppercase() + &name[1..])
        .collect::<Vec<_>>()
        .join(" ");
    let greeting = format!("Hi {},", cap_name);
    current_layer.write_text(greeting, font);
    current_layer.add_line_break();
    current_layer.add_line_break();
    current_layer.add_line_break();
    let greeting = format!("Thanks for volunteering to work at the Cornerstone Café!");
    current_layer.write_text(greeting, font);
    current_layer.add_line_break();
    let greeting = format!("Here is when you are scheduled to work at the café this week:");
    current_layer.write_text(greeting, font);
    current_layer.add_line_break();
    current_layer.add_line_break();
    current_layer.add_line_break();

    jobs.sort_by_key(|j| j.date);
    for job in jobs {
        let text = format!(
            "{}, Aug. {}: {} from {}",
            job.day, job.date, job.role, job.time
        );
        current_layer.write_text(&text, font);
        current_layer.add_line_break();
    }

    current_layer.add_line_break();
    current_layer.add_line_break();
    current_layer.add_line_break();
    current_layer.write_text("Please let me know if there are any issues.", font);
    current_layer.add_line_break();
    current_layer.add_line_break();
    current_layer.write_text("Cheers,", font);
    current_layer.add_line_break();
    // Add signature

    current_layer.end_text_section();
}
