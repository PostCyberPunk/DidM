pub mod behaviour;
mod config;
pub mod plan;
pub mod profile;
mod skip_check;

pub use behaviour::Behaviour;
pub use config::DidmConfig;
pub use plan::Plan;
pub use profile::Profile;
pub use skip_check::SkipCheck;

fn is_true(val: &bool) -> bool {
    *val
}
fn default_true() -> bool {
    true
}
fn is_false(val: &bool) -> bool {
    !*val
}
