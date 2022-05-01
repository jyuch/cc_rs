pub(crate) fn strtol(value: &str) -> (i32, String) {
    let position = value
        .find(|it: char| !it.is_numeric())
        .unwrap_or(value.len());

    let n: i32 = value[0..position].parse().unwrap();
    (n, value[position..].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strtol_test() {
        let (n, remain) = strtol("123+");
        assert_eq!(n, 123);
        assert_eq!(remain, "+");
    }
}
