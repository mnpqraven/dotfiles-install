use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct ConfigFile {
    profiles: Vec<Profile>,
    tasks: Vec<Task>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct Task {
    name: String,
    // not specifying profile = selecting all of them
    profile: Option<Vec<Profile>>,
    cmds: Vec<TaskCommand>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct TaskCommand {
    description: Option<String>,
    install_type: InstallType,
    // currently not supporting user-provided args
    args: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Profile {
    Default,
    Work,
    Home,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum InstallType {
    Pacman,
    Yay,
    Script,
}
