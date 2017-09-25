// Copyright 2017, Paul Hammant
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate env_logger;
#[macro_use]
extern crate log;
extern crate notify;
extern crate sha1;

mod worker;

#[cfg(test)]
mod test;

use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use std::ffi::OsStr;

pub fn comma_separated_list(dir_sha1s: &Vec<String>) -> String {
    let mut dir_sha1s = dir_sha1s.clone();
    dir_sha1s.sort();
    dir_sha1s.as_slice().join(",")
}

pub fn get_contents(path: &Path) -> String {
    if !path.exists() {
        return "".to_string();
    }
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    contents
}

pub fn write_contents(path: &Path, contents: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

pub fn make_sha1(input: &str) -> String {
    let mut sha1 = sha1::Sha1::new();
    sha1.update(input.as_bytes());
    sha1.digest().to_string()
}

fn process_directory(dir: &Path) -> String {
    let mut dir_sha1s = Vec::new();

    for entry in dir.read_dir().unwrap() {
        let path = entry.unwrap().path();

        if path.is_dir() {
            dir_sha1s.push(process_directory(&path));
        } else if let Some("json") = path.extension().and_then(OsStr::to_str) {
            let sha1_file = format!("{}.sha1", path.to_string_lossy());
            let sha1 = make_sha1(&get_contents(&path));
            if sha1 != get_contents(Path::new(&sha1_file)) {
                write_contents(Path::new(&sha1_file), &sha1);
            }
            dir_sha1s.push(sha1);
        }
    }

    let sha1 = make_sha1(&comma_separated_list(&dir_sha1s));
    let sha1_file = dir.join(".sha1");
    if sha1 != get_contents(&sha1_file) {
        write_contents(&sha1_file, &sha1);
    }

    sha1
}

fn run(path: &str) {
    let start = Instant::now();
    info!("Starting initial sha1 generation");
    let sha1 = process_directory(Path::new(path));
    info!(
        "Initial generation finished - root sha1: {}, duration: {}s",
        sha1,
        start.elapsed().as_secs()
    );

    let jobs = Arc::new(Mutex::new(Vec::new()));

    let path = Path::new(&path).to_path_buf().canonicalize().unwrap();
    let thread_path = path.clone();
    let thread_jobs = jobs.clone();
    thread::spawn(|| worker::work(thread_path, thread_jobs));

    let (tx, rx) = channel();
    let mut watcher = raw_watcher(tx).unwrap();
    watcher.watch(&path, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(RawEvent {
                path: Some(path),
                op: Ok(op),
                ..
            }) => {
                debug!("FS event - path: {:?}, op: {:?}", path, op);
                if op & ::notify::op::WRITE == ::notify::op::WRITE {
                    if let Some(extension) = path.clone().extension() {
                        if extension == "json" {
                            info!("Queueing job for {:?}", path);
                            let mut guard = match jobs.lock() {
                                Ok(guard) => guard,
                                Err(poisoned) => poisoned.into_inner(),
                            };
                            (*guard).push(path);
                        }
                    }
                }
            }
            Ok(event) => error!("Broken event: {:?}", event),
            Err(e) => error!("Watch error: {:?}", e),
        }
    }
}

fn main() {
    env_logger::init().unwrap();

    if let Some(path) = std::env::args().nth(1) {
        run(&path);
    } else {
        println!("Specify data directory - `cargo run -- <directory>`");
        std::process::exit(1);
    }
}
