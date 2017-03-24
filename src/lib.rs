use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate handlebars;

mod spec;
mod templates;
mod render;

use spec::ShSpec;

pub fn generate<P: AsRef<Path>, Q: AsRef<Path>>(in_path: P, out_path: Q) {
    let mut file = File::open(in_path).expect("Failed to open input file");
    let mut string = String::new();
    file.read_to_string(&mut string).expect("Failed to read input file");

    let sh_spec: ShSpec = toml::from_str(string.as_ref())
        .expect("Failed to parse input file");

    let output_string = render::render(&sh_spec);

    let mut outfile = File::create(out_path).expect("Failed to create output file");
    write!(outfile, "{}", output_string).expect("Failed to write output file");
}
