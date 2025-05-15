#[cfg(test)]
mod knorm_tests {
    use crate::ast::Expr;
    use crate::parse::parse;
    use crate::compiler::knorm::{k_normalize, knorm_to_expr};

    fn test_knorm(input: &str) -> Expr {
        let parsed = parse(input);
        let knormed = k_normalize(parsed);
        knorm_to_expr(knormed)
    }

    #[test]
    fn test_simple_application() {
        let input = "(+ 1 2)";
        let result = test_knorm(input);
        println!("Original: {}", input);
        println!("K-normalized: {}", result);
    }

    #[test]
    fn test_nested_application() {
        let input = "(+ (* 2 3) 4)";
        let result = test_knorm(input);
        println!("Original: {}", input);
        println!("K-normalized: {}", result);
    }

    #[test]
    fn test_let_binding() {
        let input = "(let ((x 1) (y 2)) (+ x y))";
        let result = test_knorm(input);
        println!("Original: {}", input);
        println!("K-normalized: {}", result);
    }

    #[test]
    fn test_if_expression() {
        let input = "(if (< 1 2) (+ 3 4) (* 5 6))";
        let result = test_knorm(input);
        println!("Original: {}", input);
        println!("K-normalized: {}", result);
    }

    #[test]
    fn test_function_definition() {
        let input = "(fn (x y) (+ x y))";
        let result = test_knorm(input);
        println!("Original: {}", input);
        println!("K-normalized: {}", result);
    }

    #[test]
    fn test_complex_expression() {
        let input = "(let ((double (fn (x) (* x 2))) (add3 (fn (x) (+ x 3))))
                     (double (add3 5)))";
        let result = test_knorm(input);
        println!("Original: {}", input);
        println!("K-normalized: {}", result);
    }

    #[test]
    fn test_complex_function() {
        let input = "((fn (x) (+ (+ x 1) 1)) 2)";
        let result = test_knorm(input);
        println!("Original: {}", input);
        println!("K-normalized: {}", result);
    }
}
