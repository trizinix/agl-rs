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

fn main() {
    let f = File::open("agl-aglfn/glyphlist.txt").unwrap();
    let mut b = BufReader::new(f);
    let glyphlist = parse_glyphlist(&mut b).unwrap();


    // Output file
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("adobe.data");
    let mut f = File::create(&dest_path).unwrap();

    f.write_all(&serialize(&glyphlist, Infinite).unwrap()).unwrap();
}


/*
#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Entity {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct World(Vec<Entity>);

fn main() {
    let world = World(vec![Entity { x: 0.0, y: 4.0 }, Entity { x: 10.0, y: 20.5 }]);

    let encoded: Vec<u8> = serialize(&world, Infinite).unwrap();

    // 8 bytes for the length of the vector, 4 bytes per float.
    assert_eq!(encoded.len(), 8 + 4 * 4);

    let decoded: World = deserialize(&encoded[..]).unwrap();

    assert_eq!(world, decoded);
}*/