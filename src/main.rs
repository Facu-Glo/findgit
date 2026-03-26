mod cache;
mod config;
mod fzf;
mod git;
mod models;
mod repo;
mod runtime;

const DEFAULT_CONFIG_PATH: &str = "~/.config/findgit/config.json";

fn main() {
    runtime::init();

    let args: Vec<String> = std::env::args().collect();
    let reload = args.iter().any(|a| a == "--reload");

    let app_config = config::load_config(DEFAULT_CONFIG_PATH);

    let repo_paths = if reload {
        let paths = repo::collect_repo_paths(&app_config);
        cache::save_cache(&paths);
        paths
    } else {
        match cache::load_cache() {
            Some(paths) => {
                let filtered = cache::filter_existing(paths);
                filtered
            }
            None => {
                let paths = repo::collect_repo_paths(&app_config);
                cache::save_cache(&paths);
                paths
            }
        }
    };

    let mut repos = repo::build_repos(&repo_paths);
    repo::sort_repos(&mut repos);

    let mut child = fzf::build_fzf_command()
        .spawn()
        .expect("Failed to start fzf");

    if let Some(mut stdin) = child.stdin.take() {
        fzf::send_repos_to_fzf(&repos, &mut stdin);
    }

    fzf::handle_fzf_output(child);
}
