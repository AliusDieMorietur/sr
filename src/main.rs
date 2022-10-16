
use std::io::{stdin,stdout,Write};

const RADIX: u32 = 10;

#[derive(PartialEq, Copy, Clone, Debug)]
enum TokenType {
    Number, Plus, Eof
}

#[derive(Clone, Debug)]
enum Value {
    Number(f64),
    String(String)
}

impl Value {
    fn number(self) -> f64 {
        if let Value::Number(n) = self { n } else { panic!("Not a number") }
    }

    fn string(self) -> String {
        if let Value::String(s) = self { s } else { panic!("Not a string") }
    }
}


#[derive(Clone, Debug)]
struct Token {
    pub token_type: TokenType,
    pub value: Option<Value>
}

impl Token {
    fn new(token_type: TokenType, value: Option<Value>) -> Token {
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

        let current_char = self.text.chars().nth(self.pos).unwrap();

        if current_char.is_digit(RADIX) {
            let token = Token::new(
                TokenType::Number,
                Some(
                    Value::Number(
                        current_char
                            .to_digit(RADIX)
                            .unwrap()
                            as f64
                        )
                    )
                );
            self.pos += 1;
            return token;
        }

        if current_char == '+' {
            let token = Token::new(
                TokenType::Plus,
                Some(
                    Value::String(
                        current_char
                            .to_string()
                        )
                    )
                );
            self.pos += 1;
            return token
        }

       panic!("Can't get next token")
    }
    
    fn eat(&mut self, token_type: TokenType)  {
        let current_token = self.current_token.clone().unwrap();
        // println!("current_token: {:?}", current_token);
        // println!("token_tpye: {:?}", token_type);
        if current_token.token_type == token_type {
            self.current_token = Some(self.get_next_token());
        } else {
            panic!("Can't get next token")
        }
    }

    fn expr(&mut self) -> f64 {
        self.current_token = Some(self.get_next_token());

        let left = self.current_token.clone();
        self.eat(TokenType::Number);

        // println!("after eat: {:?}", self.current_token);

        let op = self.current_token.clone();
        self.eat(TokenType::Plus);

        // println!("after eat: {:?}", self.current_token);

        let right = self.current_token.clone();
        self.eat(TokenType::Number);

        // println!("after eat: {:?}", self.current_token);

        println!("op: {:?}", op);
        println!("left: {:?}", left);
        println!("right: {:?}", right);

        let left_value = left.unwrap().value.unwrap().number(); 
        
        let right_value = right.unwrap().value.unwrap().number();

        left_value + right_value
    }
}

fn main() {
    loop {
        let mut  s= String::new();
        print!("calc> ");
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        println!("s: |{}|", s);
        let mut interpreter = Interpreter::new(s);
        let result = interpreter.expr();
        println!("{}", result);
    }
}
