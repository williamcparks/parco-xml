pub fn skip(src: &str) -> usize {
    match ws(src) {
        0 => comment(src),
        ws => ws,
    }
}

pub fn ws(src: &str) -> usize {
    for (idx, ch) in src.char_indices() {
        if !ch.is_whitespace() {
            return idx;
        }
    }
    src.len()
}

pub fn comment(src: &str) -> usize {
    let stripped = match src.strip_prefix("<!--") {
        Some(some) => some,
        None => return 0,
    };

    match stripped.find("-->") {
        Some(end) => 4 + end + 3,
        None => src.len(),
    }
}
