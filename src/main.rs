use std::collections::VecDeque;
use evaluator::evaluate;

use crate::tokenizer::Element; // to edit these imports for testing purposes only 

mod helpers;
mod shunting_yard;
mod tokenizer;
mod test; 
mod evaluator;

fn main() {

    let parsed: VecDeque<Element> = tokenizer::parse_input("-7".to_string()); // 214
    println!("parsed: {:?}", parsed); 
    let output: VecDeque<Element> = shunting_yard::convert_postfix(parsed); 
    println!("converted: {:?}", output);

    let result = evaluate(output); 
    println!("result: {}", result); 

}
