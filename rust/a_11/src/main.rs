fn main() {
    let mut memo: Vec<u64> = vec![];
    let result: Vec<u64> = (0..).map(|x| fib_memo(x, &mut memo))
                                .filter(|&x| (x > 144) & ans(x))
                                .take(5)
                                .collect();
    
    println!("{:?}", result);
}

fn fib_memo(n: usize, memo: &mut Vec<u64>) -> u64 {
    if let Some(x) = memo.clone().get(n) {
        *x
    }
    else {
        let result = fib(n, memo);
        memo.push(result);
        result
    }
}

fn fib(n: usize, memo: &Vec<u64>) -> u64 {
    match n {
        0...1 => {
            1
        },
        _ => {
            let mut current = memo.clone();
            fib_memo(n - 2, &mut current) + fib_memo(n - 1, &mut current)
        }
    }    
}

fn ans(n: u64) -> bool {
    n % right_exp(n) == 0
}

fn right_exp(n: u64) -> u64 {
    n.to_string()
        .as_str()
        .chars()
        .map(|x| x.to_digit(10))
        .flat_map(|x| x)
        .fold(0, |sum, x| sum + x) as u64
}