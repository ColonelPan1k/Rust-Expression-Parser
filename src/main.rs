#[allow(unused_variables, dead_code)]
// I can't figure out how to properly import the token module,
// pub mod token; doesn't seem to want to work so I've just
// included the enum TokenType and struct Token in this file
// for the time being.

// It never gets old writing lexers in new languages

enum TokenType {
    PLUS, MINUS, STAR, SLASH,
    LParen, RParen, END, NUMBER,
    NONE
}

struct Token {
    t_type: TokenType,
    lexeme: String,
    line: isize,
}

impl Token {
    fn new() -> Self{
        Token {
            t_type: TokenType::NONE,
            lexeme: String::from(""),
            line: 0
        }
    }
}

// There's probably a way to work this into new() but I'll figure it out later
fn create_token(token_type: TokenType, lexeme: String, line: isize) -> Token{
    let mut new_token = Token::new();

    new_token.t_type = token_type;
    new_token.lexeme = String::from(lexeme);
    new_token.line = line;

    return new_token;
}
pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: isize,
    token_pos: usize,
}


impl Lexer{
    fn new() -> Self {
        Lexer {
            source: String::from(""),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            token_pos: 0, // is this needed when using a vector?
        }
    }

    fn is_at_end(&mut self) -> bool {
        return self.current >= self.source.chars().count();
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        return self.source.chars().nth(self.current - 1).unwrap();
    }

    fn peek(&mut self) -> char {
        return self.source.chars().nth(self.current).unwrap();
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
        self.token_pos += 1;
    }

    fn add_lexeme_token(&mut self, token_type: TokenType) {

        let text = &self.source[self.start..self.current];

        let _to_add = Token{
            t_type: token_type,
            lexeme: text.to_string(),
            line: self.line,
        };

        self.add_token(_to_add);
    }

    // This needs the `!self.is_at_end()` or else it will hit a None type error
    fn number(&mut self) {
        while !self.is_at_end() && self.peek().is_digit(10){
            self.advance();
        }
        self.add_lexeme_token(TokenType::NUMBER);
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_lexeme_token(TokenType::LParen),
            ')' => self.add_lexeme_token(TokenType::RParen),
            '+' => self.add_lexeme_token(TokenType::PLUS),
            '-' => self.add_lexeme_token(TokenType::MINUS),
            '*' => self.add_lexeme_token(TokenType::STAR),
            '/' => self.add_lexeme_token(TokenType::SLASH),
            ' ' => (),
            _ => if c.is_digit(10) {
                self.number();
            } else {
                panic!("when the error is SUS {}", self.line);
            }
        }

    }

    fn scan_tokens(&mut self) {
        while !(self.is_at_end()){
            self.start = self.current;
            self.scan_token();
        }
        self.add_token(create_token(TokenType::END, "".to_string(), self.line));
    }
}

pub fn init_lexer(prog_source: &str) -> Lexer {
    let mut new_lexer = Lexer::new();

    new_lexer.source = String::from(prog_source);

    return new_lexer;
}


// Hacky but this should work just for now
fn print_tokens(vec: &Vec<Token>) {
    for i in 0..vec.len() {
        println!("Lexeme: {}", vec[i].lexeme);
    }
}


fn main() {
    let mut test_lexer = init_lexer("73 + (10 * 5) - 12");
    test_lexer.scan_tokens();
    print_tokens(&test_lexer.tokens);

    println!("SUCCESS");
}
