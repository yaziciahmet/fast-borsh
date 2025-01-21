use borsh::BorshSerialize;
use fastborsh::{BorshSize, FastBorshSerialize};

#[derive(Default, BorshSerialize, BorshSize)]
struct Stack {
    a: u32,
    b: u64,
    c: [u8; 32],
    d: Vec<u8>,
}

#[test]
fn t() {
    let mut s = Stack::default();
    s.d = vec![5; 900];
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
    println!("elapsed: {}", start.elapsed().as_micros());

    let start = std::time::Instant::now();
    for _ in 0..n {
        borsh::to_vec(&s).unwrap();
    }
    println!("elapsed: {}", start.elapsed().as_micros());
}
