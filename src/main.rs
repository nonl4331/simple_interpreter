use crate::expression::evaluate_expression;
use crate::expression::Expression;
use crate::literal::Literal;
use crate::token::tokenify;
use std::collections::HashMap;

mod expression;
pub(crate) mod literal;
pub(crate) mod token;

fn main() {
    let mut function_map = HashMap::new();

    let input = std::io::stdin();

    let mut lines = input.lines();
    while let Some(Ok(line)) = lines.next() {
        let out = evaluate_expression(&mut Expression::from_tokens(
            tokenify(&line),
            &mut HashMap::new(),
            &mut HashMap::new(),
            &mut function_map,
        ))
        .map(|v| {
            if v == Literal::Unit {
                String::new()
            } else {
                format!("{v:?}\n")
            }
        })
        .unwrap_or("Invalid input".to_string());
        print!("{out}");
    }
}

#[cfg(test)]
mod tests {
    use crate::literal::Literal;

    use super::*;

    #[test]
    fn functions() {
        let mut function_map = HashMap::new();

        evaluate_expression(&mut Expression::from_tokens(
            tokenify("fn test_function a b c => -a + 5 * c"),
            &mut HashMap::new(),
            &mut HashMap::new(),
            &mut function_map,
        ))
        .unwrap();

        evaluate_expression(&mut Expression::from_tokens(
            tokenify("fn test_function_two a b  => a + 2 * b"),
            &mut HashMap::new(),
            &mut HashMap::new(),
            &mut function_map,
        ))
        .unwrap();

        let out = evaluate_expression(&mut Expression::from_tokens(
            tokenify(
                "((1 + 2) -  1) * test_function test_function_two 1 3 4 7 + 3 - -(-(-(-(-(1.5)))))",
            ),
            &mut HashMap::new(),
            &mut HashMap::new(),
            &mut function_map,
        ))
        .unwrap();

        evaluate_expression(&mut Expression::from_tokens(
            tokenify("\"Hello \" + \"World!\""),
            &mut HashMap::new(),
            &mut HashMap::new(),
            &mut function_map,
        ))
        .unwrap();

        assert_eq!(Literal::Number(60.5), out);
    }
}
