use std::collections::VecDeque;
use crate::tokenizer::Element; // to edit these imports for testing purposes only 

mod helpers;
mod shunting_yard;
mod tokenizer;
mod test; 

fn main() {

    let parsed: VecDeque<Element> = tokenizer::parse_input("10 + 6 * (34 - 6 / 2 + 6 / 2)".to_string()); // 214
    let output: VecDeque<Element> = shunting_yard::convert_postfix(parsed); 

    for item in &output {
        match item {
            Element::Integer(i) => print!("{}", i), 
            Element::Operator(o) => print!("{}", o), 
        }
        print!(" ");
    }

}
