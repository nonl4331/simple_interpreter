use crate::literal::Literal;

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    Literal(Literal),
    Identifier(String),
    Fn,
    FnAssign,
    LeftBracket,
    RightBracket,
    Add,
    Hyphen,
    Multiply,
    Divide,
    Remainder,
    Equals,
    End,
}

/*impl Add for Token {
    type Output = Result<Self, String>;

    fn add(self, other: Self) -> Self::Output {
        if let Token::(Literal)
    }
}*/

pub fn tokenify(string: String) -> Vec<Token> {
    let string = string.trim();
    let mut tokens = Vec::new();

    if string.is_empty() {
        return tokens;
    }

    let chars = string.chars().enumerate();

    let mut is_string = false;

    let mut number_buffer = String::new();
    let mut ident_buffer = String::new();

    let parse_number = |tokens: &mut Vec<Token>, index: usize, buffer: &mut String| {
        if !buffer.is_empty() {
            match buffer.parse::<f64>() {
                Ok(n) => tokens.push(Token::Literal(Literal::Number(n))),
                Err(_) => {
                    panic!("Error parsing at {index}.");
                }
            }
            *buffer = String::new();
        }
    };

    let parse_ident = |tokens: &mut Vec<Token>, buffer: &mut String, is_string: bool| {
        if !buffer.is_empty() {
            match &buffer[..] {
                "fn" => tokens.push(Token::Fn),
                _ => {
                    if is_string {
                        tokens.push(Token::Literal(Literal::String(buffer.clone())))
                    } else {
                        tokens.push(Token::Identifier(buffer.clone()))
                    }
                }
            }

            *buffer = String::new();
        }
    };

    for (i, c) in chars {
        if c == '"' {
            parse_ident(&mut tokens, &mut ident_buffer, is_string);
            is_string = !is_string;
            continue;
        } else if is_string {
            ident_buffer.push(c);
            continue;
        }

        if ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || '_' == c {
            ident_buffer.push(c);
            continue;
        } else if ('0'..='9').contains(&c) || '.' == c {
            number_buffer.push(c);
            continue;
        }

        parse_number(&mut tokens, i, &mut number_buffer);
        parse_ident(&mut tokens, &mut ident_buffer, is_string);

        match c {
            '"' => {
                is_string = !is_string;
            }
            '>' => {
                if let Token::Equals = *tokens.last().unwrap() {
                    tokens.pop();
                    tokens.push(Token::FnAssign);
                } else {
                    panic!("Error parsing at {i}.");
                }
            }
            '=' => tokens.push(Token::Equals),
            '+' => tokens.push(Token::Add),
            '-' => tokens.push(Token::Hyphen),
            '*' => tokens.push(Token::Multiply),
            '/' => tokens.push(Token::Divide),
            '%' => tokens.push(Token::Remainder),
            ')' => {
                if let Token::LeftBracket = *tokens.last().unwrap() {
                    tokens.push(Token::Literal(Literal::Unit))
                } else {
                    tokens.push(Token::RightBracket)
                }
            }
            '(' => tokens.push(Token::LeftBracket),
            ' ' => continue,
            _ => panic!("Error parsing at {i}."),
        }
    }
    if is_string {
        panic!("Unmatched \"");
    }

    parse_number(&mut tokens, string.len(), &mut number_buffer);
    parse_ident(&mut tokens, &mut ident_buffer, is_string);

    tokens.reverse();

    tokens
}
