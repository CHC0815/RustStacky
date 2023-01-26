#[cfg(test)]
mod tests {
    use crate::stacky::Stacky;

    #[test]
    fn add() {
        let input = "1 2 + .";
        let mut output: Vec<u8> = Vec::new();
        let mut stacky = Stacky::new();
        let tokens = stacky.lex(&input);
        let ast = stacky.parse(&tokens);
        stacky.run(&ast, &mut output);

        let _s = match String::from_utf8(output) {
            Ok(v) => assert!(v == "3"),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    }

    #[test]
    fn word() {
        let input = ":Test 1 2 + . ; Test";
        let mut output: Vec<u8> = Vec::new();
        let mut stacky = Stacky::new();
        let tokens = stacky.lex(&input);
        let ast = stacky.parse(&tokens);
        stacky.run(&ast, &mut output);

        let _s = match String::from_utf8(output) {
            Ok(v) => assert!(v == "3"),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    }

    #[test]
    #[should_panic(expected = "Word not found")]
    fn word_not_defined() {
        let input = ":Test 1 2 + . ; Testl";
        let mut output: Vec<u8> = Vec::new();
        let mut stacky = Stacky::new();
        let tokens = stacky.lex(&input);
        let ast = stacky.parse(&tokens);
        stacky.run(&ast, &mut output);

    }
}
