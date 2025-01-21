use borsh::BorshSerialize;

pub trait FastBorshSerialize: BorshSerialize + BorshSize {
    fn fast_serialize(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.borsh_size());
        self.serialize(&mut buf).expect("Serialization must not fail");
        buf
    }
}

impl<T> FastBorshSerialize for T
where
    T: BorshSerialize + BorshSize
{
}

pub trait BorshSize {
    fn borsh_size(&self) -> usize;
}

impl BorshSize for u8 {
    #[inline(always)]
    fn borsh_size(&self) -> usize {
        1
    }
}

impl BorshSize for u16 {
    #[inline(always)]
    fn borsh_size(&self) -> usize {
        2
    }
}

impl BorshSize for u32 {
    #[inline(always)]
    fn borsh_size(&self) -> usize {
        4
    }
}

impl BorshSize for u64 {
    #[inline(always)]
    fn borsh_size(&self) -> usize {
        8
    }
}

impl BorshSize for [u8; 32] {
    #[inline(always)]
    fn borsh_size(&self) -> usize {
        32
    }
}

impl BorshSize for Vec<u8> {
    #[inline(always)]
    fn borsh_size(&self) -> usize {
        4 + self.len()
    }
}

impl<'a> BorshSize for &'a [u8] {
    #[inline(always)]
    fn borsh_size(&self) -> usize {
        4 + self.len()
    }
}
