use std::thread;
use std::path::Path;
use std::time::{Duration, Instant};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn file_matches(path: &Path, expected: &str) -> bool {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    contents == expected
}

fn wait_for_file_exists(path: &Path) {
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

#[test]
fn integration_test() {
    ::env_logger::init().unwrap();

    thread::spawn(|| ::run("../data"));

    let root = Path::new("../data");
    let root_sha1_path = root.join(".sha1");
    let dummy_file_path = root.join("O/OK/J/Johnston_County/38920.json");

    // Wait for root sha1 to be created
    info!(
        "Waiting for root sha1 to be created - path: {:?}",
        root_sha1_path
    );
    wait_for_file_exists(&root_sha1_path);
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

    ::std::fs::remove_file(&dummy_file_path).unwrap();
}
