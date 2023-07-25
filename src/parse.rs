use crate::job::Job;

pub fn process_file(text: String) -> Vec<Job> {
    let csv: Vec<Vec<String>> = text
        .split('\n')
        .map(|s| s.split(',').map(|s| s.to_owned()).collect())
        .collect();

    let date_line = &csv[2][0];
    let day = date_line
        .split_whitespace()
        .nth(1)
        .expect("Could not read date line")
        .to_owned();
    let date = date_line
        .split_whitespace()
        .nth(4)
        .expect("Could not read date line")
        .parse()
        .expect("Could not read date line");

    println!("Collecting shifts from {day} the {date}th");

    let mut jobs = vec![
        // MANAGERS
        managers(&csv[6][0], &csv[6][2]),
        managers(&csv[7][0], &csv[7][2]),
        managers(&csv[8][0], &csv[8][2]),
        // CASHIERS
        with_job("Cashier", &csv[11][0], &csv[11][2]),
        with_job("Cashier", &csv[12][0], &csv[12][2]),
        with_job("Cashier", &csv[13][0], &csv[13][2]),
        with_job("Cashier", &csv[14][0], &csv[14][2]),
        //with_job("Cashier", &csv[15][0], &csv[15][2]),
        parens(&csv[15][0], &csv[15][2]), // Not sure if this is unique to the one I have
        // WORKERS
        parens(&csv[18][0], &csv[18][2]), //Griddle
        parens(&csv[20][0], &csv[20][2]), // Food Super
        // Start Group 1
        parens(&csv[21][0], &csv[21][2]),
        parens(&csv[21][0], &csv[21][6]),
        parens(&csv[21][0], &csv[22][2]),
        parens(&csv[21][0], &csv[22][6]),
        parens(&csv[21][0], &csv[23][2]),
        // End Group 1
        parens(&csv[24][0], &csv[24][2]),
        parens(&csv[25][0], &csv[25][2]),
        // Start Group 2
        parens(&csv[26][0], &csv[26][2]),
        parens(&csv[26][0], &csv[26][6]),
        parens(&csv[26][0], &csv[27][2]),
        parens(&csv[26][0], &csv[27][6]),
        parens(&csv[26][0], &csv[28][2]),
        // End Group 2
        parens(&csv[29][0], &csv[29][2]),
        parens(&csv[29][0], &csv[29][6]),
        // Skips cleaners (unique)
        parens(&csv[31][0], &csv[31][2]),
        // Start Group 4
        parens(&csv[32][0], &csv[32][2]),
        parens(&csv[32][0], &csv[33][2]),
        parens(&csv[32][0], &csv[33][6]),
    ];
    jobs.extend_from_slice(&cleaners(&csv[30][0], &csv[30][2]));

    jobs.into_iter()
        .map(|(time, role, name)| Job {
            day: day.clone(),
            date,
            role,
            time,
            name,
        })
        .collect()
}

fn parens(time: &str, input: &str) -> (String, String, Option<String>) {
    input.split_once(')').map_or(
        (
            time.to_owned(),
            "UNKNOWN".to_owned(),
            Some(input.trim().to_lowercase()).filter(|s| !s.is_empty()),
        ),
        |(role, name)| {
            (
                time.to_owned(),
                role.trim_start_matches([' ', '(', '1', '2', '3', '5'])
                    .to_owned(),
                Some(name.trim().to_lowercase()).filter(|s| !s.is_empty()),
            )
        },
    )
}

fn managers(time: &str, input: &str) -> (String, String, Option<String>) {
    let name = match input.split_once(':') {
        Some((_, name)) => name.trim().to_lowercase(),
        None => input.trim().to_lowercase(),
    };
    (
        time.to_owned(),
        "Manager".to_owned(),
        Some(name).filter(|s| !s.is_empty()),
    )
}

fn with_job(job: &str, time: &str, input: &str) -> (String, String, Option<String>) {
    (
        time.to_owned(),
        job.to_owned(),
        Some(input.trim().to_lowercase()).filter(|s| !s.is_empty()),
    )
}

fn cleaners(time: &str, input: &str) -> [(String, String, Option<String>); 2] {
    let (_, _, names) = parens(time, input);
    match names {
        None => [
            (time.to_owned(), "Cleaner".to_owned(), None),
            (time.to_owned(), "Cleaner".to_owned(), None),
        ],
        Some(names) => {
            let Some((first, second)) = names.split_once('&') else {
                return [
                    (time.to_owned(), "Cleaner".to_owned(), Some(names.trim().to_lowercase())),
                    (time.to_owned(), "Cleaner".to_owned(), None),
                ];
            };
            [
                (
                    time.to_owned(),
                    "Cleaner".to_owned(),
                    Some(first.trim().to_lowercase()),
                ),
                (
                    time.to_owned(),
                    "Cleaner".to_owned(),
                    Some(second.trim().to_lowercase()),
                ),
            ]
        }
    }
}
