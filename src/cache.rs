use std::fs;
use std::path::{Path, PathBuf};

const CACHE_PATH: &str = "~/.cache/findgit/repos.json";

fn get_cache_path() -> PathBuf {
    PathBuf::from(shellexpand::tilde(CACHE_PATH).into_owned())
}

pub fn load_cache() -> Option<Vec<PathBuf>> {
    let path = get_cache_path();

    let data = fs::read_to_string(path).ok()?;
    let paths: Vec<String> = serde_json::from_str(&data).ok()?;

    Some(paths.into_iter().map(PathBuf::from).collect())
}

pub fn save_cache(paths: &[PathBuf]) {
    let path = get_cache_path();

    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let paths_str: Vec<&str> = paths
        .iter()
        .filter_map(|p| p.to_str())
        .collect();

    if let Ok(json) = serde_json::to_string(&paths_str) {
        let _ = fs::write(path, json);
    }
}

pub fn filter_existing(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    paths
        .into_iter()
        .filter(|p| Path::new(p).exists())
        .collect()
}
