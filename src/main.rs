
use std::io::{stdin,stdout,Write};

const RADIX: u32 = 10;

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
    pub value: Option<String>
}

impl Token {
    // fn new(token_type: TokenType, value: Option<Value>) -> Token {
    fn new(token_type: TokenType, value: Option<String>) -> Token {
        Token {
            token_type,
            value
        }
    }
}

#[derive(Clone, Debug)]
struct Interpreter {
    pub text: String,
    pub pos: usize,
    pub current_token: Option<Token>
}

impl Interpreter {
    fn new(text: String) -> Interpreter {
        Interpreter {
            text,
            pos: 0,
            current_token: None
        }
    }

    fn get_next_token(&mut self) -> Token {
        if self.pos > self.text.len() - 1 {
            return Token::new(TokenType::Eof, None)
        }

        let mut chars = self.text.chars();
        let current_char = chars.nth(self.pos).unwrap();

        if current_char.is_digit(RADIX)  {
            let mut s = current_char.to_string();
            let next_char_option = chars.next();
            if next_char_option.is_none() {
                self.pos += s.len();
                return Token::new(
                    TokenType::Number,
                    Some(s)
                );
            } 
            let mut next_char = next_char_option.unwrap();
            while next_char.is_digit(RADIX)  {
                s.push(next_char);
                let next_char_option = chars.next();
                if next_char_option.is_none() {
                    next_char = 'n';
                    continue;
                }
                next_char = next_char_option.unwrap();
            }
            self.pos += s.len();
            return Token::new(
                TokenType::Number,
                Some(s)
            );
        };

        match current_char {
            ' ' => {
                self.pos += 1;
                return self.get_next_token();
            },
            '+' => {
                self.pos += 1;
                return Token::new(
                    TokenType::Plus,
                    Some(current_char.to_string())
                );
            },
            '-' => {
                self.pos += 1;
                return Token::new(
                    TokenType::Minus,
                    Some(current_char.to_string())
                )
            },
            _ => panic!("Can't get next token")
        }
    }
    
    fn eat(&mut self,)  {
        self.current_token = Some(self.get_next_token());
    }

    fn expr(&mut self) -> f64 {
        self.current_token = Some(self.get_next_token());

        let left = self.current_token.clone();
        self.eat();

        let op = self.current_token.clone();
        self.eat();

        let right = self.current_token.clone();
        self.eat();

        let op = op.unwrap().value.unwrap();
        let left = left.unwrap().value.unwrap(); 
        let right = right.unwrap().value.unwrap();

        match op.as_str() {
            "+" => {
                let left = left.as_str().parse::<f64>().unwrap();
                let right = right.as_str().parse::<f64>().unwrap();
                return left + right;
            },
            "-" => {
                let left = left.as_str().parse::<f64>().unwrap();
                let right = right.as_str().parse::<f64>().unwrap();
                return left - right;
            },
            _ => {
                panic!("unallowed operator")
            }
        }
    }
}

fn main() {
    loop {
        let mut  s = String::new();
        print!("calc> ");
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
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
