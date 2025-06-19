mod config;
pub use config::DidmConfig;

pub mod profile;
pub use profile::Profile;

mod check_config;
pub use check_config::CheckConfig;

mod composition;
pub use composition::Composition;

mod behaviour;
pub use behaviour::Behaviour;

fn is_true(val: &bool) -> bool {
    *val
}
fn default_true() -> bool {
    true
}
fn is_false(val: &bool) -> bool {
    !*val
}
