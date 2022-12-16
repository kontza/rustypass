use core::fmt::Debug;
use std::{env, path::PathBuf};

pub const CONFIG_NAME: &str = "config.toml";

#[derive(Debug)]
pub enum ConfigDirKind {
    Real,
    Test,
    Nil,
}
pub trait ConfigDirInterface {
    fn get(&self) -> Option<PathBuf>;
    fn get_kind(&self) -> ConfigDirKind;
}

impl Debug for dyn ConfigDirInterface {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ConfigDirInterface {:?}", self.get_kind())
    }
}

#[derive(Debug)]
pub struct ConfigDir;
impl ConfigDirInterface for ConfigDir {
    fn get(&self) -> Option<PathBuf> {
        let mut b = dirs::config_dir()?;
        b.push(env::var("CARGO_PKG_NAME").unwrap());
        b.push(CONFIG_NAME);
        Some(b)
    }

    fn get_kind(&self) -> ConfigDirKind {
        ConfigDirKind::Real
    }
}

#[derive(Debug)]
pub struct TestConfigDir;
impl ConfigDirInterface for TestConfigDir {
    fn get(&self) -> Option<PathBuf> {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").ok()?;
        let mut b = PathBuf::from(manifest_dir);
        b.push("resources");
        b.push("test");
        b.push(CONFIG_NAME);
        Some(b)
    }

    fn get_kind(&self) -> ConfigDirKind {
        ConfigDirKind::Test
    }
}

#[derive(Debug)]
pub struct NoneConfigDir;
impl ConfigDirInterface for NoneConfigDir {
    fn get(&self) -> Option<PathBuf> {
        None
    }

    fn get_kind(&self) -> ConfigDirKind {
        ConfigDirKind::Nil
    }
}

#[cfg(test)]
mod tests {
    use std::{env, path::PathBuf};

    use super::{ConfigDir, ConfigDirInterface, NoneConfigDir, TestConfigDir, CONFIG_NAME};

    #[test]
    fn none_gets_none() {
        let n = NoneConfigDir;
        assert_eq!(None, n.get());
    }

    #[test]
    fn test_gets_test() {
        let t = TestConfigDir;
        let p = t.get();
        assert_ne!(None, p);
        let u = p.unwrap();
        assert_eq!(true, u.ends_with("src-tauri/resources/test/config.toml"));
    }

    #[test]
    fn config_dir_is_found() {
        let c = ConfigDir;
        let p = c.get();
        assert_ne!(None, p);
        let u = p.unwrap();
        let package_name = env::var("CARGO_PKG_NAME").unwrap();
        let mac: PathBuf = ["Application Support", &package_name, CONFIG_NAME]
            .iter()
            .collect();
        let linux: PathBuf = [".config", &package_name, CONFIG_NAME].iter().collect();
        let windows: PathBuf = ["Roaming", &package_name, CONFIG_NAME].iter().collect();
        let found = u.ends_with(mac) || u.ends_with(linux) || u.ends_with(windows);
        assert_eq!(true, found);
    }
}
