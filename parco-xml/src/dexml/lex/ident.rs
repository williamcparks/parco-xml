pub fn ident(src: &str) -> Option<&str> {
    let ch = src.chars().next()?;
    if !ch.is_ascii_alphabetic() && ch != '_' {
        return None;
    }

    for (idx, ch) in src.char_indices().skip(1) {
        if !ch.is_ascii_alphanumeric() && ch != '_' && ch != '.' && ch != '-' {
            return src.get(..idx);
        }
    }

    Some(src)
}
