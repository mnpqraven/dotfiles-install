mod types;
use schemars::schema_for;
pub use types::ConfigFile;
pub use types::Profile;
pub use types::TaskCommand;

pub use types::impls::build_command;
pub use types::InstallType;
pub use types::Task;

pub fn generate_schema() -> String {
    let schema = schema_for!(ConfigFile);
    serde_json::to_string_pretty(&schema).unwrap()
}
