use crate::Num;
use crate::{impl_from_int, impl_from_u};

impl_from_int!(i128);
impl_from_int!(i64);
impl_from_int!(i32);
impl_from_int!(i16);
impl_from_int!(i8);
impl_from_int!(isize);

impl_from_u!(u128);
impl_from_u!(u64);
impl_from_u!(u32);
impl_from_u!(u16);
impl_from_u!(u8);
impl_from_u!(usize);