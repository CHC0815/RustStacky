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
    fn sub() {
        let input = "1 2 - .";
        let mut output: Vec<u8> = Vec::new();
        let mut stacky = Stacky::new();
        let tokens = stacky.lex(&input);
        let ast = stacky.parse(&tokens);
        stacky.run(&ast, &mut output);

        let _s = match String::from_utf8(output) {
            Ok(v) => assert!(v == "-1"),
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

    #[test]
    fn if_test_true() {
        let input = "1 1 = IF \"TRUE\" ELSE \"FALSE\" THEN PUTS";
        let mut output: Vec<u8> = Vec::new();
        let mut stacky = Stacky::new();
        let tokens = stacky.lex(&input);
        let ast = stacky.parse(&tokens);
        stacky.run(&ast, &mut output);

        let _s = match String::from_utf8(output) {
            Ok(v) => assert!(v == "TRUE"),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    }
    #[test]
    fn if_test_false() {
        let input = "1 2 = IF \"TRUE\" ELSE \"FALSE\" THEN PUTS";
        let mut output: Vec<u8> = Vec::new();
        let mut stacky = Stacky::new();
        let tokens = stacky.lex(&input);
        let ast = stacky.parse(&tokens);
        stacky.run(&ast, &mut output);

        let _s = match String::from_utf8(output) {
            Ok(v) => assert!(v == "FALSE"),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    }

    #[test]
    fn lt() {
        let input = "1 2 < .";
        let mut output: Vec<u8> = Vec::new();
        let mut stacky = Stacky::new();
        let tokens = stacky.lex(&input);
        let ast = stacky.parse(&tokens);
        stacky.run(&ast, &mut output);

        let _s = match String::from_utf8(output) {
            Ok(v) => assert!(v == "1"),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    }

    #[test]
    fn lt_false() {
        let input = "2 2 < .";
        let mut output: Vec<u8> = Vec::new();
        let mut stacky = Stacky::new();
        let tokens = stacky.lex(&input);
        let ast = stacky.parse(&tokens);
        stacky.run(&ast, &mut output);

        let _s = match String::from_utf8(output) {
            Ok(v) => assert!(v == "0"),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    }

        #[test]
    fn lte_equals() {
        let input = "2 2 <= .";
        let mut output: Vec<u8> = Vec::new();
        let mut stacky = Stacky::new();
        let tokens = stacky.lex(&input);
        let ast = stacky.parse(&tokens);
        stacky.run(&ast, &mut output);

        let _s = match String::from_utf8(output) {
            Ok(v) => assert!(v == "1"),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    }

    #[test]
    fn lte_less() {
        let input = "1 2 <= .";
        let mut output: Vec<u8> = Vec::new();
        let mut stacky = Stacky::new();
        let tokens = stacky.lex(&input);
        let ast = stacky.parse(&tokens);
        stacky.run(&ast, &mut output);

        let _s = match String::from_utf8(output) {
            Ok(v) => assert!(v == "1"),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    }

    #[test]
    fn lte_false() {
        let input = "3 2 <= .";
        let mut output: Vec<u8> = Vec::new();
        let mut stacky = Stacky::new();
        let tokens = stacky.lex(&input);
        let ast = stacky.parse(&tokens);
        stacky.run(&ast, &mut output);

        let _s = match String::from_utf8(output) {
            Ok(v) => assert!(v == "0"),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    }

    #[test]
    fn gt() {
        let input = "3 2 > .";
        let mut output: Vec<u8> = Vec::new();
        let mut stacky = Stacky::new();
        let tokens = stacky.lex(&input);
        let ast = stacky.parse(&tokens);
        stacky.run(&ast, &mut output);

        let _s = match String::from_utf8(output) {
            Ok(v) => assert!(v == "1"),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    }

    #[test]
    fn gt_false() {
        let input = "2 2 > .";
        let mut output: Vec<u8> = Vec::new();
        let mut stacky = Stacky::new();
        let tokens = stacky.lex(&input);
        let ast = stacky.parse(&tokens);
        stacky.run(&ast, &mut output);

        let _s = match String::from_utf8(output) {
            Ok(v) => assert!(v == "0"),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    }

    #[test]
    fn gte_equals() {
        let input = "2 2 >= .";
        let mut output: Vec<u8> = Vec::new();
        let mut stacky = Stacky::new();
        let tokens = stacky.lex(&input);
        let ast = stacky.parse(&tokens);
        stacky.run(&ast, &mut output);

        let _s = match String::from_utf8(output) {
            Ok(v) => assert!(v == "1"),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    }

    #[test]
    fn gte_greater() {
        let input = "3 2 >= .";
        let mut output: Vec<u8> = Vec::new();
        let mut stacky = Stacky::new();
        let tokens = stacky.lex(&input);
        let ast = stacky.parse(&tokens);
        stacky.run(&ast, &mut output);

        let _s = match String::from_utf8(output) {
            Ok(v) => assert!(v == "1"),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    }

    #[test]
    fn gte_false() {
        let input = "1 2 >= .";
        let mut output: Vec<u8> = Vec::new();
        let mut stacky = Stacky::new();
        let tokens = stacky.lex(&input);
        let ast = stacky.parse(&tokens);
        stacky.run(&ast, &mut output);

        let _s = match String::from_utf8(output) {
            Ok(v) => assert!(v == "0"),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    }

    #[test]
    #[should_panic (expected = "Expected identifier")]
    fn expected_identifier() {
        let input = ": 1 2 + ;";
        let mut output: Vec<u8> = Vec::new();
        let mut stacky = Stacky::new();
        let tokens = stacky.lex(&input);
        let ast = stacky.parse(&tokens);
        stacky.run(&ast, &mut output);
    }
}
