use std::io::{self, BufRead};

fn parse_line() -> Option<String> {
    io::stdin().lock().lines().next()?.ok()
}

fn parse_operation(part: &String) -> Option<Box<dyn Fn(f64, f64) -> f64>> {
    if part.len() != 1 {
        return None;
    }
    match part.chars().next()? {
        '+' => Some(Box::new(|x, y| x + y)),
        '-' => Some(Box::new(|x, y| x - y)),
        '*' => Some(Box::new(|x, y| x * y)),
        '/' => Some(Box::new(|x, y| x / y)),
        _ => None,
    }
}

fn calculate_postfix(line: &str) -> Option<f64> {
    let mut stack: Vec<f64> = vec![];
    for part in line.split_whitespace() {
        if let Ok(num) = part.parse::<f64>() {
            stack.push(num);
        } else {
            let f = parse_operation(&part.to_string())?;
            let y = stack.pop()?;
            let x = stack.pop()?;
            let result = f(x, y);
            stack.push(result);
        }
    }
    return if stack.len() == 1 {
        stack.first().copied()
    } else {
        None
    };
}

fn main() {
    parse_line().map_or_else(
        || {
            println!("Could not parse line.");
        },
        |line| {
            calculate_postfix(&line).map_or_else(
                || println!("Could not calculate result from expression."),
                |result| println!("Result: {result}"),
            );
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_postfix() {
        let input = "2 2 +";
        assert_eq!(calculate_postfix(input), Some(4.0));

        let input = "2 2 * 2 * 2 * 2 *";
        assert_eq!(calculate_postfix(input), Some(32.0));
    }
}
