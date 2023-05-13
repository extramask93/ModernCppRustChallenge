fn isprime(num: u32) -> bool
{
    if (num == 0) || (num ==1) {
        return true;
    }
    //brute force for now (and forever, probably xd)
    let result = (2..num).find(|n| {if num % n ==0 {return true;}else {return false;}});
    match result {
        Some(_) => false,
        None => true
    }

}
fn findMaxPrime(num: u32) -> u32{
    let result = (num..0).find(|n| isprime(*n)).unwrap();
    result
}
#[test]
fn isprime_test() {
    assert_eq!(isprime(1),true);
    assert_eq!(isprime(0),true);
    assert_eq!(isprime(3),true);
}
#[test]
fn findMaxPrime_test() {
    assert_eq!(findMaxPrime(0),0);
    assert_eq!(findMaxPrime(1),1);
    assert_eq!(findMaxPrime(3),3);
    assert_eq!(findMaxPrime(8),7);
}

fn main() {
    println!("Hello, world!");
}
