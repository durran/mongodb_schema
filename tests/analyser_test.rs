#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

extern crate schema;
use schema::Analyser;
use std::io::prelude::*;
use std::env;
use std::fs::File;
use std::path::PathBuf;

describe! field_test {

    describe! standard_json {

        before_each {
            let cwd = env::current_dir().unwrap();
            let mut path = PathBuf::from(&cwd);
            let mut s = String::new();
            path.push("tests");
            path.push("fixtures");
            path.push("standard.json");
            let mut f = File::open(path.as_os_str()).unwrap();
            f.read_to_string(&mut s).unwrap();
        }

        it "deserializes JSON into documents" {
            let analyser = Analyser::new(&s);
            let json = analyser.documents.as_array().unwrap();
        }
    }
}
