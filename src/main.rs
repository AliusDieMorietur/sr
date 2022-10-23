use core::panic;
use std::io::{stdin, stdout, Write};

const RADIX: u32 = 10;

#[derive(PartialEq, Copy, Clone, Debug)]
enum Operation {
    Plus,
    Minus,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum TokenType {
    Number,
    Plus,
    Minus,
    Eof,
}

// #[derive(Clone, Debug)]
// enum Value {
//     Number(f64),
//     String(String)
// }

// impl Value {
//     fn number(self) -> f64 {
//         if let Value::Number(n) = self { n } else { panic!("Not a number") }
//     }

//     fn string(self) -> String {
//         if let Value::String(s) = self { s } else { panic!("Not a string") }
//     }
// }

#[derive(Clone, Debug)]
struct Token {
    pub token_type: TokenType,
    // pub value: Option<Value>
    pub value: Option<String>,
}

impl Token {
    // fn new(token_type: TokenType, value: Option<Value>) -> Token {
    fn new(token_type: TokenType, value: Option<String>) -> Token {
        Token { token_type, value }
    }

    fn value_to_number(&self) -> f64 {
        self.value.clone().unwrap().as_str().parse::<f64>().unwrap()
    }
}

#[derive(Clone, Debug)]
struct Interpreter {
    pub text: String,
    pub pos: usize,
    pub current_token: Option<Token>,
    pub current_char: Option<char>,
}

impl Interpreter {
    fn new(text: String) -> Interpreter {
        Interpreter {
            text: text.clone(),
            pos: 0,
            current_token: None,
            current_char: text.clone().chars().nth(0),
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
        if self.pos > self.text.len() - 1 {
            self.current_char = None;
        } else {
            self.current_char = self.text.chars().nth(self.pos);
        }
    }

    fn number(&mut self) -> String {
        let mut s = String::from("");
        while let Some(current_char) = self.current_char {
            if !current_char.is_digit(RADIX) {
                break;
            }
            s.push(current_char);
            self.advance();
        }
        return s;
    }

    fn get_next_token(&mut self) -> Token {
        while let Some(current_char) = self.current_char {
            if current_char.is_digit(RADIX) {
                return Token::new(TokenType::Number, Some(self.number()));
            }
            match current_char {
                ' ' => {
                    self.skip_whitespace();
                    continue;
                }
                '+' => {
                    self.advance();
                    return Token::new(TokenType::Plus, Some(current_char.to_string()));
                }
                '-' => {
                    self.advance();
                    return Token::new(TokenType::Minus, Some(current_char.to_string()));
                }
                _ => panic!("Can't get next token"),
            }
        }
        return Token::new(TokenType::Eof, None);
    }

    fn skip_whitespace(&mut self) {
        if let Some(current_char) = self.current_char {
            if current_char == ' ' {
                self.advance();
            }
        }
    }

    fn eat(&mut self, token_type: TokenType) {
        if let Some(current_token) = self.current_token.clone() {
            if current_token.token_type == token_type {
                self.current_token = Some(self.get_next_token());
            } else {
                panic!("Unallowed token type");
            }
        }
    }

    fn expr(&mut self) -> f64 {
        self.current_token = Some(self.get_next_token());

        let mut result = self.current_token.clone().unwrap().value_to_number();
        let mut current_number;
        let mut current_operation: Option<Operation> = None;

        self.eat(TokenType::Number);

        while self.current_token.clone().unwrap().token_type != TokenType::Eof {
            let token_type = self.current_token.clone().unwrap().token_type;
            if token_type == TokenType::Number {
                current_number = self.current_token.clone().unwrap().value_to_number();
                self.eat(TokenType::Number);
                match current_operation {
                    Some(Operation::Minus) => {
                        result -= current_number;
                    }
                    Some(Operation::Plus) => {
                        result += current_number;
                    }
                    None => {
                        panic!("Unallowed operator")
                    }
                }
                continue;
            }
            match token_type {
                TokenType::Minus => {
                    current_operation = Some(Operation::Minus);
                    self.eat(TokenType::Minus);
                }
                TokenType::Plus => {
                    current_operation = Some(Operation::Plus);
                    self.eat(TokenType::Plus);
                }
                _ => {
                    panic!("Unexpected operator")
                }
            }
        }
        result
    }
}

fn main() {
    loop {
        let mut s = String::new();
        print!("calc> ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        let mut interpreter = Interpreter::new(s);
        let result = interpreter.expr();
        println!("{}", result);
    }
}
