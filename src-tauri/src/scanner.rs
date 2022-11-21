//! Do a recursive scan of a given directory looking for gpg files.
use config::*;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use walkdir::WalkDir;

struct WalkResult {
    /// The path of the found file.
    pub path: String,
    /// - _bool_:
    ///   - `true`: A suitable file was found.
    /// - _io::Error_: Something went wrong during the walk.
    pub result: Result<bool, Error>,
}

/// Initiate scanning.
fn start_scanning() {}

/// Start scanning through the given directory. Returns a channel of
/// WalkResult structs.
fn _start_scanning(first_step: &PathBuf) -> Receiver<WalkResult> {
    let (tx, rx) = mpsc::channel();

    let starter = first_step.clone();
    thread::spawn(move || {
        let walker = WalkDir::new(starter).follow_links(true).into_iter();
        for entry_result in walker {
            if let Ok(entry) = entry_result {
                if entry.file_type().is_file() {
                    if let Some(path) = entry.path().to_str() {
                        let mut walk_result = WalkResult {
                            path: path.to_string(),
                            result: Ok(true),
                        };
                        if let Some(extension) = entry.path().extension() {
                            if extension != "gpg" {
                                walk_result.result = Ok(false);
                            }
                        } else {
                            walk_result.result = Err(Error::new(
                                ErrorKind::Other,
                                "Could not get the extension of the file!",
                            ));
                        }
                        tx.send(walk_result).unwrap();
                    }
                }
            }
        }
    });
    return rx;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn it_finds_the_expected_amount_of_files() {
        let mut resource_dir = PathBuf::new();
        if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            resource_dir.push(manifest_dir);
        }
        resource_dir.push("resources");
        resource_dir.push("test");
        let mut items: Vec<String> = vec![];
        // let rx = start_scanning(&resource_dir);
        let mut invalid_count = 0;
        // for received in rx {
        //     match received.result {
        //         Ok(result) => {
        //             if result == true {
        //                 items.push(received.path);
        //             } else {
        //                 println!("Unknown media type: {}", received.path);
        //                 invalid_count += 1;
        //             }
        //         }
        //         Err(err) => {
        //             println!("{}: {:?}", received.path, err);
        //         }
        //     }
        // }
        assert_eq!(items.len(), 3);
        assert_eq!(invalid_count, 1);
    }
}
