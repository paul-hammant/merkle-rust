use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn dir_sha1(dir: &Path) -> String {
    let mut sha1s = Vec::new();

    for entry in dir.read_dir().unwrap() {
        let path = entry.unwrap().path();

        if path.is_dir() {
            sha1s.push(::get_contents(&path.join(".sha1")));
        } else if let Some(extension) = path.extension() {
            if extension == "json.sha1" {
                sha1s.push(::get_contents(&path));
            }
        }
    }

    ::make_sha1(&::comma_separated_list(&sha1s))
}

fn is_subdir(path: &Path, root: &Path) -> bool {
    let mut path = path.clone();

    while let Some(parent) = path.parent() {
        if parent == root {
            return true;
        }
        path = parent;
    }

    false
}

fn process_leaf(path: &Path, root: &Path) -> String {
    assert!(
        is_subdir(path, root) || path == root,
        "Leaf is not within root directory"
    );

    let sha1;
    let sha1_file;
    if path.is_dir() {
        sha1 = dir_sha1(path);
        sha1_file = path.join(".sha1");
    } else {
        sha1 = ::make_sha1(&::get_contents(path));
        sha1_file = path.with_extension(format!(
            "{}.sha1",
            path.extension().unwrap().to_str().unwrap()
        ));
    }
    if sha1 != ::get_contents(&sha1_file) {
        ::write_contents(&sha1_file, &sha1);
    }

    if path != root {
        process_leaf(path.parent().unwrap(), root);
    }

    sha1
}

pub fn work(root: PathBuf, jobs: Arc<Mutex<Vec<PathBuf>>>) {
    let start = Instant::now();

    loop {
        // Introduce a new scope so the mutex gets dropped and other threads can pick it up
        {
            let mut guard = match jobs.lock() {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner(),
            };
            if !guard.is_empty() {
                let job = guard.pop().unwrap();

                println!("[work] Got job for {}", job.to_string_lossy());

                let sha1 = process_leaf(&job, &root);
                println!(
                    "[work] Job finished - new root sha1: {}, duration: {}s",
                    sha1,
                    start.elapsed().as_secs()
                );
            }
        }

        thread::sleep(Duration::from_millis(50));
    }
}
