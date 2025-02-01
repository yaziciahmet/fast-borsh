use borsh::BorshSerialize;
use divan::{black_box, Bencher};
use fastborsh::{BorshSize, FastBorshSerialize};

#[derive(Default, BorshSerialize, BorshSize)]
struct Struck {
    a: u32,
    b: u64,
    c: [u8; 32],
    d: Vec<u8>,
}

#[divan::bench(args = 800..=1100, min_time = 0.2)]
fn fast(bencher: Bencher, n: usize) {
    let mut s = Struck::default();
    s.d = vec![5; n];

    bencher.bench(|| {
        black_box(s.fast_serialize());
    });
}

#[divan::bench(args = 800..=1100, min_time = 0.2)]
fn borsh(bencher: Bencher, n: usize) {
    let mut s = Struck::default();
    s.d = vec![5; n];

    bencher.bench(|| {
        borsh::to_vec(&s).unwrap();
    });
}

fn main() {
    // Run registered benchmarks.
    divan::main();
}
