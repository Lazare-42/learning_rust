extern crate num;
use num::Complex;

//Try to determine if 'c' is in the Mandelbrot set, using at most 'limit'
//iterations to decide

/// If 'c' is not a member, return 'Some(i)', where 'i' is the number of 
/// iterations it took for 'c' to leave the circle of radiustwo centered on 
/// the origin. If 'c' seems to be a member (more precisely, if we reached the
/// iteration limit without being able to prove that 'c' is not a member),
/// return 'None'.
fn  escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0};
    for i in 0..limit {
        z = z*z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}

use std::str::FromStr;

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

fn main() {
    let c= Complex {re : 10, im : 5};
    let c2 = Complex{re : 10.4, im : 5.3};
    println!("Hello, world! {:?} {:?}", c, c2 );
}

#[test]
fn test_parse_pair() {
 assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
 assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}
