#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
extern crate bincode;

use bincode::deserialize;

mod parser;
use parser::Glyphlist;

lazy_static! {
    static ref ADOBE: Glyphlist = {
        let ser = include_bytes!(concat!(env!("OUT_DIR"), "/adobe.data"));
        let list: Glyphlist = deserialize(&ser[..]).unwrap();
        list
    };
}


#[cfg(test)]
mod tests {
    use std::io::{self, BufReader};
    use std::io::prelude::*;
    use std::fs::File;
    use std::io::Cursor;

    #[test]
    fn parse_simple_glyphlist() {
        let input = br#"# A comment
ibengali;0987
hyphen;002D
huhiragana;3075
"#;
        let v = input.to_owned();
        let mut buff = Cursor::new(v.as_ref());
        let glyphlist = ::parser::parse_glyphlist(&mut buff).unwrap();

        assert_eq!(glyphlist.to_unicode("ibengali"), Some("ই"));
        assert_eq!(glyphlist.to_unicode("hyphen"), Some("-"));
        assert_eq!(glyphlist.to_unicode("huhiragana"), Some("ふ"));
    }

    #[test]
    fn parse_adobe_glyphlist() {
        let f = File::open("agl-aglfn/glyphlist.txt").unwrap();
        let mut b = BufReader::new(f);
        let glyphlist = ::parser::parse_glyphlist(&mut b).unwrap();

        assert_eq!(glyphlist.to_unicode("ibengali"), Some("ই"));
        assert_eq!(glyphlist.to_unicode("hyphen"), Some("-"));
        assert_eq!(glyphlist.to_unicode("huhiragana"), Some("ふ"));
    }

    use super::ADOBE;
    #[test]
    fn test_build_rs_adobe() {
        assert_eq!(ADOBE.to_unicode("ibengali"), Some("ই"));
        assert_eq!(ADOBE.to_unicode("hyphen"), Some("-"));
        assert_eq!(ADOBE.to_unicode("huhiragana"), Some("ふ"));
    }

}
