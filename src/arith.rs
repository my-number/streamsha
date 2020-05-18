use core::ops::*;

macro_rules! define_word {
    ($word:ident,$t:ty) => {
        #[derive(Debug, PartialEq, Clone, Copy, Default)]
        pub struct $word(pub $t);

        impl BitXor for $word {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self::Output {
                Self(self.0 ^ rhs.0)
            }
        }
        impl BitOr for $word {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }
        impl BitAnd for $word {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }
        impl Not for $word {
            type Output = Self;

            fn not(self) -> Self::Output {
                Self(!self.0)
            }
        }

        impl Add for $word {
            type Output = Self;

            fn add(self, other: Self) -> Self::Output {
                Self(self.0.wrapping_add(other.0))
            }
        }
        impl Shl<usize> for $word {
            type Output = Self;
            fn shl(self, rhs: usize) -> Self::Output {
                Self(self.0 << rhs)
            }
        }
        impl Shr<usize> for $word {
            type Output = Self;
            fn shr(self, rhs: usize) -> Self::Output {
                Self(self.0 >> rhs)
            }
        }
        impl From<$t> for $word {
            fn from(i: $t) -> Self {
                Self(i)
            }
        }
    };
}

define_word!(Word32, u32);
define_word!(Word64, u64);

/// Performs circular right shift
pub fn rotr<T: Shl<usize, Output = T> + Shr<usize, Output = T> + BitOr<Output = T> + Copy>(
    x: T,
    n: usize,
) -> T {
    (x >> n) | (x << (core::mem::size_of::<T>() * 8 - n))
}
/// Performs circular left shift
#[allow(dead_code)]
pub fn rotl<T: Shl<usize, Output = T> + Shr<usize, Output = T> + BitOr<Output = T> + Copy>(
    x: T,
    n: usize,
) -> T {
    (x << n) | (x >> (core::mem::size_of::<T>() * 8 - n))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rotation_equivalence() {
        assert_eq!(rotr(32u32, 2), rotl(32u32, 30));
        assert_eq!(rotr(32u64, 2), rotl(32u64, 62));
        assert_eq!(rotr(Word64(32u64), 2), rotl(Word64(32u64), 62));
    }
}
