pub fn is_operator(ch: char) -> bool {
    matches!(ch, '+' | '-' | '*' | '/' | '^' | '(' | ')')
}

pub fn get_op_value(ch: char) -> (i32, bool) {
    match ch {
        '^' => (5, true), 
        '*' | '/' => (3, false),
        '+' | '-' => (1, false), 
        _ => (-1, false)
    }
}