fn main() {
    let mut memo: Vec<u64> = vec![];
    let mut log: Vec<u64> = vec![];
    
    for x in (0..) {
        let v = fib_memo(x, &mut memo);
        if (v > 144) & ans(v) {
            log.push(v)
        }
        
        if log.len() == 5 {
            break;
        }
    }
    println!("{:?}", log);
}

fn fib_memo(n: usize, memo: &mut Vec<u64>) -> u64 {
    if let Some(x) = memo.clone().get(n) {
        *x
    }
    else {
        let result = match n {
            0...1 => {
                1
            },
            _ => {
                fib_memo(n - 2, &mut memo.clone()) + fib_memo(n - 1, &mut memo.clone())
            }
        };
        memo.push(result);
        result
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