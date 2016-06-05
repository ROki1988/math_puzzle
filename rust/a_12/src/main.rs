fn main() {
    let sqrts = (2..).map(|x| f64::from(x).sqrt());
    
    for num in sqrts.into_iter().filter(|&x| x < 10.0) {
        println!("{}", num);
    }
    
}