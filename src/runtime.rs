pub fn init_thread_pool() {
    init_thread_pool_with((num_cpus::get() / 2).max(1));
}

pub fn init_thread_pool_with(threads: usize) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()
        .ok();
}

pub fn init_colors() {
    colored::control::set_override(true);
}

pub fn init() {
    init_thread_pool();
    init_colors();
}
