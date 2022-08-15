use crate::expression::evaluate_expression;
use crate::expression::Expression;
use crate::token::tokenify;
use std::collections::HashMap;

mod expression;
pub(crate) mod literal;
pub(crate) mod token;

fn main() {
    let mut function_map = HashMap::new();

    evaluate_expression(&mut Expression::from_tokens(
        tokenify("fn test_function a b c => -a + 5 * c".to_string()),
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut function_map,
    ))
    .unwrap();

    evaluate_expression(&mut Expression::from_tokens(
        tokenify("fn test_function_two a b  => a + 2 * b".to_string()),
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut function_map,
    ))
    .unwrap();

    let a = evaluate_expression(&mut Expression::from_tokens(
        tokenify(
            "((1 + 2) -  1) * test_function test_function_two 1 3 4 7 + 3 - -(-(-(-(-(1.5)))))" //60.5
                .to_string(),
        ),
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut function_map,
    ))
    .unwrap();

    println!("{}", a);

    let b = evaluate_expression(&mut Expression::from_tokens(
        tokenify("\"Hello \" + \"World!\"".to_string()),
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut function_map,
    ))
    .unwrap();

    println!("{}", b);
}
