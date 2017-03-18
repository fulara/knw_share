#![feature(test)]

extern crate test;

pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn main() {
    println!("hello world");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }

    #[bench]
    fn bench_xor_100_ints(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(100);
            (0..n).fold(0, |a, b| a ^ b)
        })
    }
}
