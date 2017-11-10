// Copyright 2017, Nick Renieris
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

use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::prelude::*;

use futures;
use futures::future::Future;

use hyper;
use hyper::header::*;
use hyper::server::{Http, Request, Response, Service};
use hyper::{Method, StatusCode};

use glob::glob;

use std::sync::Mutex;

struct MerkleTreeExplorer {
    // stores the path to the data directory
    data_path: String,
}

impl Service for MerkleTreeExplorer {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;

    // The future representing the eventual Response our call will resolve to
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new().with_header(ContentType::plaintext());
        let mut contents = String::new();

        // read directory SHA
        let path_str = self.data_path.clone() + req.path();
        let path = Path::new(&path_str);

        if path.is_dir() {
            // read root SHA
            read_sha_path(&path, &mut contents, false, false);

            // read children SHAs
            let glob_path = path.join("*");

            for entry in glob(&glob_path.to_str().unwrap()).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => {
                        if path.is_dir() {
                            // Directory
                            // Print the directory name and the hash
                            read_sha_path(&path, &mut contents, true, false);
                        } else {
                            // File
                            // Skip .sha1 files
                            if path.to_str().unwrap().ends_with(".sha1") {
                                continue;
                            };
                            // Print the file name and the hash
                            read_sha_path(&path, &mut contents, true, true);
                        }
                    }
                    Err(e) => println!("{:?}", e),
                }
            }
        } else {
            // just serve the static file
            match File::open(&path) {
                Ok(file) => {
                    (&file)
                        .read_to_string(&mut contents)
                        .expect("something went wrong reading the file");
                }
                Err(why) => {
                    error!("couldn't read file {:?}, error {:?}", path, why);
                    contents.push_str("not there");
                }
            }
        }

        if req.method() == &Method::Get {
            response.set_body(contents);
        } else {
            response.set_status(StatusCode::NotFound);
        }

        Box::new(futures::future::ok(response))
    }
}

fn read_sha_path(path: &Path, contents: &mut String, incl_filename: bool, is_file: bool) {
    let sha1_path: PathBuf = if is_file {
        // if it's a file we need to just add ".sha" at the end
        let mut pathbuf = PathBuf::new();
        pathbuf.push(path.to_str().unwrap().to_string() + ".sha1");
        pathbuf
    } else {
        // if it's a directory we need to also add the / ("/.sha")
        path.join(".sha1")
    };

    debug!("reading file: {:?}", &sha1_path);

    match File::open(&sha1_path) {
        Ok(file) => {
            if incl_filename {
                contents.push_str(path.file_name().unwrap().to_str().unwrap());
                contents.push(' ');
            }

            (&file)
                .read_to_string(contents)
                .expect("something went wrong reading the file");

            contents.push('\n');
        }
        Err(why) => error!("couldn't read file {:?}, error {:?}", sha1_path, why),
    }
}

pub fn run_server(path: String, port: u16) {
    let addr = format!("127.0.0.1:{}", port).parse().unwrap();

    // Create and run the HTTP server
    let server = Http::new()
        .bind(&addr, move || {
            Ok(// Create the MerkleTreeExplorer object and update its data_path field
               MerkleTreeExplorer { data_path: path.to_owned() })
        })
        .unwrap();

    server.run().unwrap();
}
