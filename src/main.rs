use std::collections::VecDeque;

enum Element {
    Integer(i32),
    // Float(f64), 
    Operand(char)
}

fn is_operand(ch: char) -> bool {
    if ch == '+' || ch == '-' || ch == '*' || ch == '/' || ch == '^' {
        return true; 
    }
    return false; 
}

fn is_number(ch: char) -> bool {
    if ch >= '0' && ch <= '9' {
        return true; 
    }
    return false; 
}

fn get_op_value(ch: char) -> i32 {
    if ch == '^' {
        return 5; 
    }
    else if ch == '/' || ch == '*' {
        return 3; 
    }
    else if ch == '+' || ch == '-' {
        return 1; 
    }
    
    return -1; 
}

fn parse_input(input: String) -> VecDeque<Element> {
    let mut output: VecDeque<Element> = VecDeque::new(); 
    let mut temp: String = "".to_string(); 

    for ch in input.chars() {
        if is_number(ch) {
            temp.push(ch); 
        }
        else {
            if temp != "" {
                let number = temp.parse().expect("Error parsing to int");
                output.push_back(Element::Integer(number)); 
                temp = "".to_string(); 
            } 
            if ch != ' ' && is_operand(ch) {
                output.push_back(Element::Operand(ch)); 
            }
        }
    }

    if temp != "" {
        let number = temp.parse().expect("Error parsing to int");
        output.push_back(Element::Integer(number)); 
    }

    return output; 
}

fn convert_postfix(infix: VecDeque<Element>) -> VecDeque<Element>{
    // let mut last = infix.front_mut(); 

    let mut op_stack: Vec<char> = Vec::new(); 
    let mut out_queue: VecDeque<Element> = VecDeque::new(); 

    for item in &infix {

        match item {
            Element::Integer(i) => out_queue.push_back(Element::Integer(*i)), 
            Element::Operand(o) => {
                match *o {
                    '(' => op_stack.push(*o),
                    ')' => {
                        while let Some(top) = op_stack.last() {
                            if *top == '(' {
                                op_stack.pop(); 
                                break; 
                            }
                            else {
                                out_queue.push_back(Element::Operand(*top)); 
                                op_stack.pop(); 
                            }
                        }
                    }
                    _ => {
                        while let Some(top) = op_stack.last() {
                            if get_op_value(*top) >= get_op_value(*o) {
                                out_queue.push_back(Element::Operand(*top)); 
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
        out_queue.push_back(Element::Operand(top)); 
    }

    return out_queue; 

}

fn main() {

    let parsed: VecDeque<Element> = parse_input("10 + 6 * (34 - 6 / 2 + 6 / 2)".to_string()); // 214
    let output: VecDeque<Element> = convert_postfix(parsed); 

    for item in &output {
        match item {
            Element::Integer(i) => print!("{}", i), 
            // Element::Float(f) => print!("{}", f), 
            Element::Operand(o) => print!("{}", o), 
        }
        print!(" ");
    }

}
