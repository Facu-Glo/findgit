mod config;
mod git;
mod models;

use crate::models::Repo;
use colored::*;
use rayon::prelude::*;
use std::collections::HashSet;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get() / 2)
        .build_global()
        .ok();

    colored::control::set_override(true);
    let app_config = config::load_config();

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
    let repo_paths: Vec<PathBuf> = unique_paths.into_iter().collect();

    let mut repos: Vec<Repo> = repo_paths
        .par_iter()
        .filter_map(|p| git::get_repo_details(&p))
        .collect();

    repos.sort_unstable_by(|a, b| {
        a.status_code
            .cmp(&b.status_code)
            .then_with(|| b.timestamp.cmp(&a.timestamp))
    });

    let mut child = Command::new("fzf")
        .args([
            "--ansi",
            "--delimiter",
            "\t",
            "--with-nth",
            "2,3",
            "--preview",
            "echo -e '\x1b[1;36m󰊢 GIT STATUS\x1b[0m'; \
             git -C {1} -c color.status=always status -s; \
             echo -e '\n\x1b[1;34m CONTENTS\x1b[0m'; \
             eza --color=always -al --icons {1}",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start fzf");

    if let Some(mut stdin) = child.stdin.take() {
        for (i, r) in repos.iter().enumerate() {
            let index = (i + 1).to_string().white();
            let colored_path = if r.status_code == 1 {
                r.path.red()
            } else {
                r.path.truecolor(128, 128, 128)
            };

            if let Err(e) = writeln!(stdin, "{}\t{}\t{}", r.path, index, colored_path) {
                eprintln!("Error writing to fzf: {}", e);
                break;
            }
        }
    }

    let output = child.wait_with_output().expect("Failed to read fzf output");

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        if let Some(path) = result.split('\t').next() {
            println!("{}", path.trim());
        }
    }
}
