use std::{env, path::PathBuf};

pub trait ConfigDirInterface {
    fn get(&self) -> Option<PathBuf>;
}

#[derive(Debug)]
pub struct ConfigDir;
impl ConfigDirInterface for ConfigDir {
    fn get(&self) -> Option<PathBuf> {
        dirs::config_dir()
    }
}

#[derive(Debug)]
pub struct TestConfigDir;
impl ConfigDirInterface for TestConfigDir {
    fn get(&self) -> Option<PathBuf> {
        let mut resource_dir = PathBuf::new();
        if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            resource_dir.push(manifest_dir);
            Some(resource_dir)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct NoneConfigDir;
impl ConfigDirInterface for NoneConfigDir {
    fn get(&self) -> Option<PathBuf> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{ConfigDir, ConfigDirInterface, NoneConfigDir, TestConfigDir};

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
        assert_eq!(true, u.ends_with("src-tauri"));
    }

    #[test]
    fn config_dir_is_found() {
        let c = ConfigDir;
        let p = c.get();
        assert_ne!(None, p);
        let u = p.unwrap();
        let found =
            u.ends_with("Application Support") || u.ends_with(".config") || u.ends_with("Roaming");
        assert_eq!(true, found);
    }
}
