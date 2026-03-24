use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub search_paths: Vec<String>,
    pub ignore_paths: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Repo {
    pub path: String,
    pub status_code: u8,
    pub timestamp: i64,
}
