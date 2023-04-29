use crate::ConfigFile;
use std::{error::Error, fs::File, io::BufReader};

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
