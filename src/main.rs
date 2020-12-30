pub fn concat(a: &str, b: &str) -> String {
    [a, b].join("")
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    struct MyStruct {
        a: String,
        b: String,
    }

    impl MyStruct {
        fn describe(self) -> String {
            concat(&self.a, &self.b)
        }
    }

    #[test]
    fn test_concat() {
        let mys = MyStruct { a: "a".to_string(), b: "b".to_string() };
        let mys2 = MyStruct { a: mys.a, b: mys.b };
        assert_eq!(mys2.describe(), "ab");

        //
        // let b = |a: &str| { concat(a, a) };
        //
        // assert_eq!(b("a"), "aa");
        // let a = "a";
        // let mut ma = a;
        //
        // let mut l = vec![ma, "c"];
        // l[0] = a;
        // let result = concat_with_b(ma);
        //
        // let mab = concat(&result.0, &result.1);
        // assert_eq!(mab, "ab");
    }

    fn concat_with_b(it: &str) -> (String, &str) {
        let my_var = "foo";
        let string = concat(it, &concat(my_var, "b"));
        (string, my_var)
    }
}
