pub mod impls;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct ConfigFile {
    pub profiles: Vec<Profile>,
    pub tasks: Vec<Task>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct Task {
    pub name: String,
    // not specifying profile = selecting all of them
    pub profile: Option<Vec<Profile>>,
    pub cmds: Vec<TaskCommand>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct TaskCommand {
    pub description: Option<String>,
    pub install_type: InstallType,
    // currently not supporting user-provided args
    pub args: Vec<String>,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Display, JsonSchema, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Profile {
    Default,
    Work,
    Home,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum InstallType {
    Pacman,
    Yay,
    Cargo,
    Script,
}
