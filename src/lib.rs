
mod parser;

#[derive(Debug)]
pub struct Glyphlist {
    names: Vec<(String, usize)>,
    unicode: Vec<(String, usize)>
}

impl Glyphlist {
    pub fn to_name(&self, unicode: &str) -> Option<&str> {
        let pos = self.unicode.binary_search_by_key(&unicode, |&(ref c,_)| c).ok()?;
        let name_pos = self.unicode[pos].1;
        Some(&self.names[name_pos].0)
    }

    pub fn to_unicode(&self, name: &str) -> Option<&str> {
        let pos = self.names.binary_search_by_key(&name, |&(ref n,_)| n).ok()?;
        let unicode_pos = self.names[pos].1;
        Some(&self.unicode[unicode_pos].0)
    }
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

}
