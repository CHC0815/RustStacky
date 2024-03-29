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
    fn mul() {
        let input = "3 2 * .";
        let mut output: Vec<u8> = Vec::new();
        let mut stacky = Stacky::new();
        let tokens = stacky.lex(&input);
        let ast = stacky.parse(&tokens);
        stacky.run(&ast, &mut output);

        let _s = match String::from_utf8(output) {
            Ok(v) => assert!(v == "6"),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    }

    #[test]
    fn div() {
        let input = "4 2 / .";
        let mut output: Vec<u8> = Vec::new();
        let mut stacky = Stacky::new();
        let tokens = stacky.lex(&input);
        let ast = stacky.parse(&tokens);
        stacky.run(&ast, &mut output);

        let _s = match String::from_utf8(output) {
            Ok(v) => assert!(v == "2"),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    }

    #[test]
    fn div_not_integer() {
        let input = "3 2 / .";
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
    #[should_panic(expected = "Expected identifier")]
    fn expected_identifier() {
        let input = ": 1 2 + ;";
        let mut output: Vec<u8> = Vec::new();
        let mut stacky = Stacky::new();
        let tokens = stacky.lex(&input);
        let ast = stacky.parse(&tokens);
        stacky.run(&ast, &mut output);
    }

    #[test]
    fn dup() {
        let input = "2 DUP + .";
        let mut output: Vec<u8> = Vec::new();
        let mut stacky = Stacky::new();
        let tokens = stacky.lex(&input);
        let ast = stacky.parse(&tokens);
        stacky.run(&ast, &mut output);

        let _s = match String::from_utf8(output) {
            Ok(v) => assert!(v == "4"),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    }

    #[test]
    fn swap() {
        let input = "1 2 SWAP . .";
        let mut output: Vec<u8> = Vec::new();
        let mut stacky = Stacky::new();
        let tokens = stacky.lex(&input);
        let ast = stacky.parse(&tokens);
        stacky.run(&ast, &mut output);

        let _s = match String::from_utf8(output) {
            Ok(v) => assert!(v == "21"),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    }

    #[test]
    fn drop() {
        let input = "1 2 DROP . ";
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
    fn loop_test() {
        let input = "10 0 DO 1 . LOOP";
        let mut output: Vec<u8> = Vec::new();
        let mut stacky = Stacky::new();
        let tokens = stacky.lex(&input);
        let ast = stacky.parse(&tokens);
        stacky.run(&ast, &mut output);

        let _s = match String::from_utf8(output) {
            Ok(v) => assert!(v == "1111111111"),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    }

    #[test]
    fn loop_no_iteration() {
        let input = "2 2 DO 1 . LOOP";
        let mut output: Vec<u8> = Vec::new();
        let mut stacky = Stacky::new();
        let tokens = stacky.lex(&input);
        let ast = stacky.parse(&tokens);
        stacky.run(&ast, &mut output);

        let _s = match String::from_utf8(output) {
            Ok(v) => assert!(v == ""),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    }

    #[test]
    fn negative_number() {
        let input = "10 -1 + .";
        let mut output: Vec<u8> = Vec::new();
        let mut stacky = Stacky::new();
        let tokens = stacky.lex(&input);
        let ast = stacky.parse(&tokens);
        stacky.run(&ast, &mut output);

        let _s = match String::from_utf8(output) {
            Ok(v) => assert!(v == "9"),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    }
}
