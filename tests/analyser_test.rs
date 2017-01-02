#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

extern crate schema;
extern crate serde_json;

use schema::Analyser;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

describe! analyser_test {

    before_each {
        let cwd = env::current_dir().unwrap();
    }

    describe! new {

        describe! with_basic_docs {

            before_each {
                let mut s = String::new();
                let mut path = PathBuf::from(&cwd);
                path.push("tests/fixtures/basic.json");
                let mut f = File::open(path.as_os_str()).unwrap();
                f.read_to_string(&mut s).unwrap();
                let analyser = Analyser::new(&s);
                let docs = analyser.documents.as_array().unwrap();
                let doc = docs[0].as_object().unwrap();
            }

            it "deserializes string elements" {
                assert_eq!(doc.get("name").unwrap().as_str().unwrap(), "Depeche Mode");
            }

            it "deserializes integer elements" {
                assert_eq!(doc.get("albums").unwrap().as_i64().unwrap(), 20);
            }

            it "deserializes float elements" {
                assert_eq!(doc.get("rating").unwrap().as_f64().unwrap(), 10.5);
            }

            it "deserializes bool elements" {
                assert_eq!(doc.get("active").unwrap().as_bool().unwrap(), true);
            }
        }
    }
}
