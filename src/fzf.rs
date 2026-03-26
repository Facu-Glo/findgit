use crate::models::Repo;
use colored::*;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn build_fzf_command() -> Command {
    let mut cmd = Command::new("fzf");

    cmd.arg("--ansi")
        .arg("--delimiter")
        .arg("\t")
        .arg("--with-nth")
        .arg("2")
        .arg("--style=full")
        .arg("--layout=reverse")
        .arg("--height=50%")
        .arg("--tiebreak=length");

    let preview_script = r#"
        git_out=$(git -C {1} -c color.status=always status -s 2>/dev/null);
        echo -e '\x1b[1;36m󰊢 GIT STATUS\x1b[0m';
        
        if [ -z "$git_out" ]; then
            echo -e '\x1b[3;90mNo pending changes\x1b[0m';
        else
            echo "$git_out";
        fi;

        echo -e '\n\x1b[1;34m CONTENTS\x1b[0m';

        contents=$(eza --color=always -al --icons {1} 2>/dev/null);
        if [ -z "$contents" ]; then
            echo -e '\x1b[3;90mDirectory is empty\x1b[0m';
        else
            echo "$contents";
        fi
    "#;

    cmd.arg("--preview").arg(preview_script);
    cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

    cmd
}

pub fn prettify_path(path: &str) -> String {
    if let Ok(home) = std::env::var("HOME") {
        if path.starts_with(&home) {
            return path.replacen(&home, "~", 1);
        }
    }
    path.to_string()
}

pub fn send_repos_to_fzf(repos: &[Repo], stdin: &mut std::process::ChildStdin) {
    for (i, r) in repos.iter().enumerate() {
        let index = (i + 1).to_string().truecolor(128, 128, 128);

        let pretty = prettify_path(&r.path);
        let colored_path = if r.status_code == 1 {
            pretty.red()
        } else {
            pretty.white()
        };

        if let Err(e) = writeln!(stdin, "{}\t{}  {}", r.path, index, colored_path) {
            eprintln!("Error writing to fzf: {}", e);
            break;
        }
    }
}

pub fn handle_fzf_output(child: std::process::Child) {
    let output = child.wait_with_output().expect("Failed to read fzf output");

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        if let Some(path) = result.split('\t').next() {
            println!("{}", path.trim());
        }
    }
}
