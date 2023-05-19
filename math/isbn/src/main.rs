use std::io::{self, read_to_string};
struct ISBN {
    isbn: Vec<u8>,
}
impl ISBN {
    fn new(sisbn: &[u8]) -> Self {
        let mut result : Vec<u8> = Vec::new();
        for c in sisbn {
            result.push(c - '0' as u8);
        }
        let a: &mut u8 = result.last_mut().unwrap();
        if *a == 'X' as u8  {
            *a = 10;
        }
        ISBN { isbn: result}
    }
    fn is_valid(&self)-> bool {
        let mut sum: u32 =0;
        for (&i,n) in self.isbn.iter().zip((1..11).rev()) {
            sum += n*(i as u32);
        }
        if self.isbn.len() == 10 && sum % 11 == 0 {
            return true;
        }
        return false;

    }
}
#[test]
fn isbn_valid() {
    let isbn_valid = ISBN::new("0306406152".as_bytes());
    let isbn_invalid = ISBN::new("0306406153".as_bytes());
    let isbn_invalid = ISBN::new("030640615X".as_bytes());
    assert_eq!(isbn_valid.is_valid(),true);
    assert_eq!(isbn_invalid.is_valid(),false);
}

fn main() {
    let mut isbn_string = String::new();
    io::stdin().read_line(&mut isbn_string).unwrap();
    let isbn = ISBN::new(isbn_string.trim().as_bytes());
    println!("Given isbn is {}",isbn.is_valid());
}
