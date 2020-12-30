pub fn concat<'a>(a: &'a str, b: &'a str) -> String {
    [a, b].join("")
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    struct MyStruct<'a> {
        a: Option<&'a str>,
        b: &'a str,
    }

    impl MyStruct<'_> {
        fn describe(&self) -> Option<String> {
            Some(concat(&self.a.as_ref()?, &self.b))
        }
    }

    #[test]
    fn test_concat() {
        let b= "b";
        let mys = MyStruct { a: Some("a"), b };
        let option = Some("ab".to_string());
        assert_eq!(mys.describe(), option);
        assert_eq!(mys.describe(), option);
    }
}
