#![deny(missing_docs,
        missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unused_import_braces, unused_qualifications)]

//! Crate with convenient ruby like sugar
//!
//! Aim of this crate is to provide clean and easy to use
//! ruby-like APIs for basic rust primitives

/// Trait to add times() method to built-in types
pub trait Times 
    where Self: Sized
{
    /// Execute closure N times
    ///
    /// # Examples
    /// 
    /// ```
    /// use redstone::Times;
    ///
    /// fn main() {
    ///    10.times(|i| {
    ///        println!("Printing this {}th time", i);
    ///    });
    /// }
    /// ```   
    fn times<F: FnMut(Self)>(&self, closure: F);
}

macro_rules! impl_times {
    ($ptype:ty) => {
        impl Times for $ptype {
            fn times<F: FnMut($ptype)>(&self, mut closure: F) {
                for i in 0..*self {
                    closure(i)
                }
            }
        }
    }
}

impl_times!(i8);
impl_times!(u8);
impl_times!(i16);
impl_times!(u16);
impl_times!(i32);
impl_times!(u32);
impl_times!(i64);
impl_times!(u64);
impl_times!(usize);

#[cfg(test)]
mod tests {
    macro_rules! test_times {
        ($ptype:tt) => {
            mod $ptype {
                use super::super::*;
                #[test]
                fn times() {
                    let mut counter = 0;
                    let n: $ptype = 10;
                    n.times(|i| {
                        assert_eq!(counter, i);
                        counter += 1;
                    });
                    assert_eq!(counter, n);
                }
            }
        }
    }

    test_times!(i8);
    test_times!(u8);
    test_times!(i16);
    test_times!(u16);
    test_times!(i32);
    test_times!(u32);
    test_times!(i64);
    test_times!(u64);
    test_times!(usize);
}