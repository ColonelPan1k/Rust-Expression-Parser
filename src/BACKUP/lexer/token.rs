enum TokenType {
    PLUS, MINUS, STAR, SLASH,
    L_PAREN, R_PAREN, END, NUMBER,
    NONE
}

struct Token {
    t_type: TokenType,
    lexeme: String,
    // Figure out void* in Rust
    line: isize,
}

impl Token {
    fn new() -> Self{
        Token {
            t_type: NONE,
            lexeme: String::from(""),
            line:0,
        }
    }
}

pub fn create_token(token_type: TokenType, lexeme: String, line: isize) -> Token {
    let new_token = Token::default();

    new_token.t_type = token_type;
    new_token.lexeme = lexeme.to_string();
    new_token.line = line;

    return new_token;
}
