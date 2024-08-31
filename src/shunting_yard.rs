use std::collections::VecDeque;
use crate::tokenizer::Element;
use crate::helpers;

fn handle_precedence(op_stack: &mut Vec<char>, out_queue: &mut VecDeque<Element>, operator: char) {
    while let Some(top) = op_stack.last() {
        let (top_prec, _top_right_assoc) = helpers::get_op_value(*top);
        let (op_prec, op_right_assoc) = helpers::get_op_value(operator); 

        if top_prec > op_prec || (top_prec == op_prec && !op_right_assoc) {
            out_queue.push_back(Element::Operator(*top)); 
            op_stack.pop(); 
        }
        else {
            break; 
        }
    }
}

pub fn convert_postfix(mut infix: VecDeque<Element>) -> VecDeque<Element>{

    let mut op_stack: Vec<char> = Vec::new(); 
    let mut out_queue: VecDeque<Element> = VecDeque::new(); 
    let mut expect_unary = true; 

    // for item in &infix 
    while let Some(front) = infix.pop_front(){

        match front {
            Element::Number(n) => {
                out_queue.push_back(Element::Number(n));
                expect_unary = false; 
            } 
            Element::Operator(o) => {
                match o {
                    '(' => {
                        op_stack.push(o);
                        expect_unary = true; 
                    }
                    ')' => {
                        while let Some(top) = op_stack.last() {
                            if *top == '(' {
                                op_stack.pop(); 
                                break; 
                            }
                            else {
                                out_queue.push_back(Element::Operator(*top)); 
                                op_stack.pop(); 
                            }
                        }
                        expect_unary = false; 
                    }
                    '-' => {
                        if expect_unary {
                            if let Element::Number(front) = infix.front().unwrap() {
                                out_queue.push_back(Element::Number(*front)); 
                                infix.pop_front(); 
                            }
                            out_queue.push_back(Element::Number(-1.));
                            out_queue.push_back(Element::Operator('*'));
                            
                        }
                        else {
                            handle_precedence(&mut op_stack, &mut out_queue, o); 
                            op_stack.push(o);
                        }
                        expect_unary = true; 
                    }
                    '^' => {
                        handle_precedence(&mut op_stack, &mut out_queue, o); 
                        op_stack.push(o); 
                        expect_unary = true; 
                    }
                    _ => {
                        handle_precedence(&mut op_stack, &mut out_queue, o);
                        op_stack.push(o); 
                        expect_unary = true; 
                    }
                }
            }
        }
    }

    while let Some(top) = op_stack.pop() {
        out_queue.push_back(Element::Operator(top)); 
    }

    return out_queue; 

}