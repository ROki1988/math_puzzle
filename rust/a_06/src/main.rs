fn main() {
    let result = (2..10000).filter(|x| x % 2 == 0)
                            .filter(|y| collatz_loop(*y, *y * 3 + 1));
    
    println!("ans = {}", result.count());
}

fn collatz(n: u32) -> u32 {
    match n % 2 == 0 {
        false => n * 3 + 1,
        true => n / 2
    }
}

fn collatz_loop(target: u32, n: u32) -> bool {
    if n == 1 {
        false
    }
    else if n == target {
        true
    }
    else {
        collatz_loop(target, collatz(n))
    }
}