use std::collections::VecDeque;
use evaluator::evaluate;

use crate::tokenizer::Element; // to edit these imports for testing purposes only 

mod helpers;
mod shunting_yard;
mod tokenizer;
mod test; 
mod evaluator;

fn main() {

    let parsed: VecDeque<Element> = tokenizer::parse_input("10 + 6 * (34 - 6 / 2 + 6 / 2)".to_string()); // 214
    let output: VecDeque<Element> = shunting_yard::convert_postfix(parsed); 

    let result = evaluate(output); 

    print!("result: {}", result); 

}
