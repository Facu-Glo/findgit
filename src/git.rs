use crate::models::Repo;
use git2::{Repository, StatusOptions};
use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn find_git_dirs(root: &str, ignores: &[String]) -> Vec<PathBuf> {
    let mut cmd = Command::new("fd");

    cmd.arg("-H")
        .arg("-t")
        .arg("d")
        .arg("--max-depth")
        .arg("6")
        .arg("^\\.git$")
        .arg(root)
        .arg("-0");

    for ig in ignores {
        cmd.arg("-E").arg(ig);
    }

    let output = cmd.output().expect("fd no está instalado");

    if !output.status.success() {
        return Vec::new();
    }

    output
        .stdout
        .split(|b| *b == 0)
        .filter_map(|bytes| {
            let os_str = OsString::from_vec(bytes.to_vec());
            let path = PathBuf::from(os_str);

            path.parent().map(|p| p.to_path_buf())
        })
        .collect()
}

pub fn get_repo_details(path: &Path) -> Option<Repo> {
    let repo = Repository::open(path).ok()?;

    let timestamp = repo
        .head()
        .and_then(|h| h.peel_to_commit())
        .map(|c| c.time().seconds())
        .unwrap_or(0);

    let mut status_opts = StatusOptions::new();
    status_opts.include_untracked(true);
    let is_dirty = repo
        .statuses(Some(&mut status_opts))
        .map(|s| !s.is_empty())
        .unwrap_or(false);

    Some(Repo {
        path: path.to_string_lossy().into_owned(),
        status_code: if is_dirty { 1 } else { 2 },
        timestamp,
    })
}
