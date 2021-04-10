use crate::lexer::init_lexer;

fn main() {
    // init_lexer() not found in this scope
    let mut test_lexer = init_lexer("10 + 20");
    let token = test_lexer.tokens.pop();
    println!("TEST");
}
