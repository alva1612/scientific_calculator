use std::num::ParseIntError;
use regex::Regex;
use regex::Captures;

fn capture_input(caps: Captures<'_>) -> Result<(&str ,i32, i32), ParseIntError> {

            let cap_expression = caps.get(0).unwrap().as_str();
            let left_value: i32 = match caps.get(1).unwrap().as_str().parse() {
                Ok(num) => num,
                Err(e) => {
                    return  Err(e);
                }
            };
            let rigth_value = match caps.get(2).unwrap().as_str().parse() {
                Ok(num) => num,
                Err(e) => {
                    return  Err(e);
                }
            };
            return Ok((cap_expression ,left_value, rigth_value))
}

fn main() {
    //REGEX
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_mult = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let re_div = Regex::new(r"(\d+)\s?/\s?(\d+)").unwrap();
    let re_diff = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();

    //DATA DEL USUARIO

    loop {
        
        let mut expression = String::new();
        let mut exists_error = false;
        println!("Ingrese la operación");
        std::io::stdin()
            .read_line(&mut expression)
            .expect("Couldn't read input");
        //DIVSION
        loop {
            //OPERACIONES
            let caps = re_div.captures(expression.as_str());

            if caps.is_none() {
                break;
            }

            let caps = caps.unwrap();

            let (cap_expression, left_value, right_value) = capture_input(caps).unwrap();
            let result = left_value / right_value;

            expression = expression.replace(cap_expression, &result.to_string());
        }
        //MULTIPLICATION
        loop {
            //OPERACIONES
            let caps = re_mult.captures(expression.as_str());

            if caps.is_none() {
                break;
            }

            let caps = caps.unwrap();
            let (cap_expression, left_value, right_value) = capture_input(caps).unwrap();
            let result = left_value * right_value;

            expression = expression.replace(cap_expression, &result.to_string());
        }
        //ADDITION
        loop {
            //OPERACIONES
            let caps = re_add.captures(expression.as_str());

            if caps.is_none() {
                break;
            }

            let caps = caps.unwrap();
            let (cap_expression, left_value, rigth_value) = capture_input(caps).unwrap();

            let addition = left_value + rigth_value;

            expression = expression.replace(cap_expression, &addition.to_string());
        }
        //DIFFERENCE
        loop {
            //OPERACIONES
            let caps = re_diff.captures(expression.as_str());

            if caps.is_none() {
                break;
            }

            let caps = caps.unwrap();
            let (cap_expression, left_value, rigth_value) = capture_input(caps).unwrap();

            let addition = left_value - rigth_value;

            expression = expression.replace(cap_expression, &addition.to_string());
        }
        //MOSTAR RESULTADO
        if exists_error {
            continue;
        }


        println!("Res: {}", expression);
    }
}

