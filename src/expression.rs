use crate::literal::Literal;
use crate::token::Token;
use std::collections::HashMap;

pub struct Expression<'a> {
    tokens: Vec<Token>,
    variables: &'a mut HashMap<String, Literal>,
    arguments: &'a mut HashMap<String, Literal>,
    functions: &'a mut HashMap<String, (Vec<String>, Vec<Token>)>,
}

impl<'a> Expression<'a> {
    pub fn from_tokens(
        tokens: Vec<Token>,
        variables: &'a mut HashMap<String, Literal>,
        arguments: &'a mut HashMap<String, Literal>,
        functions: &'a mut HashMap<String, (Vec<String>, Vec<Token>)>,
    ) -> Self {
        Self {
            tokens,
            variables,
            arguments,
            functions,
        }
    }

    fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::End)
    }

    fn peek_last(&mut self) -> Token {
        self.tokens.last().cloned().unwrap_or(Token::End)
    }
}

pub fn evaluate_expression(expression: &mut Expression) -> Result<Literal, String> {
    match expression.peek_last() {
        Token::Literal(_) | Token::Hyphen | Token::Identifier(_) | Token::LeftBracket => {
            expr(expression, 0)
        }
        Token::Fn => {
            if expression.tokens.len() < 4 {
                return Err("Invalid Function Definition!".to_string());
            }

            expression.tokens.pop();

            let function_name = match expression.tokens.pop().unwrap() {
                Token::Identifier(fn_name) => {
                    if expression.variables.get(&fn_name).is_some() {
                        return Err("Variable with same name as function exists!".to_string());
                    }
                    fn_name
                }
                _ => return Err("Invalid Function Definition!".to_string()),
            };

            let mut args = Vec::new();
            while let Some(tok) = expression.tokens.pop() {
                match tok {
                    Token::Identifier(var) => {
                        args.push(var);
                    }
                    Token::FnAssign => {
                        break;
                    }
                    _ => todo!(),
                }
            }

            let expr = expression.tokens.clone();

            for tok in expr.iter() {
                if let Token::Identifier(ident) = tok {
                    if !args.contains(ident) {
                        return Err("Non local variable in function".to_string());
                    }
                }
            }

            expression.functions.insert(function_name, (args, expr));

            Ok(Literal::Unit)
        }
        _ => Err("Invalid".to_string()),
    }
}

fn process_function(
    expression: &mut Expression,
    args: Vec<String>,
    mut some_expr: Vec<Token>,
) -> Result<Literal, String> {
    if expression.tokens.len() < args.len() {
        return Err("Not enough arguments for function".to_string());
    }

    let mut new_args = HashMap::new();

    for arg in &args {
        let tok = expression.next();

        let tok_val = match tok {
            Token::Literal(v) => v,
            Token::Identifier(i) => {
                if let Some((args, some_expr)) = expression.functions.get(&i) {
                    process_function(expression, args.clone(), some_expr.clone())?
                } else if let Some(lit) = expression.variables.get(&i) {
                    lit.clone()
                } else {
                    return Err("Unknown identifier".to_string());
                }
            }
            _ => return Err("Invalid argument".to_string()),
        };
        new_args.insert(arg.to_string(), tok_val);
    }
    std::mem::swap(&mut new_args, expression.arguments);
    std::mem::swap(&mut some_expr, &mut expression.tokens);

    let res = expr(expression, 0);
    expression.tokens = some_expr;
    *expression.arguments = new_args;

    res
}

fn expr(expression: &mut Expression, min_bp: u32) -> Result<Literal, String> {
    let mut lhs = match expression.next() {
        Token::Literal(v) => v,
        Token::Identifier(i) => {
            if let Some(val) = expression.arguments.get(&i) {
                val.clone()
            } else if let Some(val) = expression.variables.get(&i) {
                val.clone()
            } else if let Some((args, some_expr)) = expression.functions.get(&i) {
                process_function(expression, args.clone(), some_expr.clone())?
            } else {
                return Err("Unknown identifier".to_string());
            }
        }
        Token::Hyphen => (-expr(expression, 3)?)?,
        Token::LeftBracket => {
            let lhs = expr(expression, 0)?;
            if expression.next() != Token::RightBracket {
                return Err("Missing closing bracket!".to_string());
            }
            lhs
        }
        _ => return Err("Invalid lhs token".to_string()),
    };

    loop {
        let peek_last = expression.peek_last();
        match peek_last {
            Token::End | Token::RightBracket => break,
            Token::Multiply => {
                let (l_bp, r_bp) = (2, 2);

                if l_bp < min_bp {
                    break;
                }

                expression.next();

                let rhs = expr(expression, r_bp)?;

                lhs = (lhs * rhs)?;
            }
            Token::Divide => {
                let (l_bp, r_bp) = (2, 2);

                if l_bp < min_bp {
                    break;
                }

                expression.next();

                let rhs = expr(expression, r_bp)?;

                lhs = (lhs / rhs)?;
            }
            Token::Add => {
                let (l_bp, r_bp) = (1, 1);

                if l_bp < min_bp {
                    break;
                }

                expression.next();

                let rhs = expr(expression, r_bp)?;
                lhs = (lhs + rhs)?;
            }
            Token::Hyphen => {
                let (l_bp, r_bp) = (1, 2);

                if l_bp < min_bp {
                    break;
                }

                expression.next();

                let rhs = expr(expression, r_bp)?;
                lhs = (lhs - rhs)?;
            }
            _ => return Err("Invalid token".to_string()),
        }
    }

    Ok(lhs)
}
