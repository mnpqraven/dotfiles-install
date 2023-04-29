mod types;
use schemars::schema_for;
pub use types::ConfigFile;

pub fn generate_schema() -> String {
    let schema = schema_for!(ConfigFile);
    serde_json::to_string_pretty(&schema).unwrap()
}
