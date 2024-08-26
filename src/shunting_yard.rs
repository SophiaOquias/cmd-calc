use std::collections::VecDeque;
use crate::tokenizer::Element;
use crate::helpers;

pub fn convert_postfix(infix: VecDeque<Element>) -> VecDeque<Element>{

    let mut op_stack: Vec<char> = Vec::new(); 
    let mut out_queue: VecDeque<Element> = VecDeque::new(); 

    for item in &infix {

        match item {
            Element::Integer(i) => out_queue.push_back(Element::Integer(*i)), 
            Element::Operator(o) => {
                match *o {
                    '(' => op_stack.push(*o),
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
                    }
                    _ => {
                        while let Some(top) = op_stack.last() {
                            if helpers::get_op_value(*top) >= helpers::get_op_value(*o) {
                                out_queue.push_back(Element::Operator(*top)); 
                                op_stack.pop(); 
                            }
                            else {
                                break; 
                            }
                        }
                        op_stack.push(*o); 
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