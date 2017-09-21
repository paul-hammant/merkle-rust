use std::thread;
use std::path::Path;
use std::time::Duration;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn root_sha1_matches() -> bool {
    let file = File::open("../data/.sha1").expect("../data/.sha1 does not exist");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();

    contents == "33cf709e348a0bf57686ddc60398f755e9783517"
}

#[test]
fn integration_test() {
    thread::spawn(|| ::run("../data"));

    thread::sleep(Duration::new(3, 0));

    // Hash should match without intervention
    assert!(root_sha1_matches() == true);

    thread::sleep(Duration::new(3, 0));

    // Write new json file - this should change the hash
    File::create(Path::new("../data/O/OK/J/Johnston_County/38920.json"))
        .unwrap()
        .write_all("integration test".as_bytes())
        .unwrap();

    thread::sleep(Duration::new(3, 0));

    // Hash should not match anymore
    assert!(root_sha1_matches() == false);
}
