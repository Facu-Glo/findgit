mod config;
mod fzf;
mod git;
mod models;
mod repo;
mod runtime;

use fzf::*;
use repo::*;

const DEFAULT_CONFIG_PATH: &str = "~/.config/findgit/config.json";

fn main() {
    runtime::init();

    let app_config = config::load_config(DEFAULT_CONFIG_PATH);

    let repo_paths = collect_repo_paths(&app_config);
    let mut repos = build_repos(&repo_paths);

    sort_repos(&mut repos);

    let mut child = build_fzf_command().spawn().expect("Failed to start fzf");

    if let Some(mut stdin) = child.stdin.take() {
        send_repos_to_fzf(&repos, &mut stdin);
    }

    handle_fzf_output(child);
}
