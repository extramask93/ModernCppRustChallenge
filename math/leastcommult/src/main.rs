fn leastComMult(num1: u32, num2: u32)-> u32
{
    let mut least = num1*num2;
    if (least == 0){
        return least;
    }
    least = (2..least).find(|num| if (num1%num==0) && (num2%num==0) {return true} else {return false}).unwrap_or(least);
    least
}
// 24 16
// from 1 to max 24*16 check if first and second % i == 0 
// if so, then break early
// what about 0 and 0 should return 0
//
#[test]
fn leastComMult_test() {
    assert_eq!(leastComMult(0,0),0);
    assert_eq!(leastComMult(0,1),0);
    assert_eq!(leastComMult(1,1),1);
    assert_eq!(leastComMult(3,6),3);
    assert_eq!(leastComMult(7,8),7*8);
}
fn main() {
    println!("Hello, world!");
}
