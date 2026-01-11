pub fn xmlstr(src: &str) -> Option<&str> {
    let mut chars = src.chars();
    let quote = chars.next()?;

    if quote != '"' && quote != '\'' {
        return None;
    }

    let rest = src.get(1..)?;
    let end_pos = rest.find(quote)? + 2;

    src.get(..end_pos)
}
