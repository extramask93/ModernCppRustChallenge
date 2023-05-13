fn isprime(num: u32) -> bool
{
    if (num == 0) || (num ==1) {
        return false;
    }
    //brute force for now (and forever, probably xd)
    let result = (2..num).find(|n| {if num % n ==0 {return true;}else {return false;}});
    match result {
        Some(_) => false,
        None => true
    }
}
fn findSexyPrimes(num: u32) -> Vec<(u32,u32)>{
    let primes:Vec<_> = (0..num).filter(|n| isprime(*n)).collect();
    let mut result = Vec::<(u32,u32)>::new();
    for &item in primes.iter() {
        if primes.contains(&(item+6)){
            result.push((item,item+6))
        }
    }
    result
}
#[test]
fn sexyPrime_test(){
    let sp = findSexyPrimes(12);
    assert_eq!(sp.len(), 1);
    assert_eq!(sp.first().unwrap(), &(5,11));
}
fn main() {
    println!("Hello, world!");
}
