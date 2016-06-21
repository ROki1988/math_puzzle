use std::collections::HashSet;

fn main() {
    let a: HashSet<_> = (0..10).map(|x| x.to_string().as_str().chars().nth(0).unwrap()).collect();    
    for num in (2..10000) {
        let sqrt = f64::from(num).sqrt();
        if a_12(sqrt, &a) {
            println!("{} {:.9}", num, sqrt);  
        }
        
    }
}

fn a_12(sqrt: f64, expected: &HashSet<char>) -> bool {
    let a: HashSet<_> = format!("{:.9}", sqrt).as_str().chars().clone().filter(|&x| x != '.').collect();
    expected.difference(&a).count() == 0
}