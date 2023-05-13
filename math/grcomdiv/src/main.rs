fn grcomdiv(num1: u32,num2 : u32)->u32 {
    let max = std::cmp::max(num1, num2);
    let mut divisors:Vec<u32> = (1..max+1).
        filter(|n| if (n % num1 == 0) && (n % num2 ==0){ return true;} else{ return false})
        .collect();
    divisors.sort();
    let divisor = divisors.first().unwrap_or(&1).clone();
    divisor
}
#[test]
fn grcomdiv_test() {
    assert_eq!(grcomdiv(6,3),6);
    assert_eq!(grcomdiv(7,3),1);
}
fn main() {
    let mut dig1Str = String::new();
    let mut dig2Str = String::new();
    std::io::stdin().read_line(&mut dig1Str);
    std::io::stdin().read_line(&mut dig2Str);
    let dig1 = dig1Str.parse::<u32>().unwrap();
    let dig2 = dig2Str.parse::<u32>().unwrap();
    let a = grcomdiv(dig1,dig2);
    println!("{}",a);
}
