fn main() {
    println!("Q4_1: {}", cut_bar(20, 3, 1));
    println!("Q4_2: {}", cut_bar(100, 5, 1));
}


fn cut_bar(bar_len: u32, scissors: u32, bar_num: u32) -> u32 {
    if bar_num >= bar_len {
        return 0;
    }
    
    if bar_num < scissors {
        1 + cut_bar(bar_len, scissors, bar_num * 2)
    }
    else {
        1 + cut_bar(bar_len, scissors, bar_num + scissors)        
    }
}