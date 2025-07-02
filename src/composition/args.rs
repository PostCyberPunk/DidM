#[derive(Debug, Clone)]
pub struct AppArgs {
    pub variants: Vec<String>,
    pub is_dryrun: bool,
    pub is_verbose: bool,
    pub is_debug: bool,
    pub show_tree: bool,
}
