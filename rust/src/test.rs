use ::*;

fn root_sha1_matches() -> bool {
    let (sha1, _) = process_directory(Path::new("../data"));
    sha1 == "33cf709e348a0bf57686ddc60398f755e9783517"
}

#[test]
fn integration_test() {
    // Hash should match without intervention
    assert!(root_sha1_matches() == true);
    // Write new json file - this should change the hash
    write_contents(Path::new("../data/test.json"), "integration test");
    // Hash should not match anymore
    assert!(root_sha1_matches() == false);
    // Clean up after ourselves
    std::fs::remove_file("../data/test.json").unwrap();
}
