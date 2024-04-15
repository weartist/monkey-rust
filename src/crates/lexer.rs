use super::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut res = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0' as u8,
        };
        res.read_char();
        return res;
    }

    pub fn read_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            // println!(
            //     "self input {:?}, asbyte {:?}",
            //     self.input,
            //     self.input.as_bytes()
            // );
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
        return self.ch;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            b'=' => {
                if self.peek() as char == '=' {
                    let ch = self.ch as char;
                    self.read_char();
                    self.new_token_with_string(TokenType::EQ, format!("{}{}", ch, self.ch as char))
                } else {
                    self.new_token(TokenType::ASSIGN, self.ch)
                }
            }
            b';' => self.new_token(TokenType::SEMICOLON, self.ch),
            b'(' => self.new_token(TokenType::LPAREN, self.ch),
            b')' => self.new_token(TokenType::RPAREN, self.ch),
            b',' => self.new_token(TokenType::COMMA, self.ch),
            b'+' => self.new_token(TokenType::PLUS, self.ch),
            b'{' => self.new_token(TokenType::LBRACE, self.ch),
            b'}' => self.new_token(TokenType::RBRACE, self.ch),
            b'\0' => self.new_token(TokenType::EOF, self.ch),
            b'!' => {
                if self.peek() as char == '=' {
                    let ch = self.ch as char;
                    self.read_char();
                    self.new_token_with_string(
                        TokenType::NOT_EQ,
                        format!("{}{}", ch, self.ch as char),
                    )
                } else {
                    self.new_token(TokenType::BANG, self.ch)
                }
            }
            b'-' => self.new_token(TokenType::MINUS, self.ch),
            b'/' => self.new_token(TokenType::SLASH, self.ch),
            b'*' => self.new_token(TokenType::ASTERISK, self.ch),
            b'<' => self.new_token(TokenType::LT, self.ch),
            b'>' => self.new_token(TokenType::GT, self.ch),
            _ => {
                if self.is_letter(self.ch) {
                    let identifier = self.read_identifier();
                    // println!("identifier is {:?}", identifier);
                    return self
                        .new_token_with_string(Token::look_up_ident(&identifier), identifier);
                } else if self.is_dight(self.ch) {
                    let digit = self.read_digit();
                    // println!("digit is {:?}", digit);
                    return self.new_token_with_string(TokenType::INT, digit);
                } else {
                    self.new_token(TokenType::ILLEGAL, self.ch)
                }
            }
        };

        self.read_char();
        return token;
    }

    // fn new_toekn_with_read_identifier(&mut self) -> Token {
    //     let identifier = self.read_identifier();
    //     self.new_token_with_string(Token::look_up_ident(&identifier), identifier)
    // }
    //
    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn is_letter(&self, ch: u8) -> bool {
        match ch {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => true,
            _ => false,
        }
    }

    fn is_dight(&self, ch: u8) -> bool {
        match ch {
            b'0'..=b'9' => true,
            _ => false,
        }
    }

    fn peek(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } else {
            return self.input.as_bytes()[self.read_position];
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.is_letter(self.ch) {
            self.read_char();
        }
        return self.input[position..self.position].to_string();
    }

    fn read_digit(&mut self) -> String {
        let position = self.position;
        while self.is_dight(self.ch) {
            self.read_char();
        }
        return self.input[position..self.position].to_string();
    }

    fn new_token(&self, t: TokenType, c: u8) -> Token {
        // println!(
        //     "new token, c is {:?}--{:?}",
        //     c.to_string(),
        //     (c as char).to_string()
        // );
        let cha = (c as char).to_string();
        // println!("cha is {:?}", cha);
        Token { t: t, ch: cha }
    }

    fn new_token_with_string(&self, t: TokenType, c: String) -> Token {
        Token { t: t, ch: c }
    }
}
