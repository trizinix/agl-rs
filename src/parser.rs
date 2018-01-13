use std::char;
use std::io::BufRead;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct StaticGlyphList {
    names: Vec<(String, usize)>,
    unicode: Vec<(String, usize)>
}

impl StaticGlyphList {
    pub fn lookup_name(&self, unicode: &str) -> Option<&str> {
        let pos = self.unicode.binary_search_by_key(&unicode, |&(ref c,_)| c).ok()?;
        let name_pos = self.unicode[pos].1;
        Some(&self.names[name_pos].0)
    }

    pub fn lookup_unicode(&self, name: &str) -> Option<&str> {
        let pos = self.names.binary_search_by_key(&name, |&(ref n,_)| n).ok()?;
        let unicode_pos = self.names[pos].1;
        Some(&self.unicode[unicode_pos].0)
    }
}

pub struct GlyphListBuilder {
    pairs: Vec<(String, String)>
}

impl GlyphListBuilder {
    pub fn new() -> GlyphListBuilder {
        GlyphListBuilder { pairs: Vec::new() }
    }

    pub fn add(&mut self, name: String, unicode: String) {
        self.pairs.push((name, unicode));
    }

    pub fn finalize(self) -> Option<StaticGlyphList> {
        let mut mapping = self.pairs;

        let mut names: Vec<(String, usize)>;
        let mut codepoints: Vec<(String, usize)> = Vec::with_capacity(mapping.len());

        // Sort by names
        mapping.sort_unstable_by_key(|&(ref n, _)| n.clone());
        names = mapping.iter().map(|&(ref n, _)| (n.clone(), 0)).collect();

        // Sort by codepoints
        mapping.sort_unstable_by_key(|&(_, ref c)| c.clone());
        for (i, &(ref name, ref codepoint)) in mapping.iter().enumerate() {
            let pos = names.binary_search_by_key(&name, |&(ref n,_)| n).ok()?;
            codepoints.push((codepoint.to_string(), pos));
            names[pos].1 = i;
        }

        Some(StaticGlyphList { names, unicode: codepoints })
    }
}

pub fn parse_glyph_list(reader: &mut BufRead) -> Option<StaticGlyphList> {
    let mut builder = GlyphListBuilder::new();

    for line_r in reader.lines() {
        let line = line_r.ok()?;
        if line.starts_with('#') { continue; }

        let pos = line.find(';')?;
        let (name, code_points) = line.split_at(pos);
        let unicode_str: String = code_points.split_whitespace()
            .filter_map(|s| {
                u32::from_str_radix(&s[1..], 16).ok().and_then(|u| char::from_u32(u))
            }).collect();

        // TODO handle dupplicates, multiple names can point to the same unicode chars.
        // When that happens we should use the first one unless the name is part of
        // a standard encoding
        builder.add(String::from(name), unicode_str);
    }

    builder.finalize()
}