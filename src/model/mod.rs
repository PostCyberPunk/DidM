pub mod behaviour;
mod check_config;
mod config;
pub mod plan;
pub mod profile;

pub use behaviour::Behaviour;
pub use check_config::CheckConfig;
pub use config::DidmConfig;
pub use plan::Plan;
pub use profile::Profile;

fn is_true(val: &bool) -> bool {
    *val
}
fn default_true() -> bool {
    true
}
fn is_false(val: &bool) -> bool {
    !*val
}
