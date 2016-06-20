fn main() {
    let sqrts = (2..100).map(|x| f64::from(x).sqrt());
    
    for num in sqrts.into_iter() {
        println!("{}", num);
    }
    
}