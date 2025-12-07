pub fn parse_amount(raw: &str) -> Option<i64> {
    let cleaned: String = raw
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '.')
        .collect();

    if cleaned.is_empty() {
        return None;
    }

    let parts: Vec<&str> = cleaned.split('.').collect();

    match parts.len() {
        1 => parts[0].parse::<i64>().ok().map(|v| v * 100),
        2 => {
            let whole = parts[0].parse::<i64>().ok()?;
            let frac_str = format!("{:0<2}", parts[1]);
            let frac: i64 = frac_str[0..2].parse().ok()?;
            Some(whole * 100 + frac)
        }
        _ => None,
    }
}
