//! Here be config file operations
pub mod config_dir;
use crate::config::config_dir::ConfigDirInterface;
use config::{Config, File, FileFormat};

pub const SCAN_DIRECTORY_VALUE: &str = "scan-directory";
pub const SHORTCUT_VALUE: &str = "shortcut";

pub trait ConfigStoreInterface {
    fn get_config(&mut self, value: &str) -> Option<String>;
}

#[derive(Debug)]
pub struct ConfigStore {
    pub config_dir: Box<dyn ConfigDirInterface>,
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
    use crate::config::{ConfigStoreInterface, SCAN_DIRECTORY_VALUE, SHORTCUT_VALUE};

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
        match bd.get_config(SHORTCUT_VALUE) {
            Some(value) => assert_eq!(value, "global-shortcut"),
            None => assert!(false),
        }
    }
}
