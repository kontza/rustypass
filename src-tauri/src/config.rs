//! Here be config file operations
use config::{Config, File, FileFormat};

use self::config_dir::ConfigDirInterface;
mod config_dir;

pub const SCAN_DIRECTORY_VALUE: &str = "scan-directory";

trait ConfigStoreInterface {
    fn get_config(&mut self, value: &str) -> Option<String>;
}

#[derive(Debug)]
struct ConfigStore {
    config_dir: Box<dyn ConfigDirInterface>,
}
impl ConfigStoreInterface for ConfigStore {
    fn get_config(&mut self, value: &str) -> Option<String> {
        let binding = self.config_dir.get()?;
        let d = binding.to_str()?;
        let mut data_dir = dirs::data_dir()?;
        data_dir.push("gopass");
        data_dir.push("stores");
        let builder = Config::builder()
            .set_default(SCAN_DIRECTORY_VALUE, data_dir.to_str())
            .ok()?
            .add_source(File::new(d, FileFormat::Toml));
        let config = builder.build().ok()?;
        config.get_string(value).ok()
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{ConfigStoreInterface, SCAN_DIRECTORY_VALUE};

    use super::config_dir::TestConfigDir;
    use super::ConfigStore;

    #[test]
    fn config_gets_a_string() {
        let mut bd = ConfigStore {
            config_dir: Box::new(TestConfigDir),
        };
        match bd.get_config(SCAN_DIRECTORY_VALUE) {
            Some(value) => assert_eq!(value, "/tmp"),
            None => assert!(false),
        }
    }
}
