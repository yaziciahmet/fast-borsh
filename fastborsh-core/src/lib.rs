use std::collections::VecDeque;

use borsh::BorshSerialize;

pub trait FastBorshSerialize: BorshSerialize + BorshSize {
    fn fast_serialize(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.borsh_size());
        self.serialize(&mut buf)
            .expect("Serialization must not fail");
        buf
    }
}

impl<T> FastBorshSerialize for T where T: BorshSerialize + BorshSize {}

pub trait BorshSize {
    fn borsh_size(&self) -> usize;
}

macro_rules! impl_const_borsh_size {
    ($ty:ty, $size:expr) => {
        impl BorshSize for $ty {
            #[inline(always)]
            fn borsh_size(&self) -> usize {
                $size
            }
        }
    };
}

impl_const_borsh_size!(usize, 8);
impl_const_borsh_size!(u8, 1);
impl_const_borsh_size!(u16, 2);
impl_const_borsh_size!(u32, 4);
impl_const_borsh_size!(u64, 8);
impl_const_borsh_size!(u128, 16);

impl_const_borsh_size!(isize, 8);
impl_const_borsh_size!(i8, 1);
impl_const_borsh_size!(i16, 2);
impl_const_borsh_size!(i32, 4);
impl_const_borsh_size!(i64, 8);
impl_const_borsh_size!(i128, 16);

impl_const_borsh_size!(f32, 4);
impl_const_borsh_size!(f64, 8);

impl<T: BorshSize> BorshSize for Vec<T> {
    #[inline(always)]
    fn borsh_size(&self) -> usize {
        self.as_slice().borsh_size()
    }
}

impl<T: BorshSize> BorshSize for VecDeque<T> {
    #[inline(always)]
    fn borsh_size(&self) -> usize {
        let (s1, s2) = self.as_slices();
        s1.borsh_size() + s2.borsh_size()
    }
}

impl<T: BorshSize> BorshSize for [T] {
    #[inline(always)]
    fn borsh_size(&self) -> usize {
        let mut size = 4;
        for el in self.iter() {
            size += el.borsh_size();
        }
        size
    }
}

impl<T: BorshSize + ?Sized> BorshSize for &T {
    #[inline(always)]
    fn borsh_size(&self) -> usize {
        (*self).borsh_size()
    }
}

impl<T: BorshSize, const N: usize> BorshSize for [T; N] {
    #[inline(always)]
    fn borsh_size(&self) -> usize {
        if N == 0 {
            0
        } else {
            let mut size = 0;
            for el in self.iter() {
                size += el.borsh_size();
            }
            size
        }
    }
}

impl<T: BorshSize> BorshSize for Option<T> {
    #[inline(always)]
    fn borsh_size(&self) -> usize {
        let inner_size = match self {
            Some(v) => v.borsh_size(),
            None => 0,
        };
        1 + inner_size
    }
}

impl<T: BorshSize, E: BorshSize> BorshSize for Result<T, E> {
    #[inline(always)]
    fn borsh_size(&self) -> usize {
        let inner_size = match self {
            Ok(v) => v.borsh_size(),
            Err(e) => e.borsh_size(),
        };
        1 + inner_size
    }
}
