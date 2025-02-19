use borsh::BorshSerialize;
use fastborsh::{BorshSize, FastBorshSerialize};

#[derive(Default, BorshSerialize, BorshSize)]
struct Struck {
    a: u32,
    b: u64,
    c: [u8; 32],
    d: Vec<u8>,
    e: Vec<Option<[u8; 32]>>,
}

#[test]
fn t() {
    let mut s = Struck::default();
    s.d = vec![5; 900];
    s.e = vec![None; 2000];
    let r1 = borsh::to_vec(&s).unwrap();
    let r2 = s.fast_serialize();
    assert_eq!(r1, r2);
    dbg!(r1.len());
    dbg!(r1.capacity());
    dbg!(r2.len());
    dbg!(r2.capacity());

    let n = 100000;

    let start = std::time::Instant::now();
    for _ in 0..n {
        s.fast_serialize();
    }
    println!("elapsed fast: {}", start.elapsed().as_micros());

    let start = std::time::Instant::now();
    for _ in 0..n {
        borsh::to_vec(&s).unwrap();
    }
    println!("elapsed borsh: {}", start.elapsed().as_micros());
}
