#[derive(Debug, Clone, Copy)]
pub struct AppArgs {
    pub is_dryrun: bool,
    pub is_verbose: bool,
    pub is_debug: bool,
    pub show_tree: bool,
}
