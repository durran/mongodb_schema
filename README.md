# schema-rs [![][travis_img]][travis_url]

Analyses JSON documents and produces a schema.

## Installation

## Usage

```rust
extern crate schema;
use schema::Analyser;

let analyser = Analyser::new("{\"name\":\"testing\"}");
let schema = analyser.run();
```

[travis_img]: https://travis-ci.org/durran/schema-rs.svg?branch=master
[travis_url]: https://travis-ci.org/durran/schema-rs
