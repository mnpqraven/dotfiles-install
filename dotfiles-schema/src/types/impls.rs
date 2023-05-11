use tokio::{io::AsyncReadExt, process::Command};
use tracing::{debug, error, info};

use crate::{ConfigFile, TaskCommand};
use std::{
    error::Error,
    fs::File,
    io::{self, BufReader, Write},
    process::Stdio,
};

use super::InstallType;

impl ConfigFile {
    pub fn from_dir(path: &str) -> Result<Self, Box<dyn Error>> {
        let reader = File::open(path)?;
        let cfg: ConfigFile = serde_json::from_reader(BufReader::new(reader))?;
        Ok(cfg)
    }

    pub async fn from_url(url: &str) -> Result<Self, Box<dyn Error>> {
        Ok(reqwest::get(url).await?.json::<ConfigFile>().await?)
    }
}

impl InstallType {
    fn get_command(&self) -> Command {
        match self {
            InstallType::Pacman => {
                let mut cmd = Command::new("sudo");
                cmd.args(self.default_args())
                    .stdout(Stdio::piped())
                    .stdin(Stdio::piped());
                cmd
            }
            InstallType::Yay => todo!(),
            InstallType::Script => todo!(),
            InstallType::Cargo => todo!(),
        }
    }

    fn default_args(&self) -> Vec<String> {
        match self {
            InstallType::Pacman => vec!["pacman".into(), "-S".into(), "--noconfirm".into()],
            InstallType::Yay => vec![],
            InstallType::Cargo => vec![],
            InstallType::Script => vec![],
        }
    }
}

impl TaskCommand {
    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut packages = self.args.clone();
        let mut child = build_command(self.install_type, &mut packages)
            .stderr(Stdio::piped())
            .spawn()?;
        let status = child.wait().await;
        match status {
            Ok(status) if status.success() => {
                info!("Packages {} installed successfully", self.args.join(", "));
            }
            Ok(status) => {
                let mut stderr_buf = Vec::new();
                child.stderr.unwrap().read_to_end(&mut stderr_buf).await?;

                // NOTE: actual stderr
                io::stdout().write_all(&stderr_buf).unwrap();
                // TODO:
                let err_fmt = format!(
                    "failed installing package {} with status code {}",
                    self.args.join(", "),
                    status
                );
                error!(err_fmt);
                return Err(err_fmt.into());
            }
            Err(_) => {
                return Err("Command wasn't running".into());
            }
        }

        Ok(())
    }
}

pub fn build_command(install_type: InstallType, package: &mut Vec<String>) -> Command {
    // TODO: deprecation
    let mut cmd = install_type.get_command();
    // let mut args: Vec<String> = install_type.default_args();
    // args.append(package);

    debug!("Created command {}", package.join(" "));

    cmd.args(package)
        .stdout(Stdio::piped())
        .stdin(Stdio::piped());
    cmd
}
