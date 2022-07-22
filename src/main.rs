use std::env::{args, Args};

fn main() {
    let mut args: Args = args();
    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first_num: f32 = first.parse::<f32>().unwrap();
    let second_num: f32 = second.parse::<f32>().unwrap();

    let result = operate(operator, first_num, second_num);
    println!("{} {} {} is {}", first_num, operator, second_num, result);
}

fn operate(operator: char, a: f32, b:f32) -> f32 {
    if operator == '+' {
        a + b
    } else if operator == '-' {
        a - b
    } else if operator == '*' {
        a*b
    } else if operator == '/' {
        a/b
    } else {
        0.0
    }
}
