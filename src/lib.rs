pub type Day = i8;

#[derive(Debug, PartialEq)]
pub enum Solution {
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISIZE(isize),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USIZE(usize),
    Str(String),
}

use std::fmt::{Display, Formatter, Result};
impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Solution::I16(x) => x.fmt(f),
            Solution::I32(x) => x.fmt(f),
            Solution::I64(x) => x.fmt(f),
            Solution::I128(x) => x.fmt(f),
            Solution::ISIZE(x) => x.fmt(f),
            Solution::U16(x) => x.fmt(f),
            Solution::U32(x) => x.fmt(f),
            Solution::U64(x) => x.fmt(f),
            Solution::U128(x) => x.fmt(f),
            Solution::USIZE(x) => x.fmt(f),
            Solution::Str(x) => x.fmt(f),
        }
    }
}

pub mod load;
pub use load::get_data_server;

pub mod solutions;
pub use solutions::solve;
