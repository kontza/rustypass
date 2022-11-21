//! Here be config file operations
use std::io::Error;
use std::path::PathBuf;
mod config_dir;

const APP_NAME: &str = "rustypass";
const CONFIG_NAME: &str = "config.toml";
pub const SCAN_DIRECTORY_VALUE: &str = "scan-directory";

// trait BaseDirInterface {
//     fn get_config(&mut self, value: &str) -> Result<PathBuf, Error>;
// }
//
// struct BaseDir {
//     base_dir: PathBuf,
// }
//
// impl BaseDirInterface for BaseDir {
//     fn get_config(&mut self, value: &str) -> Result<PathBuf, Error> {
//         let mut dir = self.base_dir;
//         dir.push(APP_NAME);
//         dir.push(CONFIG_NAME);
//         Ok(dir)
//     }
// }

#[cfg(test)]
mod tests {
    // use crate::config::config_dir::TestConfigDir;
    //
    // use super::{config_dir::NoneConfigDir, *};
    //
    // #[test]
    // fn it_gets_the_expected_value() {
    //     let n = NoneConfigDir {};
    //     n.get();
    // }
}
