use std::{io::{self, BufRead}};

fn parse_line() -> Option<String> {
    let mut iterator = io::stdin().lock().lines();
    let line = match iterator.next()? {
        Ok(x) => x,
        Err(_) => return None,
    };
    Some(line.to_string())
}

fn parse_operation(part: &String) -> Option<Box<dyn Fn(f64, f64) -> f64>> {
    if part.len() != 1 {
        return None
    } 
    return match part.chars().nth(0)? {
        '+' => Some(Box::new(|x, y| x + y)),
        '-' => Some(Box::new(|x, y| x - y)),
        '*' => Some(Box::new(|x, y| x * y)),
        '/' => Some(Box::new(|x, y| x / y)),
        _ => None
    }
}

fn calculate_postfix(line: &String) -> Option<f64> {
    let mut stack: Vec<f64> = vec![];
    for part in line.split_whitespace() {
        if let Ok(num) = part.parse::<i64>() {
            stack.push(num as f64);
        } else {
            let f = parse_operation(&part.to_string())?;
            let y = stack.pop()?;
            let x = stack.pop()?;
            let result = f(x, y);
            stack.push(result);
        }
    }
    stack.last().copied()
}

fn main() {
    if let Some(line) = parse_line() {
        match calculate_postfix(&line) {
            Some(result) => println!("Result: {}", result),
            None => println!("Could not calculate result from expression."),
        }
    } else {
        println!("Could not parse line.")
    }
}
