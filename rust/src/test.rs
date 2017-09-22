use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};

fn file_matches(path: &Path, expected: &str) -> bool {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    contents == expected
}

fn wait_for_file_to_exist(path: &Path) {
    let start = Instant::now();
    while !path.exists() {
        thread::sleep(Duration::from_millis(500));
        assert!(
            start.elapsed() < Duration::from_secs(60),
            "Timed out while waiting for file {:?} to be created",
            path
        );
    }
}

fn wait_for_file_matches(path: &Path, expected: &str) {
    let start = Instant::now();
    while !file_matches(path, expected) {
        thread::sleep(Duration::from_millis(500));
        assert!(
            start.elapsed() < Duration::from_secs(60),
            "Timed out while waiting for file {:?} to match \"{}\"",
            path,
            expected,
        );
    }
}

fn clean_dir(path: &Path) {
    for entry in path.read_dir().unwrap() {
        let path = entry.unwrap().path();

        if path.is_dir() {
            clean_dir(&path);
        } else if Some("json.sha1") == path.extension().and_then(OsStr::to_str) {
            fs::remove_file(path).unwrap();
        }
    }
}

#[test]
fn integration_test() {
    ::env_logger::init().unwrap();

    let root = Path::new("../data");
    let root_sha1_path = root.join(".sha1");
    let dummy_file_path = root.join("O/OK/J/Johnston_County/38920.json");

    if dummy_file_path.exists() {
        fs::remove_file(&dummy_file_path).unwrap();
    }
    clean_dir(&root);

    thread::spawn(|| ::run("../data"));

    // Wait for root sha1 to be created
    info!(
        "Waiting for root sha1 to be created - path: {:?}",
        root_sha1_path
    );
    wait_for_file_to_exist(&root_sha1_path);
    wait_for_file_matches(&root_sha1_path, "33cf709e348a0bf57686ddc60398f755e9783517");
    info!("First test passed - root sha1 matches");

    // Write new json file - this should change the hash
    info!("Creating new dummy .json file - root sha1 should change");
    File::create(&dummy_file_path)
        .unwrap()
        .write_all("integration test".as_bytes())
        .unwrap();

    // Wait for root sha1 to change
    wait_for_file_matches(&root_sha1_path, "e600c58d06aee522595acb019e71487db53eb487");
    info!("Second test passed - root sha1 matches");
}
