//! Here be config file operations
use config::{Config, ConfigError, File, FileFormat};

use self::config_dir::ConfigDirInterface;
mod config_dir;

pub const SCAN_DIRECTORY_VALUE: &str = "scan-directory";
const FAILED_TO_GET_CONFIG_DIR: &str = "FAILED_TO_GET_CONFIG_DIR";

trait ConfigStoreInterface {
    fn get_config(&mut self, value: &str) -> Result<String, ConfigError>;
}

#[derive(Debug)]
struct ConfigStore {
    config_dir: Box<dyn ConfigDirInterface>,
}
impl ConfigStoreInterface for ConfigStore {
    fn get_config(&mut self, value: &str) -> Result<String, ConfigError> {
        match self.config_dir.get() {
            None => Err(ConfigError::Message(FAILED_TO_GET_CONFIG_DIR.to_string())),
            Some(d) => match d.to_str() {
                None => Err(ConfigError::Message(FAILED_TO_GET_CONFIG_DIR.to_string())),
                Some(das) => match dirs::data_dir() {
                    Some(mut data_dir) => {
                        data_dir.push("gopass");
                        data_dir.push("stores");
                        let builder = Config::builder()
                            .set_default(SCAN_DIRECTORY_VALUE, data_dir.to_str())?
                            .add_source(File::new(das, FileFormat::Toml));
                        match builder.build() {
                            Ok(config) => config.get_string(value),
                            Err(e) => Err(e),
                        }
                    }
                    None => Err(ConfigError::Message(FAILED_TO_GET_CONFIG_DIR.to_string())),
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{ConfigStoreInterface, SCAN_DIRECTORY_VALUE};

    use super::config_dir::TestConfigDir;
    use super::ConfigStore;

    #[test]
    fn config_gets_the_expected_base_dir() {
        let mut bd = ConfigStore {
            config_dir: Box::new(TestConfigDir),
        };
        match bd.get_config(SCAN_DIRECTORY_VALUE) {
            Ok(value) => assert_eq!(value, "/tmp"),
            Err(_) => assert!(false),
        }
    }
}
