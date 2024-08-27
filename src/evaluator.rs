use std::collections::VecDeque;
use crate::tokenizer::Element; 

fn perform_operation(op1: f64, op2: f64, operation: char) -> f64 {
    match operation {
        '*' => return op1 * op2, 
        '/' => return op1 / op2, 
        '+' => return op1 + op2, 
        '-' => return op1 - op2,
        _ => return 0., 
    }
}

pub fn evaluate(mut postfix: VecDeque<Element>) -> f64 {
    let mut stack: Vec<f64> = Vec::new(); 

    while let Some(front) = postfix.pop_front() {
        match front {
            Element::Number(i) => stack.push(i as f64), 
            Element::Operator(o) => {
                let op2: f64 = stack.pop().unwrap_or_default(); 
                let op1: f64 = stack.pop().unwrap_or_default();

                let result = perform_operation(op1, op2, o);
                stack.push(result); 
            }
        }
    }

    let final_result = stack.pop().unwrap_or_default(); 

    return final_result;
}