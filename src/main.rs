pub fn concat(a: &str, b: &str) -> String {
    [a, b].join("")
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_concat() {
        let a = "a";
        let mut ma = a;
        let result = concat_with_b(ma);

        ma = &result;
        assert_eq!(ma, "ab");
    }

    fn concat_with_b(it: &str) -> String {
        concat(it, "b")
    }
}
