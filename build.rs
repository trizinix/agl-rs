use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
//use std::char;
use std::io::BufReader;

#[macro_use]
extern crate serde_derive;
extern crate bincode;

use bincode::{serialize, Infinite};

include!("src/parser.rs");


fn generate_serialized_data(input_path: &str, output_path: &str) {
    let f = File::open(input_path).unwrap();
    let mut b = BufReader::new(f);
    let glyphlist = parse_glyph_list(&mut b).unwrap();

    // Output file
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(output_path);
    let mut f = File::create(&dest_path).unwrap();

    f.write_all(&serialize(&glyphlist, Infinite).unwrap()).unwrap();
}

fn main() {
    generate_serialized_data("agl-aglfn/glyphlist.txt", "adobe.data");
    generate_serialized_data("agl-aglfn/zapfdingbats.txt", "zapfdingbats.data");
    generate_serialized_data("assets/additional.txt", "additional.data");
}