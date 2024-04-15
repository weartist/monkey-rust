mod crates;

use std::env;
use std::io;

fn main() {
    let user = env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .unwrap_or_else(|_| "unknown".to_string());
    println!("Hello {}! This is the Monkey programming language!\n", user);
    println!("Feel free to type in commands\n");

    let stdin = io::stdin();
    let input = stdin.lock();

    let stdout = io::stdout();
    let output = stdout.lock();

    crates::repl::start(input, output);
}

#[cfg(test)]
mod tests {
    use super::crates::lexer::Lexer;
    use super::crates::token::TokenType;
    // use crates::token::TokenType;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;".to_string();
        let tests: Vec<(TokenType, char)> = vec![
            (TokenType::ASSIGN, '='),
            (TokenType::PLUS, '+'),
            (TokenType::LPAREN, '('),
            (TokenType::RPAREN, ')'),
            (TokenType::LBRACE, '{'),
            (TokenType::RBRACE, '}'),
            (TokenType::COMMA, ','),
            (TokenType::SEMICOLON, ';'),
            (TokenType::EOF, '\0'),
        ];
        let mut l = Lexer::new(input);

        for (token_type, lexeme) in tests {
            let tok = l.next_token();
            println!("期望的type: {:?}, lexeme {}", token_type, lexeme);
            println!("实际的type: {:?}, lexeme {}", tok.t, tok.ch);
            assert_eq!(tok.t, token_type);
            assert_eq!(tok.ch, lexeme.to_string());
        }
    }

    #[test]
    fn test_expression() {
        let input = r#"
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
           	    return true;
            } else {
           	    return false;
            }

            10 == 10;
            10 != 9;
            "#
        .to_string();

        let tests: Vec<(TokenType, &str)> = vec![
            (TokenType::LET, "let"),
            (TokenType::IDENT, "five"),
            (TokenType::ASSIGN, "="),
            (TokenType::INT, "5"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "ten"),
            (TokenType::ASSIGN, "="),
            (TokenType::INT, "10"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "add"),
            (TokenType::ASSIGN, "="),
            (TokenType::FUNCTION, "fn"),
            (TokenType::LPAREN, "("),
            (TokenType::IDENT, "x"),
            (TokenType::COMMA, ","),
            (TokenType::IDENT, "y"),
            (TokenType::RPAREN, ")"),
            (TokenType::LBRACE, "{"),
            (TokenType::IDENT, "x"),
            (TokenType::PLUS, "+"),
            (TokenType::IDENT, "y"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::RBRACE, "}"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "result"),
            (TokenType::ASSIGN, "="),
            (TokenType::IDENT, "add"),
            (TokenType::LPAREN, "("),
            (TokenType::IDENT, "five"),
            (TokenType::COMMA, ","),
            (TokenType::IDENT, "ten"),
            (TokenType::RPAREN, ")"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::BANG, "!"),
            (TokenType::MINUS, "-"),
            (TokenType::SLASH, "/"),
            (TokenType::ASTERISK, "*"),
            (TokenType::INT, "5"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::INT, "5"),
            (TokenType::LT, "<"),
            (TokenType::INT, "10"),
            (TokenType::GT, ">"),
            (TokenType::INT, "5"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::IF, "if"),
            (TokenType::LPAREN, "("),
            (TokenType::INT, "5"),
            (TokenType::LT, "<"),
            (TokenType::INT, "10"),
            (TokenType::RPAREN, ")"),
            (TokenType::LBRACE, "{"),
            (TokenType::RETURN, "return"),
            (TokenType::TRUE, "true"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::RBRACE, "}"),
            (TokenType::ELSE, "else"),
            (TokenType::LBRACE, "{"),
            (TokenType::RETURN, "return"),
            (TokenType::FALSE, "false"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::RBRACE, "}"),
            (TokenType::INT, "10"),
            (TokenType::EQ, "=="),
            (TokenType::INT, "10"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::INT, "10"),
            (TokenType::NOT_EQ, "!="),
            (TokenType::INT, "9"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::EOF, "\0"),
        ];
        let mut l = Lexer::new(input);
        let mut result = "".to_string();
        for (token_type, lexeme) in tests {
            let tok = l.next_token();
            // println!("期望的type: {:?}, lexeme {}", token_type, lexeme);
            // println!("实际的type: {:?}, lexeme {}", tok.t, tok.ch);
            result.push_str(lexeme);
            println!("{:?}", result);
            assert_eq!(tok.t, token_type);
            assert_eq!(tok.ch, lexeme.to_string());
        }
    }
}
