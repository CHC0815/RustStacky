#[cfg(test)]
mod tests {
    use crate::stacky::Stacky;

    #[test]
    fn add() {
        let input = "1 2 + .";
        let mut stacky = Stacky::new();
        let tokens = stacky.lex(&input);
        let ast = stacky.parse(&tokens);
        stacky.run(&ast);
    }

    #[test]
    fn word() {
        let input = "";
    }
}
