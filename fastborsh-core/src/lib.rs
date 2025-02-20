use std::{
    borrow::Cow,
    cell::{Cell, RefCell},
    collections::{BTreeMap, VecDeque},
    marker::PhantomData,
};

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

impl_const_borsh_size!(bool, 1);

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

impl BorshSize for str {
    #[inline(always)]
    fn borsh_size(&self) -> usize {
        self.as_bytes().borsh_size()
    }
}

impl BorshSize for String {
    #[inline(always)]
    fn borsh_size(&self) -> usize {
        self.as_bytes().borsh_size()
    }
}

impl<K: BorshSize, V: BorshSize> BorshSize for BTreeMap<K, V> {
    fn borsh_size(&self) -> usize {
        let mut size = 4;
        for (k, v) in self.iter() {
            size += k.borsh_size();
            size += v.borsh_size();
        }
        size
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

impl<T: BorshSize + ?Sized> BorshSize for Box<T> {
    #[inline(always)]
    fn borsh_size(&self) -> usize {
        self.as_ref().borsh_size()
    }
}

impl<T: BorshSize + ToOwned + ?Sized> BorshSize for Cow<'_, T> {
    #[inline(always)]
    fn borsh_size(&self) -> usize {
        self.as_ref().borsh_size()
    }
}

impl<T: BorshSize + Copy> BorshSize for Cell<T> {
    #[inline(always)]
    fn borsh_size(&self) -> usize {
        self.get().borsh_size()
    }
}

impl<T: BorshSize + Sized> BorshSize for RefCell<T> {
    #[inline(always)]
    fn borsh_size(&self) -> usize {
        self.try_borrow()
            .expect("RefCell already borrowed mutably while trying to serialize")
            .borsh_size()
    }
}

#[cfg(feature = "rc")]
impl<T: BorshSize + ?Sized> BorshSize for std::rc::Rc<T> {
    #[inline(always)]
    fn borsh_size(&self) -> usize {
        (**self).borsh_size()
    }
}

#[cfg(feature = "rc")]
impl<T: BorshSize + ?Sized> BorshSize for std::sync::Arc<T> {
    #[inline(always)]
    fn borsh_size(&self) -> usize {
        (**self).borsh_size()
    }
}

impl<T: ?Sized> BorshSize for PhantomData<T> {
    #[inline(always)]
    fn borsh_size(&self) -> usize {
        0
    }
}

macro_rules! impl_tuple_borsh_size {
    (@unit $name:ty) => {
        impl BorshSize for $name {
            #[inline(always)]
            fn borsh_size(&self) -> usize {
                0
            }
        }
    };

    ($($idx:tt $name:ident)+) => {
        impl<$($name),+> BorshSize for ($($name,)+)
        where $($name: BorshSize,)+
        {
            #[inline(always)]
            fn borsh_size(&self) -> usize {
                $(self.$idx.borsh_size() +)+ 0
            }
        }
    };
}

// Implement for unit types
impl_tuple_borsh_size!(@unit ());
impl_tuple_borsh_size!(@unit core::ops::RangeFull);

// Implement for tuples of different sizes
impl_tuple_borsh_size!(0 T0);
impl_tuple_borsh_size!(0 T0 1 T1);
impl_tuple_borsh_size!(0 T0 1 T1 2 T2);
impl_tuple_borsh_size!(0 T0 1 T1 2 T2 3 T3);
impl_tuple_borsh_size!(0 T0 1 T1 2 T2 3 T3 4 T4);
impl_tuple_borsh_size!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5);
impl_tuple_borsh_size!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6);
impl_tuple_borsh_size!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7);
impl_tuple_borsh_size!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8);
impl_tuple_borsh_size!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9);
impl_tuple_borsh_size!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10);
impl_tuple_borsh_size!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11);
impl_tuple_borsh_size!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12);
impl_tuple_borsh_size!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13);
impl_tuple_borsh_size!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14);
impl_tuple_borsh_size!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15);
impl_tuple_borsh_size!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16);
impl_tuple_borsh_size!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17);
impl_tuple_borsh_size!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18);
impl_tuple_borsh_size!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19);
