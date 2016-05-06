use std::str::FromStr;

fn main() {
    for x in 1000..9999 {
        if q2_exp(x) == reverse_int(x) {
            println!("{}", x);
            break;
        }
    }
}

fn q2_exp(input: u32) -> u32 {
    let input_str = input.to_string();
    
    let (stack, base) = input_str.split_at(input_str.len() - 2);
        
    stack.chars()
        .map(|x| x.to_digit(10))
        .flat_map(|y| y)
        .fold(1, |acc, z| acc * z) * u32::from_str(base).unwrap()
}

fn reverse_int(input: u32) -> u32 {
    let reverse_str: String = input.to_string().as_str().chars().rev().collect();
    reverse_str.parse().unwrap()
}