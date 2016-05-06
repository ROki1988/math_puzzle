fn main() {
    for x in 10.. {
        let bin_str = format!("{:b}", x);
        let oct_str = format!("{:o}", x);
        let dig_str = format!("{}", x);
        
        if is_palindrome(bin_str.as_str()) & is_palindrome(oct_str.as_str()) & is_palindrome(dig_str.as_str()) {
            println!("{}, {}, {}", &bin_str, &oct_str, &dig_str);  
            break;      
        }
    }
}

fn is_palindrome(input_str: &str) -> bool {
    let reverse_string: String = input_str.chars().rev().collect();
    input_str == reverse_string.as_str()
}