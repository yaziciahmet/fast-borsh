#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use borsh::BorshSerialize;

pub trait FastSerialize: BorshSerialize {
    const SIZE: usize;

    fn fast_serialize(&self) -> [u8; Self::SIZE] {
        let mut buf = [0u8; Self::SIZE];
        let mut writer = buf.as_mut_slice();
        BorshSerialize::serialize(self, &mut writer).expect("Serialization must not fail");
        buf
    }
}

impl FastSerialize for u32 {
    const SIZE: usize = 4;
}

impl FastSerialize for u64 {
    const SIZE: usize = 8;
}

impl FastSerialize for [u8; 32] {
    const SIZE: usize = 32;
}
