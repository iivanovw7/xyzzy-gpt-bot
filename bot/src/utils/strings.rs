pub fn parse_amount(raw: &str) -> Option<i64> {
    let trimmed = raw.trim();
    let is_negative = trimmed.starts_with('-');
    let start_idx = if is_negative { 1 } else { 0 };

    let cleaned: String = trimmed[start_idx..]
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '.')
        .collect();

    if cleaned.is_empty() {
        return None;
    }

    let parts: Vec<&str> = cleaned.split('.').collect();

    let abs_val = match parts.len() {
        1 => parts[0].parse::<i64>().ok().map(|v| v * 100),
        2 => {
            let whole = if parts[0].is_empty() {
                0
            } else {
                parts[0].parse::<i64>().ok()?
            };
            let frac_str = format!("{:0<2}", parts[1]);
            let frac: i64 = frac_str[0..2].parse().ok()?;
            Some(whole * 100 + frac)
        }
        _ => None,
    };

    if is_negative {
        abs_val.map(|v| -v)
    } else {
        abs_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_amount_positive() {
        assert_eq!(parse_amount("10"), Some(1000));
        assert_eq!(parse_amount("10.5"), Some(1050));
        assert_eq!(parse_amount("10.50"), Some(1050));
        assert_eq!(parse_amount("0.99"), Some(99));
        assert_eq!(parse_amount(".99"), Some(99));
        assert_eq!(parse_amount("123.456"), Some(12345));
    }

    #[test]
    fn test_parse_amount_negative() {
        assert_eq!(parse_amount("-10"), Some(-1000));
        assert_eq!(parse_amount("-10.5"), Some(-1050));
        assert_eq!(parse_amount("-0.99"), Some(-99));
        assert_eq!(parse_amount("-.99"), Some(-99));
    }

    #[test]
    fn test_parse_amount_invalid() {
        assert_eq!(parse_amount("abc"), None);
        assert_eq!(parse_amount(""), None);
        assert_eq!(parse_amount("-"), None);
    }
}
