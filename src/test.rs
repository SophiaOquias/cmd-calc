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
            Element::Integer(100), 
            Element::Operator('+'),
            Element::Integer(2), 
            Element::Operator('/'), 
            Element::Operator('('), 
            Element::Integer(5), 
            Element::Operator('*'),
            Element::Integer(34), 
            Element::Operator('*'), 
            Element::Integer(2), 
            Element::Operator('+'), 
            Element::Integer(7), 
            Element::Operator(')') 
        ]);

        assert_eq!(result, expected); 
    }

    #[test]
    fn test_space_between_digits() {
        let input = "10 0 + 2 * 5+3".to_string(); 
        let result = parse_input(input); 
        let expected = VecDeque::from(vec![
            Element::Integer(10), 
            Element::Integer(0),
            Element::Operator('+'), 
            Element::Integer(2), 
            Element::Operator('*'), 
            Element::Integer(5), 
            Element::Operator('+'), 
            Element::Integer(3)
        ]);

        assert_eq!(result, expected); 
    }

    #[test]
    fn test_two_operands() {
        let input = "100 * 5".to_string(); 
        let result = parse_input(input); 
        let expected = VecDeque::from(vec![
            Element::Integer(100), 
            Element::Operator('*'), 
            Element::Integer(5)
        ]); 
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_simple_addition() {
        let infix = VecDeque::from(vec![
            Element::Integer(3),
            Element::Operator('+'),
            Element::Integer(4),
        ]);
        let result = convert_postfix(infix);
        let expected = VecDeque::from(vec![
            Element::Integer(3),
            Element::Integer(4),
            Element::Operator('+'),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_addition_and_multiplication() {
        let infix = VecDeque::from(vec![
            Element::Integer(3),
            Element::Operator('+'),
            Element::Integer(4),
            Element::Operator('*'),
            Element::Integer(5),
        ]);
        let result = convert_postfix(infix);
        let expected = VecDeque::from(vec![
            Element::Integer(3),
            Element::Integer(4),
            Element::Integer(5),
            Element::Operator('*'),
            Element::Operator('+'),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_with_parentheses() {
        let infix = VecDeque::from(vec![
            Element::Integer(3),
            Element::Operator('+'),
            Element::Operator('('),
            Element::Integer(4),
            Element::Operator('*'),
            Element::Integer(5),
            Element::Operator(')'),
        ]);
        let result = convert_postfix(infix);
        let expected = VecDeque::from(vec![
            Element::Integer(3),
            Element::Integer(4),
            Element::Integer(5),
            Element::Operator('*'),
            Element::Operator('+'),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_complex_expression() {
        let infix = VecDeque::from(vec![
            Element::Integer(12),
            Element::Operator('+'),
            Element::Integer(34),
            Element::Operator('*'),
            Element::Operator('('),
            Element::Integer(56),
            Element::Operator('-'),
            Element::Integer(78),
            Element::Operator('/'),
            Element::Integer(9),
            Element::Operator(')'),
        ]);
        let result = convert_postfix(infix);
        let expected = VecDeque::from(vec![
            Element::Integer(12),
            Element::Integer(34),
            Element::Integer(56),
            Element::Integer(78),
            Element::Integer(9),
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
            Element::Integer(5),
            Element::Operator('-'),
            Element::Integer(2),
        ]);
        let result = convert_postfix(infix);
        let expected = VecDeque::from(vec![
            Element::Integer(5),
            Element::Integer(2),
            Element::Operator('-'),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_only_numbers() {
        let infix = VecDeque::from(vec![
            Element::Integer(42),
        ]);
        let result = convert_postfix(infix);
        let expected = VecDeque::from(vec![
            Element::Integer(42),
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
            Element::Integer(3),
            Element::Operator('+'),
            Element::Integer(4),
            Element::Operator('*'),
            Element::Integer(5),
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
            Element::Integer(12),
            Element::Operator('+'),
            Element::Integer(34),
            Element::Operator('*'),
            Element::Operator('('),
            Element::Integer(56),
            Element::Operator('-'),
            Element::Integer(78),
            Element::Operator('/'),
            Element::Integer(9),
            Element::Operator(')'),
        ]);
        let postfix = convert_postfix(infix); 
        let result = evaluate(postfix); 
        let expected = 1621.33; 

        assert_approx_eq!(f64, result, expected, epsilon = 0.01); 
    }
}