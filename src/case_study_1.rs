/*

Simple functionality of rust:
	- reading arguments
	- expect / unwrap
	- borrowing reference
	- reading value of reference
	- iterators

*/

use std::io::Write;
use std::str::FromStr;

fn gdc(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n
    }
    n
}

pub fn f1 () {
	let mut numbers = Vec::new();

    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gdc NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    
    for m in &numbers[1..] {
        d = gdc(d, *m);
    }

    println!("Greatest common divisor of {:?} is {}", numbers, d);
}



#[test]
fn test_gdc() {
    assert_eq!(gdc(14, 15), 1);
    assert_eq!(gdc(2 * 3 * 7 * 11, 3 * 7 * 11 * 15), 3 * 7 * 11);
}