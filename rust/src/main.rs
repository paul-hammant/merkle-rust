extern crate sha1;
extern crate time;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

fn comma_separated_list(dir_sha1s: &Vec<String>) -> String {
    let mut dir_sha1s = dir_sha1s.clone();
    dir_sha1s.sort();
    dir_sha1s.as_slice().join(",")
}

fn get_contents(path: &Path) -> String {
    if !path.exists() {
        return "".to_string()
    }
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    contents
}

fn write_contents(path: &Path, contents: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

fn make_sha1(input: &str) -> String {
    let mut sha1 = sha1::Sha1::new();
    sha1.update(input.as_bytes());
    sha1.digest().to_string()
}

fn process_directory(dir: &Path) -> (String, u32) {
    let mut dir_sha1s = Vec::new();
    let mut count = 0;

    for entry in dir.read_dir().unwrap() {
        let path = entry.unwrap().path();

        if path.is_dir() {
            let (new_sha1, new_count) = process_directory(&path);
            dir_sha1s.push(new_sha1.to_string());
            count += new_count;
        } else if let Some(extension) = path.extension() {
            if extension == "json" {
                let sha1_file = format!("{}.sha1", path.to_string_lossy());
                let sha1 = make_sha1(&get_contents(&path));
                if sha1 != get_contents(Path::new(&sha1_file)) {
                    count += 1;
                    write_contents(Path::new(&sha1_file), &sha1);
                }
                dir_sha1s.push(sha1);
            }
        }
    }

    let sha1 = make_sha1(&comma_separated_list(&dir_sha1s));
    let sha1_file = dir.join(".sha1");
    if sha1 != get_contents(&sha1_file) {
        count += 1;
        write_contents(&sha1_file, &sha1);
    }

    (sha1, count)
}

fn main() {

    if let Some(path) = std::env::args().nth(1) {
        loop {
            let start = time::now();
            let (sha1, count) = process_directory(Path::new(&path));
            println!("Updates: {}, root SHA1: {}, duration: {}s", count, sha1, (time::now() - start).num_seconds());
        }
    } else {
        println!("Specify data directory - `cargo run -- <directory>`");
    }
}
