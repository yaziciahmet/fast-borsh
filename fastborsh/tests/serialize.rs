use borsh::BorshSerialize;
use fastborsh::FastSerialize;

#[derive(BorshSerialize, Default, FastSerialize)]
struct Stack {
    a: u32,
    b: u64,
    c: [u8; 32],
}

// impl FastSerialize for Stack {
//     const SIZE: usize = u32::SIZE + u64::SIZE + <[u8; 32] as FastSerialize>::SIZE;
// }

#[test]
fn t() {
    let s = Stack::default();
    let r1 = borsh::to_vec(&s).unwrap();
    let r2 = s.fast_serialize();
    assert_eq!(r1, r2.to_vec());

    let n = 100000;

    let start = std::time::Instant::now();
    for _ in 0..n {
        borsh::to_vec(&s).unwrap();
    }
    println!("elapsed: {}", start.elapsed().as_micros());

    let start = std::time::Instant::now();
    for _ in 0..n {
        s.fast_serialize();
    }
    println!("elapsed: {}", start.elapsed().as_micros());
}
