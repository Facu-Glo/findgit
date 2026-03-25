use crate::{
    git,
    models::{Config, Repo},
};
use rayon::prelude::*;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub fn collect_repo_paths(app_config: &Config) -> Vec<PathBuf> {
    let mut repo_paths_raw: Vec<PathBuf> = app_config
        .search_paths
        .par_iter()
        .flat_map(|p| {
            let expanded = shellexpand::tilde(p).into_owned();
            if Path::new(&expanded).exists() {
                git::find_git_dirs(&expanded, &app_config.ignore_paths)
            } else {
                Vec::new()
            }
        })
        .collect();

    let unique_paths: HashSet<PathBuf> = repo_paths_raw.drain(..).collect();
    unique_paths.into_iter().collect()
}

pub fn build_repos(repo_paths: &[PathBuf]) -> Vec<Repo> {
    repo_paths
        .par_iter()
        .filter_map(|p| git::get_repo_details(p))
        .collect()
}

pub fn sort_repos(repos: &mut [Repo]) {
    repos.sort_unstable_by(|a, b| {
        a.status_code
            .cmp(&b.status_code)
            .then_with(|| b.timestamp.cmp(&a.timestamp))
    });
}
