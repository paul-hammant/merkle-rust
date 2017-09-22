use std::thread;
use std::path::Path;
use std::time::{Duration, Instant};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn root_sha1_matches() -> Option<bool> {
    let path = Path::new("../data/.sha1");
    if path.exists() {
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents).unwrap();

        Some(contents == "33cf709e348a0bf57686ddc60398f755e9783517")
    } else {
        None
    }
}

#[test]
fn integration_test() {
    thread::spawn(|| ::run("../data"));

    let start = Instant::now();
    loop {
        // Hash should match without intervention
        match root_sha1_matches() {
            Some(true) => break,
            Some(false) => {
                panic!(
                    "First test failed - root sha1 does not match. Did you clean the data directory?"
                );
            }
            _ => (),
        }
        thread::sleep(Duration::from_secs(1));
        // The test will fail after 60 seconds
        assert!(Instant::now() - start < Duration::from_secs(60));
    }
    println!("First test passed - root sha1 matches");

    // Write new json file - this should change the hash
    println!("Creating new .json file - root sha1 should change");
    File::create(Path::new("../data/O/OK/J/Johnston_County/38920.json"))
        .unwrap()
        .write_all("integration test".as_bytes())
        .unwrap();

    let start = Instant::now();
    loop {
        // Hash should match without intervention
        if root_sha1_matches() == Some(false) {
            break;
        }
        thread::sleep(Duration::from_secs(1));
        // The test will fail after 60 seconds
        assert!(Instant::now() - start < Duration::from_secs(60));
    }
    println!("Second test passed - root sha1 does not match");
}
