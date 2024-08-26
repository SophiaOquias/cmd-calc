pub fn is_operator(ch: char) -> bool {
    matches!(ch, '+' | '-' | '*' | '/' | '^' | '(' | ')')
}

pub fn get_op_value(ch: char) -> i32 {
    match ch {
        '^' => 5, 
        '*' | '/' => 3,
        '+' | '-' => 1, 
        _ => -1
    }
}