mod helpers;
mod shunting_yard;
mod tokenizer;
mod test; 
mod evaluator;

use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    let infix = &args[1];
    let parsed = tokenizer::parse_input(infix.to_string());  
    let postfix = shunting_yard::convert_postfix(parsed); 
    let result = evaluator::evaluate(postfix); 

    println!("Result: {}", result); 

}
