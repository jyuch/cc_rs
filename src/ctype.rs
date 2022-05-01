pub(crate) fn is_space(s: &str) -> bool {
    if s.len() == 0 {
        false
    } else {
        match &s[0..1] {
            " " | "\r" | "\n" | "\t" => true,
            _ => false,
        }
    }
}

pub(crate) fn is_digit(s: &str) -> bool {
    match s.chars().next() {
        Some(c) => c.is_digit(10),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_space_test() {
        assert_eq!(true, is_space(" a"));
        assert_eq!(false, is_space("a b"));
    }

    #[test]
    fn is_digit_test() {
        assert_eq!(true, is_digit("1"));
        assert_eq!(false, is_digit("a"));
    }
}
