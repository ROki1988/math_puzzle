fn main() {
    let mut path: Vec<(i32, i32)> = vec![(0, 0)];
    
    println!("{}", cleaner(12, &mut path));
}

fn cleaner(max: usize, path: &mut Vec<(i32, i32)>) -> u32 {
    if path.len() == max + 1 {
        return 1
    }
    
    let mut count = 0;
    for pair in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
        if let Some(x) = path.last() {
            let next_path = (x.0 + pair.0, x.1 + pair.1);
            if !path.contains(&next_path) {
                let mut current_path = path.clone();
                // 代替え手段があれば、mut を外せるのに
                current_path.push(next_path);
                count = count + cleaner(max, &mut current_path);
            }
        }
    }
    count
}
