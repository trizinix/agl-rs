use std::char;
use std::io::BufRead;

use ::Glyphlist;

pub fn parse_glyphlist(reader: &mut BufRead) -> Option<Glyphlist> {
    let mut mapping: Vec<(String, String)> = Vec::new();
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
        mapping.push((String::from(name), unicode_str));

    }

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

    Some(Glyphlist { names, unicode: codepoints })
}