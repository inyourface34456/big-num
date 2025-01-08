use std::error::Error;
use std::fmt::Display;

use crate::{Num, impl_try_from_int, impl_try_from_u};

#[derive(Debug, Eq, PartialEq)]
pub enum Errors {
    ToBig,
}

impl Error for Errors {
    fn description(&self) -> &str {
        match self {
            Errors::ToBig => "The esleted value was to big for target",
        }
    }
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.to_string())?;

        Ok(())
    }
}

impl_try_from_int!(i128);
impl_try_from_int!(i64);
impl_try_from_int!(i32);
impl_try_from_int!(i16);
impl_try_from_int!(i8);
impl_try_from_int!(isize);

impl_try_from_u!(u128);
impl_try_from_u!(u64);
impl_try_from_u!(u32);
impl_try_from_u!(u16);
impl_try_from_u!(u8);
impl_try_from_u!(usize);