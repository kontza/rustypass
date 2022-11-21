//! Here be config file operations
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use self::config_dir::ConfigDirInterface;
mod config_dir;

const APP_NAME: &str = "rustypass";
const CONFIG_NAME: &str = "config.toml";
pub const SCAN_DIRECTORY_VALUE: &str = "scan-directory";

trait ConfigStoreInterface {
    fn get_config(&mut self, value: &str) -> Result<PathBuf, Error>;
}

#[derive(Debug)]
struct ConfigStore {
    config_dir: Box<dyn ConfigDirInterface>,
}
impl ConfigStoreInterface for ConfigStore {
    fn get_config(&mut self, value: &str) -> Result<PathBuf, Error> {
        Err(Error::new(ErrorKind::Other, "Not implemented!"))
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{ConfigStoreInterface, SCAN_DIRECTORY_VALUE};

    use super::config_dir::{self, ConfigDirInterface, TestConfigDir};
    use super::ConfigStore;

    #[test]
    fn config_gets_the_expected_base_dir() {
        let mut bd = ConfigStore {
            config_dir: Box::new(TestConfigDir),
        };
        match bd.get_config(SCAN_DIRECTORY_VALUE) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }
}
