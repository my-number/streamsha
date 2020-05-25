#![feature(test)]
extern crate test;
use streamsha::traits::StreamHasher;
use streamsha::*;
use test::Bencher;

#[bench]
fn bench_hash_1000000_zeros(b: &mut Bencher) {
    b.iter(|| {
        let mut hasher = Sha256::new();
        for _ in 0..1000 {
            hasher.update(&[0; 1000]);
        }
        hasher.finish()
    });
}
#[bench]
fn bench_hash_0x20000000_z(b: &mut Bencher) {
    b.iter(|| {
        let mut hasher = Sha256::new();
        for _ in 0..0x100000 {
            hasher.update(&[0x5a; 0x200]);
        }
        hasher.finish()
    });
}
