use std::collections::VecDeque;
use crate::helpers::is_operator;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Element {
    Number(f64),
    Operator(char)
}

pub(crate) fn parse_input(input: String) -> VecDeque<Element> {
    let mut output: VecDeque<Element> = VecDeque::new(); 
    let mut temp: String = "".to_string(); 

    for ch in input.chars() {
        if ch.is_digit(10) || ch == '.' {
            temp.push(ch); 
        }
        else {
            if !temp.is_empty() {
                let number = temp.parse().expect("Error parsing to int");
                output.push_back(Element::Number(number)); 
                temp = "".to_string(); 
            } 
            if ch != ' ' && is_operator(ch) {
                output.push_back(Element::Operator(ch)); 
            }
        }
    }

    if !temp.is_empty() {
        let number = temp.parse().expect("Error parsing to int");
        output.push_back(Element::Number(number)); 
    }

    return output; 
}