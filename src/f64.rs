use core::cmp::Ordering;
use core::cmp::{Eq, Ord, PartialEq, PartialOrd};
use core::hash::{Hash, Hasher};
use core::option::{Option, Option::Some};

pub struct OrderedFloat(pub f64);

// canonical raw bit patterns (for hashing)
const CANONICAL_NAN_BITS: u64 = 0x7ff8000000000000u64;

#[inline(always)]
fn canonicalize_signed_zero(x: f64) -> f64 {
    // -0.0 + 0.0 == +0.0 under IEEE754 roundTiesToEven rounding mode,
    // which Rust guarantees. Thus by adding a positive zero we
    // canonicalize signed zero without any branches in one instruction.
    x + 0.0
}

impl Hash for OrderedFloat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let bits = if self.0.is_nan() {
            CANONICAL_NAN_BITS
        } else {
            canonicalize_signed_zero(self.0).to_bits()
        };

        bits.hash(state)
    }
}

impl Eq for OrderedFloat {}

impl PartialEq for OrderedFloat {
    #[inline]
    fn eq(&self, other: &OrderedFloat) -> bool {
        if self.0.is_nan() {
            other.0.is_nan()
        } else {
            self.0 == other.0
        }
    }
}

impl PartialOrd for OrderedFloat {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        !self.ge(other)
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        other.ge(self)
    }

    #[inline]
    fn gt(&self, other: &Self) -> bool {
        !other.ge(self)
    }

    #[inline]
    fn ge(&self, other: &Self) -> bool {
        // We consider all NaNs equal, and NaN is the largest possible
        // value. Thus if self is NaN we always return true. Otherwise
        // self >= other is correct. If other is also not NaN it is trivially
        // correct, and if it is we note that nothing can be greater or
        // equal to NaN except NaN itself, which we already handled earlier.
        self.0.is_nan() | (self.0 >= other.0)
    }
}

impl Ord for OrderedFloat {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        #[allow(clippy::comparison_chain)]
        if self < other {
            Ordering::Less
        } else if self > other {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
