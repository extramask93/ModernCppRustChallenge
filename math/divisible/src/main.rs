use std::io::{self, Write};

fn divisible(limit: u32) -> u32{
    (0..limit).fold(0, |acc,x| {
        if x%3==0 || x%5==0 {return acc+x}
        else{return acc;}
    })
}
#[test]
fn divisible_test() {
    assert_eq!(divisible(6),8);
    assert_eq!(divisible(0),0);
}

fn main() {
    let mut x = String::new();
    print!("Please enter max limit to search in: ");
    io::stdout().flush().expect("Flush failed");
    io::stdin().read_line(&mut x).expect("Failed to read line");
    let limit:u32 = x.trim().parse().expect("Not an interger");
    let result = divisible(limit);
    println!("Result: {}" , result);

}
