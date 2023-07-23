#[derive(Debug, Clone)]
pub struct Job {
    pub day: String,
    pub date: u8,
    pub role: String,
    pub time: String,
    pub name: Option<String>,
}
