#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use float_cmp::assert_approx_eq;

    use crate::tokenizer::parse_input;
    use crate::tokenizer::Element; 
    use crate::shunting_yard::convert_postfix; 
    use crate::evaluator::evaluate; 

    #[test]
    fn test_parse_input() {
        let input = "100 + 2 / (5*34*2 +7)".to_string();
        let result = parse_input(input); 
        let expected = VecDeque::from(vec![
            Element::Number(100.), 
            Element::Operator('+'),
            Element::Number(2.), 
            Element::Operator('/'), 
            Element::Operator('('), 
            Element::Number(5.), 
            Element::Operator('*'),
            Element::Number(34.), 
            Element::Operator('*'), 
            Element::Number(2.), 
            Element::Operator('+'), 
            Element::Number(7.), 
            Element::Operator(')') 
        ]);

        assert_eq!(result, expected); 
    }

    #[test]
    fn test_space_between_digits() {
        let input = "10 0 + 2 * 5+3".to_string(); 
        let result = parse_input(input); 
        let expected = VecDeque::from(vec![
            Element::Number(10.), 
            Element::Number(0.),
            Element::Operator('+'), 
            Element::Number(2.), 
            Element::Operator('*'), 
            Element::Number(5.), 
            Element::Operator('+'), 
            Element::Number(3.)
        ]);

        assert_eq!(result, expected); 
    }

    #[test]
    fn test_two_operands() {
        let input = "100 * 5".to_string(); 
        let result = parse_input(input); 
        let expected = VecDeque::from(vec![
            Element::Number(100.), 
            Element::Operator('*'), 
            Element::Number(5.)
        ]); 
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_simple_addition() {
        let infix = VecDeque::from(vec![
            Element::Number(3.),
            Element::Operator('+'),
            Element::Number(4.),
        ]);
        let result = convert_postfix(infix);
        let expected = VecDeque::from(vec![
            Element::Number(3.),
            Element::Number(4.),
            Element::Operator('+'),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_addition_and_multiplication() {
        let infix = VecDeque::from(vec![
            Element::Number(3.),
            Element::Operator('+'),
            Element::Number(4.),
            Element::Operator('*'),
            Element::Number(5.),
        ]);
        let result = convert_postfix(infix);
        let expected = VecDeque::from(vec![
            Element::Number(3.),
            Element::Number(4.),
            Element::Number(5.),
            Element::Operator('*'),
            Element::Operator('+'),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_with_parentheses() {
        let infix = VecDeque::from(vec![
            Element::Number(3.),
            Element::Operator('+'),
            Element::Operator('('),
            Element::Number(4.),
            Element::Operator('*'),
            Element::Number(5.),
            Element::Operator(')'),
        ]);
        let result = convert_postfix(infix);
        let expected = VecDeque::from(vec![
            Element::Number(3.),
            Element::Number(4.),
            Element::Number(5.),
            Element::Operator('*'),
            Element::Operator('+'),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_complex_expression() {
        let infix = VecDeque::from(vec![
            Element::Number(12.),
            Element::Operator('+'),
            Element::Number(34.),
            Element::Operator('*'),
            Element::Operator('('),
            Element::Number(56.),
            Element::Operator('-'),
            Element::Number(78.),
            Element::Operator('/'),
            Element::Number(9.),
            Element::Operator(')'),
        ]);
        let result = convert_postfix(infix);
        let expected = VecDeque::from(vec![
            Element::Number(12.),
            Element::Number(34.),
            Element::Number(56.),
            Element::Number(78.),
            Element::Number(9.),
            Element::Operator('/'),
            Element::Operator('-'),
            Element::Operator('*'),
            Element::Operator('+'),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_operator() {
        let infix = VecDeque::from(vec![
            Element::Number(5.),
            Element::Operator('-'),
            Element::Number(2.),
        ]);
        let result = convert_postfix(infix);
        let expected = VecDeque::from(vec![
            Element::Number(5.),
            Element::Number(2.),
            Element::Operator('-'),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_only_numbers() {
        let infix = VecDeque::from(vec![
            Element::Number(42.),
        ]);
        let result = convert_postfix(infix);
        let expected = VecDeque::from(vec![
            Element::Number(42.),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_input() {
        let infix: VecDeque<Element> = VecDeque::new();
        let result = convert_postfix(infix);
        let expected: VecDeque<Element> = VecDeque::new();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_evaluate_simple_operation() {
        let infix = VecDeque::from(vec![
            Element::Number(3.),
            Element::Operator('+'),
            Element::Number(4.),
            Element::Operator('*'),
            Element::Number(5.),
        ]);
        let postfix = convert_postfix(infix); 
        let result = evaluate(postfix); 
        let expected = 23.; 

        assert_approx_eq!(f64, result, expected, epsilon = 0.01); 
    }

    #[test]
    fn test_evaluate_complex_operation() {
        // 12 + 34 * (56 - 78 / 9)
        let infix = VecDeque::from(vec![
            Element::Number(12.),
            Element::Operator('+'),
            Element::Number(34.),
            Element::Operator('*'),
            Element::Operator('('),
            Element::Number(56.),
            Element::Operator('-'),
            Element::Number(78.),
            Element::Operator('/'),
            Element::Number(9.),
            Element::Operator(')'),
        ]);
        let postfix = convert_postfix(infix); 
        let result = evaluate(postfix); 
        let expected = 1621.33; 

        assert_approx_eq!(f64, result, expected, epsilon = 0.01); 
    }

    #[test]
    fn test_evaluate_simple_operation_float() {
        let infix = VecDeque::from(vec![
            Element::Number(3.25),
            Element::Operator('+'),
            Element::Number(4.333),
            Element::Operator('*'),
            Element::Number(5.),
        ]);
        let postfix = convert_postfix(infix); 
        let result = evaluate(postfix); 
        let expected = 24.915; 

        assert_approx_eq!(f64, result, expected, epsilon = 0.01); 
    }

    #[test]
    fn test_evaluate_complex_operation_float() {
        // 12.5 + 34 * (56.33 - 78.91 / 9.005)
        let infix = VecDeque::from(vec![
            Element::Number(12.5),
            Element::Operator('+'),
            Element::Number(34.),
            Element::Operator('*'),
            Element::Operator('('),
            Element::Number(56.33),
            Element::Operator('-'),
            Element::Number(78.91),
            Element::Operator('/'),
            Element::Number(9.005),
            Element::Operator(')'),
        ]);
        let postfix = convert_postfix(infix); 
        let result = evaluate(postfix); 
        let expected = 1629.78107717934; 

        assert_approx_eq!(f64, result, expected, epsilon = 0.01); 
    }

    #[test]
    fn test_unary_minus_with_multiplication() {
        let input = "3 * -4".to_string(); // Using parse_input
        let infix = parse_input(input);
        let postfix = convert_postfix(infix);
        let result = evaluate(postfix); 
        let expected = -12.;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_unary_minus_in_expression() {
        let input = "5 + (-3 * 4)".to_string(); // Using parse_input
        let infix = parse_input(input);
        let postfix = convert_postfix(infix);
        let result = evaluate(postfix);
        let expected = -7.;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_evaluate_unary_minus_simple() {
        let input = "-3 + 4".to_string(); // Using parse_input
        let infix = parse_input(input);
        let postfix = convert_postfix(infix); 
        let result = evaluate(postfix); 
        let expected = 1.; // -3 + 4 = 1

        assert_eq!(result, expected); 
    }

    #[test]
    fn test_evaluate_unary_minus_with_multiplication() {
        let input = "3 * -4".to_string(); // Using parse_input
        let infix = parse_input(input);
        let postfix = convert_postfix(infix); 
        let result = evaluate(postfix); 
        let expected = -12.; // 3 * -4 = -12

        assert_approx_eq!(f64, result, expected, epsilon = 0.01); 
    }

    #[test]
    fn test_evaluate_unary_minus_in_expression() {
        let input = "5 + (-3 * 4)".to_string(); // Using parse_input
        let infix = parse_input(input);
        let postfix = convert_postfix(infix); 
        let result = evaluate(postfix); 
        let expected = -7.; // 5 + (-3 * 4) = -7

        assert_approx_eq!(f64, result, expected, epsilon = 0.01); 
    }

    #[test]
    fn test_evaluate_simple_exponentiation() {
        // 2 ^ 3 = 8
        let infix = parse_input("2 ^ 3".to_string());
        let postfix = convert_postfix(infix);
        let result = evaluate(postfix);
        let expected = 8.0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_evaluate_exponent_with_unary_minus() {
        // -2 ^ 3 = -8
        let infix = parse_input("-2 ^ 3".to_string());
        let postfix = convert_postfix(infix);
        let result = evaluate(postfix);
        let expected = -8.0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_evaluate_complex_expression_with_exponents() {
        // 2 + 3 ^ 2 * 2 = 20
        let infix = parse_input("2 + 3 ^ 2 * 2".to_string());
        let postfix = convert_postfix(infix);
        let result = evaluate(postfix);
        let expected = 20.0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_evaluate_expression_with_parentheses_and_exponents() {
        // (2 + 3) ^ 2 = 25
        let infix = parse_input("(2 + 3) ^ 2".to_string());
        let postfix = convert_postfix(infix);
        let result = evaluate(postfix);
        let expected = 25.0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_evaluate_expression_with_multiple_exponents() {
        // 2 ^ 3 ^ 2 = 512
        let infix = parse_input("2 ^ 3 ^ 2".to_string());
        let postfix = convert_postfix(infix);
        let result = evaluate(postfix);
        let expected = 512.0;
        assert_eq!(result, expected);
    }

}