#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
extern crate bincode;

use std::char;

use bincode::deserialize;

mod parser;
use parser::StaticGlyphList;

lazy_static! {
    static ref ADOBE: StaticGlyphList = {
        let ser = include_bytes!(concat!(env!("OUT_DIR"), "/adobe.data"));
        let list: StaticGlyphList = deserialize(&ser[..]).unwrap();
        list
    };

    static ref ZAPF_DINGBATS: StaticGlyphList = {
        let ser = include_bytes!(concat!(env!("OUT_DIR"), "/zapfdingbats.data"));
        let list: StaticGlyphList = deserialize(&ser[..]).unwrap();
        list
    };

    static ref ADDITIONAL: StaticGlyphList = {
        let ser = include_bytes!(concat!(env!("OUT_DIR"), "/additional.data"));
        let list: StaticGlyphList = deserialize(&ser[..]).unwrap();
        list
    };
}

pub enum GlyphList {
    Static(&'static StaticGlyphList),
    Combined(Box<GlyphList>, Box<GlyphList>)
}

impl GlyphList {
    fn lookup_name(&self, unicode: &str) -> Option<&str> {
        match *self {
            GlyphList::Static(ref gl) => gl.lookup_name(unicode),
            GlyphList::Combined(ref gl1, ref gl2) => {
                let name1 = gl1.lookup_name(unicode);
                if name1.is_some() { return name1; }
                return gl2.lookup_name(unicode)
            }
        }
    }
    fn lookup_unicode(&self, name: &str) -> Option<&str> {
        match *self {
            GlyphList::Static(ref gl) => gl.lookup_unicode(name),
            GlyphList::Combined(ref gl1, ref gl2) => {
                let unicode1 = gl1.lookup_unicode(name);
                if unicode1.is_some() { return unicode1; }
                return gl2.lookup_unicode(name)
            }
        }
    }

    pub fn adobe() -> GlyphList {
        GlyphList::Static(&ADOBE)
    }

    pub fn zapf_dingbats() -> GlyphList {
        // TODO add space
        GlyphList::Static(&ZAPF_DINGBATS)
    }

    pub fn pdf_extended() -> GlyphList {
        GlyphList::Combined(Box::new(GlyphList::Static(&ADDITIONAL)), Box::new(GlyphList::adobe()))
    }

    pub fn name_from_unicode_str(&self, unicode: &str) -> &str {
        if let Some(name) = self.lookup_name(unicode) {
            name
        } else {
            ".notdef"
        }
    }


    pub fn unicode_from_name(&self, name: &str) -> Option<String> {
        if let Some(unicode) = self.lookup_unicode(name) {
            return Some(unicode.to_owned());
        }

        if let Some(pos) = name.find('.') {
            return self.unicode_from_name(&name[(pos+1)..]).to_owned();
        }

        if name.starts_with("uni") {
            let s: Vec<char> = (&name[3..]).chars().collect();
            let unicode: String = s.chunks(4).filter_map(|v| {
                let s: String = v.iter().collect();
                u32::from_str_radix(&s, 16).ok().and_then(|u| {char::from_u32(u)})
            }).collect();
            return Some(unicode);
        }

        if name.starts_with('u') {
            let u = u32::from_str_radix(&name[1..5], 16).ok();
            return u.and_then(|u| char::from_u32(u).map(|c| c.to_string()))
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
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
        let glyphlist = ::parser::parse_glyph_list(&mut buff).unwrap();

        assert_eq!(glyphlist.lookup_unicode("ibengali"), Some("ই"));
        assert_eq!(glyphlist.lookup_unicode("hyphen"), Some("-"));
        assert_eq!(glyphlist.lookup_unicode("huhiragana"), Some("ふ"));
    }

    #[test]
    fn parse_adobe_glyphlist() {
        let f = File::open("agl-aglfn/glyphlist.txt").unwrap();
        let mut b = BufReader::new(f);
        let glyphlist = ::parser::parse_glyph_list(&mut b).unwrap();

        assert_eq!(glyphlist.lookup_unicode("ibengali"), Some("ই"));
        assert_eq!(glyphlist.lookup_unicode("hyphen"), Some("-"));
        assert_eq!(glyphlist.lookup_unicode("huhiragana"), Some("ふ"));
    }

    use super::ADOBE;
    #[test]
    fn test_build_rs_adobe() {
        assert_eq!(ADOBE.lookup_unicode("ibengali"), Some("ই"));
        assert_eq!(ADOBE.lookup_unicode("hyphen"), Some("-"));
        assert_eq!(ADOBE.lookup_unicode("huhiragana"), Some("ふ"));
    }

}
