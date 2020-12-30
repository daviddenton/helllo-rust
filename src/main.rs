pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn concat(a: &str, b: &str) -> String {
    [a, b].join("")
}

// This is a really bad adding function, its purpose is to fail in this
// example.
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }    
    
    #[test]
    fn test_concat() {
        let a = "a";
        let ref_a = &a;
        let result = concat(ref_a,"b");
        
        assert_eq!(result, "ab");
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_ne!(bad_add(1, 2), 3);
    }
}
