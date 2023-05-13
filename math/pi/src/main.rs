use rand::distributions::{Distribution, Uniform};
use num_traits::{Float, NumCast};

struct Point<T> {
    x: T,
    y: T,
}
fn isPointInCircle<T: Float>(point: &Point<T>) -> bool {
    //point on a cicrle is defined by x2+y2=1
    //point is in a circle if x2+y2<= 1
    //so if y2<=1-x2
    let one : T = NumCast::from(1).unwrap();
    if point.y.powi(2) <= one - point.x.powi(2){
        return true;
    }
    false

}

fn calcMonteCarloPI<K: Float, T: Distribution<K>>(between: &T,nrOfSamples:u32) -> K {
    let mut hits :u32= 0;
    let mut rng = rand::thread_rng();
    for _ in 0..nrOfSamples {
        let point: Point<K> = Point {
        x : between.sample(&mut rng),
        y : between.sample(&mut rng), 
        };
        if true==isPointInCircle(&point) {
            hits+=1;
        }
    }
    //if we divide the formula for square by the formula for circle we get
    //Ps/Pc = 4/pi
    //pi = 4Pc/Ps
    let four : K = NumCast::from(4.0).unwrap();
    let circleArea: K = NumCast::from(hits).unwrap();
    let squareArea: K =NumCast::from(nrOfSamples).unwrap(); 

    let  numerator : K = four * circleArea ;
    let pi = numerator / squareArea ;
    pi
    
}
fn main() {
    let between = Uniform::from(0.0..=1.0);
    println!("{}",calcMonteCarloPI(&between,100000));
}
