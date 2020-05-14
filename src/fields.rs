use crate::prime_field::{PrimeField, PrimeFieldElt};
use crate::prime_power_field::PrimePowerField;



#[derive(Clone, Copy, Debug)]
pub struct GF2 {}
impl PrimeField for GF2 {
    const CHARACTERISTIC : u8 = 2;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_002 {
    use crate::GF2;
    use crate::prime_field::*;

    #[test]
    fn gf002() {
        let zero = GF2::zero;
        let one = GF2::one;
        for x in GF2::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF2::elts() {
            for y in GF2::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF2::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..3 {
            assert_eq!(x, PrimeFieldElt::from(i % 2));
            x = x + one;
        }
        for i in 0..2 {
            for j in 0..2 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF2>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF3 {}
impl PrimeField for GF3 {
    const CHARACTERISTIC : u8 = 3;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,    2,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_003 {
    use crate::GF3;
    use crate::prime_field::*;

    #[test]
    fn gf003() {
        let zero = GF3::zero;
        let one = GF3::one;
        for x in GF3::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF3::elts() {
            for y in GF3::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF3::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..4 {
            assert_eq!(x, PrimeFieldElt::from(i % 3));
            x = x + one;
        }
        for i in 0..3 {
            for j in 0..3 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF3>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF5 {}
impl PrimeField for GF5 {
    const CHARACTERISTIC : u8 = 5;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,    3,    2,    4,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_005 {
    use crate::GF5;
    use crate::prime_field::*;

    #[test]
    fn gf005() {
        let zero = GF5::zero;
        let one = GF5::one;
        for x in GF5::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF5::elts() {
            for y in GF5::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF5::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..6 {
            assert_eq!(x, PrimeFieldElt::from(i % 5));
            x = x + one;
        }
        for i in 0..5 {
            for j in 0..5 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF5>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF7 {}
impl PrimeField for GF7 {
    const CHARACTERISTIC : u8 = 7;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,    4,    5,    2,    3,    6,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_007 {
    use crate::GF7;
    use crate::prime_field::*;

    #[test]
    fn gf007() {
        let zero = GF7::zero;
        let one = GF7::one;
        for x in GF7::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF7::elts() {
            for y in GF7::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF7::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..8 {
            assert_eq!(x, PrimeFieldElt::from(i % 7));
            x = x + one;
        }
        for i in 0..7 {
            for j in 0..7 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF7>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF11 {}
impl PrimeField for GF11 {
    const CHARACTERISTIC : u8 = 11;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,    6,    4,    3,    9,    2,    8,    7,    5,   10,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_011 {
    use crate::GF11;
    use crate::prime_field::*;

    #[test]
    fn gf011() {
        let zero = GF11::zero;
        let one = GF11::one;
        for x in GF11::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF11::elts() {
            for y in GF11::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF11::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..12 {
            assert_eq!(x, PrimeFieldElt::from(i % 11));
            x = x + one;
        }
        for i in 0..11 {
            for j in 0..11 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF11>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF13 {}
impl PrimeField for GF13 {
    const CHARACTERISTIC : u8 = 13;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,    7,    9,   10,    8,   11,    2,    5,    3,    4,    6,   12,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_013 {
    use crate::GF13;
    use crate::prime_field::*;

    #[test]
    fn gf013() {
        let zero = GF13::zero;
        let one = GF13::one;
        for x in GF13::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF13::elts() {
            for y in GF13::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF13::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..14 {
            assert_eq!(x, PrimeFieldElt::from(i % 13));
            x = x + one;
        }
        for i in 0..13 {
            for j in 0..13 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF13>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF17 {}
impl PrimeField for GF17 {
    const CHARACTERISTIC : u8 = 17;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,    9,    6,   13,    7,    3,    5,   15,    2,   12,   14,   10,    4,   11,    8,
          16,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_017 {
    use crate::GF17;
    use crate::prime_field::*;

    #[test]
    fn gf017() {
        let zero = GF17::zero;
        let one = GF17::one;
        for x in GF17::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF17::elts() {
            for y in GF17::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF17::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..18 {
            assert_eq!(x, PrimeFieldElt::from(i % 17));
            x = x + one;
        }
        for i in 0..17 {
            for j in 0..17 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF17>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF19 {}
impl PrimeField for GF19 {
    const CHARACTERISTIC : u8 = 19;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   10,   13,    5,    4,   16,   11,   12,   17,    2,    7,    8,    3,   15,   14,
           6,    9,   18,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_019 {
    use crate::GF19;
    use crate::prime_field::*;

    #[test]
    fn gf019() {
        let zero = GF19::zero;
        let one = GF19::one;
        for x in GF19::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF19::elts() {
            for y in GF19::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF19::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..20 {
            assert_eq!(x, PrimeFieldElt::from(i % 19));
            x = x + one;
        }
        for i in 0..19 {
            for j in 0..19 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF19>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF23 {}
impl PrimeField for GF23 {
    const CHARACTERISTIC : u8 = 23;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   12,    8,    6,   14,    4,   10,    3,   18,    7,   21,    2,   16,    5,   20,
          13,   19,    9,   17,   15,   11,   22,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_023 {
    use crate::GF23;
    use crate::prime_field::*;

    #[test]
    fn gf023() {
        let zero = GF23::zero;
        let one = GF23::one;
        for x in GF23::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF23::elts() {
            for y in GF23::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF23::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..24 {
            assert_eq!(x, PrimeFieldElt::from(i % 23));
            x = x + one;
        }
        for i in 0..23 {
            for j in 0..23 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF23>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF29 {}
impl PrimeField for GF29 {
    const CHARACTERISTIC : u8 = 29;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   15,   10,   22,    6,    5,   25,   11,   13,    3,    8,   17,    9,   27,    2,
          20,   12,   21,   26,   16,   18,    4,   24,   23,    7,   19,   14,   28,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_029 {
    use crate::GF29;
    use crate::prime_field::*;

    #[test]
    fn gf029() {
        let zero = GF29::zero;
        let one = GF29::one;
        for x in GF29::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF29::elts() {
            for y in GF29::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF29::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..30 {
            assert_eq!(x, PrimeFieldElt::from(i % 29));
            x = x + one;
        }
        for i in 0..29 {
            for j in 0..29 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF29>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF31 {}
impl PrimeField for GF31 {
    const CHARACTERISTIC : u8 = 31;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   16,   21,    8,   25,   26,    9,    4,    7,   28,   17,   13,   12,   20,   29,
           2,   11,   19,   18,   14,    3,   24,   27,   22,    5,    6,   23,   10,   15,   30,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_031 {
    use crate::GF31;
    use crate::prime_field::*;

    #[test]
    fn gf031() {
        let zero = GF31::zero;
        let one = GF31::one;
        for x in GF31::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF31::elts() {
            for y in GF31::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF31::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..32 {
            assert_eq!(x, PrimeFieldElt::from(i % 31));
            x = x + one;
        }
        for i in 0..31 {
            for j in 0..31 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF31>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF37 {}
impl PrimeField for GF37 {
    const CHARACTERISTIC : u8 = 37;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   19,   25,   28,   15,   31,   16,   14,   33,   26,   27,   34,   20,    8,    5,
           7,   24,   35,    2,   13,   30,   32,   29,   17,    3,   10,   11,    4,   23,   21,    6,
          22,    9,   12,   18,   36,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_037 {
    use crate::GF37;
    use crate::prime_field::*;

    #[test]
    fn gf037() {
        let zero = GF37::zero;
        let one = GF37::one;
        for x in GF37::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF37::elts() {
            for y in GF37::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF37::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..38 {
            assert_eq!(x, PrimeFieldElt::from(i % 37));
            x = x + one;
        }
        for i in 0..37 {
            for j in 0..37 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF37>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF41 {}
impl PrimeField for GF41 {
    const CHARACTERISTIC : u8 = 41;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   21,   14,   31,   33,    7,    6,   36,   32,   37,   15,   24,   19,    3,   11,
          18,   29,   16,   13,   39,    2,   28,   25,   12,   23,   30,   38,   22,   17,   26,    4,
           9,    5,   35,   34,    8,   10,   27,   20,   40,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_041 {
    use crate::GF41;
    use crate::prime_field::*;

    #[test]
    fn gf041() {
        let zero = GF41::zero;
        let one = GF41::one;
        for x in GF41::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF41::elts() {
            for y in GF41::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF41::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..42 {
            assert_eq!(x, PrimeFieldElt::from(i % 41));
            x = x + one;
        }
        for i in 0..41 {
            for j in 0..41 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF41>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF43 {}
impl PrimeField for GF43 {
    const CHARACTERISTIC : u8 = 43;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   22,   29,   11,   26,   36,   37,   27,   24,   13,    4,   18,   10,   40,   23,
          35,   38,   12,   34,   28,   41,    2,   15,    9,   31,    5,    8,   20,    3,   33,   25,
          39,   30,   19,   16,    6,    7,   17,   32,   14,   21,   42,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_043 {
    use crate::GF43;
    use crate::prime_field::*;

    #[test]
    fn gf043() {
        let zero = GF43::zero;
        let one = GF43::one;
        for x in GF43::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF43::elts() {
            for y in GF43::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF43::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..44 {
            assert_eq!(x, PrimeFieldElt::from(i % 43));
            x = x + one;
        }
        for i in 0..43 {
            for j in 0..43 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF43>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF47 {}
impl PrimeField for GF47 {
    const CHARACTERISTIC : u8 = 47;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   24,   16,   12,   19,    8,   27,    6,   21,   33,   30,    4,   29,   37,   22,
           3,   36,   34,    5,   40,    9,   15,   45,    2,   32,   38,    7,   42,   13,   11,   44,
          25,   10,   18,   43,   17,   14,   26,   41,   20,   39,   28,   35,   31,   23,   46,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_047 {
    use crate::GF47;
    use crate::prime_field::*;

    #[test]
    fn gf047() {
        let zero = GF47::zero;
        let one = GF47::one;
        for x in GF47::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF47::elts() {
            for y in GF47::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF47::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..48 {
            assert_eq!(x, PrimeFieldElt::from(i % 47));
            x = x + one;
        }
        for i in 0..47 {
            for j in 0..47 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF47>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF53 {}
impl PrimeField for GF53 {
    const CHARACTERISTIC : u8 = 53;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   27,   18,   40,   32,    9,   38,   20,    6,   16,   29,   31,   49,   19,   46,
          10,   25,    3,   14,    8,   48,   41,   30,   42,   17,   51,    2,   36,   11,   23,   12,
           5,   45,   39,   50,   28,   43,    7,   34,    4,   22,   24,   37,   47,   33,   15,   44,
          21,   13,   35,   26,   52,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_053 {
    use crate::GF53;
    use crate::prime_field::*;

    #[test]
    fn gf053() {
        let zero = GF53::zero;
        let one = GF53::one;
        for x in GF53::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF53::elts() {
            for y in GF53::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF53::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..54 {
            assert_eq!(x, PrimeFieldElt::from(i % 53));
            x = x + one;
        }
        for i in 0..53 {
            for j in 0..53 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF53>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF59 {}
impl PrimeField for GF59 {
    const CHARACTERISTIC : u8 = 59;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   30,   20,   15,   12,   10,   17,   37,   46,    6,   43,    5,   50,   38,    4,
          48,    7,   23,   28,    3,   45,   51,   18,   32,   26,   25,   35,   19,   57,    2,   40,
          24,   34,   33,   27,   41,    8,   14,   56,   31,   36,   52,   11,   55,   21,    9,   54,
          16,   53,   13,   22,   42,   49,   47,   44,   39,   29,   58,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_059 {
    use crate::GF59;
    use crate::prime_field::*;

    #[test]
    fn gf059() {
        let zero = GF59::zero;
        let one = GF59::one;
        for x in GF59::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF59::elts() {
            for y in GF59::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF59::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..60 {
            assert_eq!(x, PrimeFieldElt::from(i % 59));
            x = x + one;
        }
        for i in 0..59 {
            for j in 0..59 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF59>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF61 {}
impl PrimeField for GF61 {
    const CHARACTERISTIC : u8 = 61;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   31,   41,   46,   49,   51,   35,   23,   34,   55,   50,   56,   47,   48,   57,
          42,   18,   17,   45,   58,   32,   25,    8,   28,   22,   54,   52,   24,   40,   59,    2,
          21,   37,    9,    7,   39,   33,   53,   36,   29,    3,   16,   44,   43,   19,    4,   13,
          14,    5,   11,    6,   27,   38,   26,   10,   12,   15,   20,   30,   60,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_061 {
    use crate::GF61;
    use crate::prime_field::*;

    #[test]
    fn gf061() {
        let zero = GF61::zero;
        let one = GF61::one;
        for x in GF61::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF61::elts() {
            for y in GF61::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF61::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..62 {
            assert_eq!(x, PrimeFieldElt::from(i % 61));
            x = x + one;
        }
        for i in 0..61 {
            for j in 0..61 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF61>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF67 {}
impl PrimeField for GF67 {
    const CHARACTERISTIC : u8 = 67;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   34,   45,   17,   27,   56,   48,   42,   15,   47,   61,   28,   31,   24,    9,
          21,    4,   41,   60,   57,   16,   64,   35,   14,   59,   49,    5,   12,   37,   38,   13,
          44,   65,    2,   23,   54,   29,   30,   55,   62,   18,    8,   53,   32,    3,   51,   10,
           7,   26,   63,   46,   58,   43,   36,   39,    6,   20,   52,   25,   19,   11,   40,   50,
          22,   33,   66,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_067 {
    use crate::GF67;
    use crate::prime_field::*;

    #[test]
    fn gf067() {
        let zero = GF67::zero;
        let one = GF67::one;
        for x in GF67::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF67::elts() {
            for y in GF67::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF67::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..68 {
            assert_eq!(x, PrimeFieldElt::from(i % 67));
            x = x + one;
        }
        for i in 0..67 {
            for j in 0..67 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF67>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF71 {}
impl PrimeField for GF71 {
    const CHARACTERISTIC : u8 = 71;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   36,   24,   18,   57,   12,   61,    9,    8,   64,   13,    6,   11,   66,   19,
          40,   46,    4,   15,   32,   44,   42,   34,    3,   54,   41,   50,   33,   49,   45,   55,
          20,   28,   23,   69,    2,   48,   43,   51,   16,   26,   22,   38,   21,   30,   17,   68,
          37,   29,   27,   39,   56,   67,   25,   31,   52,    5,   60,   65,   58,    7,   63,   62,
          10,   59,   14,   53,   47,   35,   70,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_071 {
    use crate::GF71;
    use crate::prime_field::*;

    #[test]
    fn gf071() {
        let zero = GF71::zero;
        let one = GF71::one;
        for x in GF71::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF71::elts() {
            for y in GF71::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF71::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..72 {
            assert_eq!(x, PrimeFieldElt::from(i % 71));
            x = x + one;
        }
        for i in 0..71 {
            for j in 0..71 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF71>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF73 {}
impl PrimeField for GF73 {
    const CHARACTERISTIC : u8 = 73;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   37,   49,   55,   44,   61,   21,   64,   65,   22,   20,   67,   45,   47,   39,
          32,   43,   69,   50,   11,    7,   10,   54,   70,   38,   59,   46,   60,   68,   56,   33,
          16,   31,   58,   48,   71,    2,   25,   15,   42,   57,   40,   17,    5,   13,   27,   14,
          35,    3,   19,   63,   66,   62,   23,    4,   30,   41,   34,   26,   28,    6,   53,   51,
           8,    9,   52,   12,   29,   18,   24,   36,   72,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_073 {
    use crate::GF73;
    use crate::prime_field::*;

    #[test]
    fn gf073() {
        let zero = GF73::zero;
        let one = GF73::one;
        for x in GF73::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF73::elts() {
            for y in GF73::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF73::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..74 {
            assert_eq!(x, PrimeFieldElt::from(i % 73));
            x = x + one;
        }
        for i in 0..73 {
            for j in 0..73 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF73>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF79 {}
impl PrimeField for GF79 {
    const CHARACTERISTIC : u8 = 79;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   40,   53,   20,   16,   66,   34,   10,   44,    8,   36,   33,   73,   17,   58,
           5,   14,   22,   25,    4,   64,   18,   55,   56,   19,   76,   41,   48,   30,   29,   51,
          42,   12,    7,   70,   11,   47,   52,   77,    2,   27,   32,   68,    9,   72,   67,   37,
          28,   50,   49,   31,   38,    3,   60,   23,   24,   61,   15,   75,   54,   57,   65,   74,
          21,   62,    6,   46,   43,   71,   35,   69,   45,   13,   63,   59,   26,   39,   78,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_079 {
    use crate::GF79;
    use crate::prime_field::*;

    #[test]
    fn gf079() {
        let zero = GF79::zero;
        let one = GF79::one;
        for x in GF79::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF79::elts() {
            for y in GF79::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF79::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..80 {
            assert_eq!(x, PrimeFieldElt::from(i % 79));
            x = x + one;
        }
        for i in 0..79 {
            for j in 0..79 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF79>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF83 {}
impl PrimeField for GF83 {
    const CHARACTERISTIC : u8 = 83;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   42,   28,   21,   50,   14,   12,   52,   37,   25,   68,    7,   32,    6,   72,
          26,   44,   60,   35,   54,    4,   34,   65,   45,   10,   16,   40,    3,   63,   36,   75,
          13,   78,   22,   19,   30,    9,   59,   66,   27,   81,    2,   56,   17,   24,   74,   53,
          64,   61,    5,   70,    8,   47,   20,   80,   43,   67,   73,   38,   18,   49,   79,   29,
          48,   23,   39,   57,   11,   77,   51,   76,   15,   58,   46,   31,   71,   69,   33,   62,
          55,   41,   82,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_083 {
    use crate::GF83;
    use crate::prime_field::*;

    #[test]
    fn gf083() {
        let zero = GF83::zero;
        let one = GF83::one;
        for x in GF83::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF83::elts() {
            for y in GF83::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF83::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..84 {
            assert_eq!(x, PrimeFieldElt::from(i % 83));
            x = x + one;
        }
        for i in 0..83 {
            for j in 0..83 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF83>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF89 {}
impl PrimeField for GF89 {
    const CHARACTERISTIC : u8 = 89;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   45,   30,   67,   18,   15,   51,   78,   10,    9,   81,   52,   48,   70,    6,
          39,   21,    5,   75,   49,   17,   85,   31,   26,   57,   24,   33,   35,   43,    3,   23,
          64,   27,   55,   28,   47,   77,   82,   16,   69,   76,   53,   29,   87,    2,   60,   36,
          13,   20,   73,    7,   12,   42,   61,   34,   62,   25,   66,   86,   46,   54,   56,   65,
          32,   63,   58,    4,   72,   40,   14,   84,   68,   50,   83,   19,   41,   37,    8,   80,
          79,   11,   38,   74,   71,   22,   59,   44,   88,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_089 {
    use crate::GF89;
    use crate::prime_field::*;

    #[test]
    fn gf089() {
        let zero = GF89::zero;
        let one = GF89::one;
        for x in GF89::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF89::elts() {
            for y in GF89::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF89::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..90 {
            assert_eq!(x, PrimeFieldElt::from(i % 89));
            x = x + one;
        }
        for i in 0..89 {
            for j in 0..89 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF89>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF97 {}
impl PrimeField for GF97 {
    const CHARACTERISTIC : u8 = 97;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   49,   65,   73,   39,   81,   14,   85,   54,   68,   53,   89,   15,    7,   13,
          91,   40,   27,   46,   34,   37,   75,   38,   93,   66,   56,   18,   52,   87,   55,   72,
          94,   50,   20,   61,   62,   21,   23,    5,   17,   71,   67,   88,   86,   69,   19,   64,
          95,    2,   33,   78,   28,   11,    9,   30,   26,   80,   92,   74,   76,   35,   36,   77,
          47,    3,   25,   42,   10,   45,   79,   41,   31,    4,   59,   22,   60,   63,   51,   70,
          57,    6,   84,   90,   82,    8,   44,   29,   43,   12,   83,   16,   58,   24,   32,   48,
          96,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_097 {
    use crate::GF97;
    use crate::prime_field::*;

    #[test]
    fn gf097() {
        let zero = GF97::zero;
        let one = GF97::one;
        for x in GF97::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF97::elts() {
            for y in GF97::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF97::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..98 {
            assert_eq!(x, PrimeFieldElt::from(i % 97));
            x = x + one;
        }
        for i in 0..97 {
            for j in 0..97 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF97>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF101 {}
impl PrimeField for GF101 {
    const CHARACTERISTIC : u8 = 101;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   51,   34,   76,   81,   17,   29,   38,   45,   91,   46,   59,   70,   65,   27,
          19,    6,   73,   16,   96,   77,   23,   22,   80,   97,   35,   15,   83,    7,   64,   88,
          60,   49,    3,   26,   87,   71,    8,   57,   48,   69,   89,   47,   62,    9,   11,   43,
          40,   33,   99,    2,   68,   61,   58,   90,   92,   39,   54,   12,   32,   53,   44,   93,
          30,   14,   75,   98,   52,   41,   13,   37,   94,   18,   86,   66,    4,   21,   79,   78,
          24,    5,   85,   28,   95,   82,   74,   36,   31,   42,   55,   10,   56,   63,   72,   84,
          20,   25,   67,   50,  100,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_101 {
    use crate::GF101;
    use crate::prime_field::*;

    #[test]
    fn gf101() {
        let zero = GF101::zero;
        let one = GF101::one;
        for x in GF101::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF101::elts() {
            for y in GF101::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF101::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..102 {
            assert_eq!(x, PrimeFieldElt::from(i % 101));
            x = x + one;
        }
        for i in 0..101 {
            for j in 0..101 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF101>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF103 {}
impl PrimeField for GF103 {
    const CHARACTERISTIC : u8 = 103;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   52,   69,   26,   62,   86,   59,   13,   23,   31,   75,   43,    8,   81,   55,
          58,   97,   63,   38,   67,   54,   89,    9,   73,   33,    4,   42,   92,   32,   79,   10,
          29,   25,  100,   53,   83,   39,   19,   37,   85,   98,   27,   12,   96,   87,   56,   57,
          88,   82,   68,  101,    2,   35,   21,   15,   46,   47,   16,    7,   91,   76,    5,   18,
          66,   84,   64,   20,   50,    3,   78,   74,   93,   24,   71,   11,   61,   99,   70,   30,
          94,   14,   49,   36,   65,   40,    6,   45,   48,   22,   95,   60,   28,   72,   80,   90,
          44,   17,   41,   77,   34,   51,  102,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_103 {
    use crate::GF103;
    use crate::prime_field::*;

    #[test]
    fn gf103() {
        let zero = GF103::zero;
        let one = GF103::one;
        for x in GF103::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF103::elts() {
            for y in GF103::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF103::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..104 {
            assert_eq!(x, PrimeFieldElt::from(i % 103));
            x = x + one;
        }
        for i in 0..103 {
            for j in 0..103 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF103>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF107 {}
impl PrimeField for GF107 {
    const CHARACTERISTIC : u8 = 107;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   54,   36,   27,   43,   18,   46,   67,   12,   75,   39,    9,   33,   23,   50,
          87,   63,    6,   62,   91,   51,   73,   14,   58,   30,   70,    4,   65,   48,   25,   38,
          97,   13,   85,   52,    3,   81,   31,   11,   99,   47,   79,    5,   90,   88,    7,   41,
          29,   83,   15,   21,   35,  105,    2,   72,   86,   92,   24,   78,   66,  100,   19,   17,
         102,   28,   60,    8,   96,   76,   26,  104,   55,   22,   94,   10,   69,   82,   59,   42,
         103,   37,   77,   49,   93,   34,   56,   16,   45,  101,   44,   20,   57,   84,   74,   98,
          68,   32,   95,   40,   61,   89,   64,   80,   71,   53,  106,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_107 {
    use crate::GF107;
    use crate::prime_field::*;

    #[test]
    fn gf107() {
        let zero = GF107::zero;
        let one = GF107::one;
        for x in GF107::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF107::elts() {
            for y in GF107::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF107::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..108 {
            assert_eq!(x, PrimeFieldElt::from(i % 107));
            x = x + one;
        }
        for i in 0..107 {
            for j in 0..107 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF107>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF109 {}
impl PrimeField for GF109 {
    const CHARACTERISTIC : u8 = 109;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   55,   73,   82,   22,   91,   78,   41,   97,   11,   10,  100,   42,   39,   80,
          75,   77,  103,   23,   60,   26,    5,   19,   50,   48,   21,  105,   74,   94,   40,  102,
          92,   76,   93,   81,  106,   56,   66,   14,   30,    8,   13,   71,   57,   63,   64,   58,
          25,   89,   24,   62,   65,   72,  107,    2,   37,   44,   47,   85,   20,   84,   51,   45,
          46,   52,   38,   96,  101,   79,   95,   43,   53,    3,   28,   16,   33,   17,    7,   69,
          15,   35,    4,   88,   61,   59,   90,  104,   83,   49,   86,    6,   32,   34,   29,   70,
          67,    9,   99,   98,   12,   68,   31,   18,   87,   27,   36,   54,  108,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_109 {
    use crate::GF109;
    use crate::prime_field::*;

    #[test]
    fn gf109() {
        let zero = GF109::zero;
        let one = GF109::one;
        for x in GF109::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF109::elts() {
            for y in GF109::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF109::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..110 {
            assert_eq!(x, PrimeFieldElt::from(i % 109));
            x = x + one;
        }
        for i in 0..109 {
            for j in 0..109 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF109>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF113 {}
impl PrimeField for GF113 {
    const CHARACTERISTIC : u8 = 113;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   57,   38,   85,   68,   19,   97,   99,   88,   34,   72,   66,   87,  105,   98,
         106,   20,   44,    6,   17,   70,   36,   59,   33,  104,  100,   67,  109,   39,   49,   62,
          53,   24,   10,   42,   22,   55,    3,   29,   65,  102,   35,   92,   18,  108,   86,  101,
          73,   30,   52,   82,   50,   32,   90,   37,  111,    2,   76,   23,   81,   63,   31,   61,
          83,   40,   12,   27,    5,   95,   21,   78,   11,   48,   84,  110,   58,   91,   71,  103,
          89,   60,   51,   64,   74,    4,   46,   13,    9,   80,   54,   77,   43,   96,  107,   69,
          93,    7,   15,    8,   26,   47,   41,   79,   25,   14,   16,   94,   45,   28,   75,   56,
         112,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_113 {
    use crate::GF113;
    use crate::prime_field::*;

    #[test]
    fn gf113() {
        let zero = GF113::zero;
        let one = GF113::one;
        for x in GF113::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF113::elts() {
            for y in GF113::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF113::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..114 {
            assert_eq!(x, PrimeFieldElt::from(i % 113));
            x = x + one;
        }
        for i in 0..113 {
            for j in 0..113 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF113>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF127 {}
impl PrimeField for GF127 {
    const CHARACTERISTIC : u8 = 127;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   64,   85,   32,   51,  106,  109,   16,  113,   89,  104,   53,   88,  118,   17,
           8,   15,  120,  107,  108,  121,   52,  116,   90,   61,   44,   80,   59,   92,   72,   41,
           4,   77,   71,   98,   60,  103,  117,  114,   54,   31,  124,   65,   26,   48,   58,  100,
          45,   70,   94,    5,   22,   12,   40,   97,   93,   78,   46,   28,   36,   25,   84,  125,
           2,   43,  102,   91,   99,   81,   49,   34,   30,   87,  115,  105,  122,   33,   57,   82,
          27,   69,   79,  101,   62,    3,   96,   73,   13,   10,   24,   67,   29,   56,   50,  123,
          86,   55,   35,   68,   47,   83,   66,   37,   11,   75,    6,   19,   20,    7,  112,  119,
         110,    9,   39,   74,   23,   38,   14,  111,   18,   21,   76,   95,   42,   63,  126,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_127 {
    use crate::GF127;
    use crate::prime_field::*;

    #[test]
    fn gf127() {
        let zero = GF127::zero;
        let one = GF127::one;
        for x in GF127::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF127::elts() {
            for y in GF127::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF127::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..128 {
            assert_eq!(x, PrimeFieldElt::from(i % 127));
            x = x + one;
        }
        for i in 0..127 {
            for j in 0..127 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF127>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF131 {}
impl PrimeField for GF131 {
    const CHARACTERISTIC : u8 = 131;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   66,   44,   33,  105,   22,   75,   82,  102,  118,   12,   11,  121,  103,   35,
          41,   54,   51,   69,   59,   25,    6,   57,   71,   21,  126,   34,  117,  122,   83,   93,
          86,    4,   27,   15,   91,   85,  100,   84,   95,   16,   78,   64,    3,   99,   94,   92,
         101,  123,   76,   18,   63,   89,   17,   81,  124,   23,   61,   20,  107,   58,  112,   52,
          43,  129,    2,   88,   79,   19,   73,   24,  111,   70,  108,    7,   50,  114,   42,   68,
         113,   55,    8,   30,   39,   37,   32,  128,   67,   53,  115,   36,   47,   31,   46,   40,
         116,  104,  127,   45,   38,   48,    9,   14,   97,    5,  110,   60,   74,  125,  106,   72,
          62,   80,   77,   90,   96,   28,   10,  120,  119,   13,   29,   49,   56,  109,   26,   98,
          87,   65,  130,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_131 {
    use crate::GF131;
    use crate::prime_field::*;

    #[test]
    fn gf131() {
        let zero = GF131::zero;
        let one = GF131::one;
        for x in GF131::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF131::elts() {
            for y in GF131::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF131::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..132 {
            assert_eq!(x, PrimeFieldElt::from(i % 131));
            x = x + one;
        }
        for i in 0..131 {
            for j in 0..131 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF131>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF137 {}
impl PrimeField for GF137 {
    const CHARACTERISTIC : u8 = 137;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   69,   46,  103,   55,   23,   98,  120,   61,   96,   25,   80,  116,   49,   64,
          60,  129,   99,  101,   48,  124,   81,    6,   40,   11,   58,   66,   93,   52,   32,   84,
          30,   54,  133,   47,  118,  100,  119,  130,   24,  127,   62,   51,  109,   67,    3,   35,
          20,   14,   74,   43,   29,  106,   33,    5,  115,  125,   26,   72,   16,    9,   42,   87,
          15,   78,   27,   45,  135,    2,   92,  110,   59,  122,   50,   95,  128,  121,   65,  111,
          12,   22,  132,  104,   31,  108,   94,   63,  123,  117,  102,  134,   70,   28,   86,   75,
          10,  113,    7,   18,   37,   19,   90,    4,   83,  107,   53,  105,   85,   44,   71,   79,
         126,   97,  131,   56,   13,   89,   36,   38,    8,   77,   73,   88,   21,   57,  112,   41,
          76,   17,   39,  114,   82,   34,   91,   68,  136,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_137 {
    use crate::GF137;
    use crate::prime_field::*;

    #[test]
    fn gf137() {
        let zero = GF137::zero;
        let one = GF137::one;
        for x in GF137::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF137::elts() {
            for y in GF137::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF137::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..138 {
            assert_eq!(x, PrimeFieldElt::from(i % 137));
            x = x + one;
        }
        for i in 0..137 {
            for j in 0..137 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF137>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF139 {}
impl PrimeField for GF139 {
    const CHARACTERISTIC : u8 = 139;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   70,   93,   35,   28,  116,   20,   87,   31,   14,   38,   58,  107,   10,  102,
         113,   90,   85,   22,    7,   53,   19,  133,   29,   89,  123,  103,    5,   24,   51,    9,
         126,   59,   45,    4,  112,  124,   11,   82,   73,   78,   96,   97,   79,   34,  136,   71,
          84,  122,  114,   30,  131,   21,  121,   91,   72,  100,   12,   33,   95,   98,   74,   64,
          63,   77,   99,   83,   92,  137,    2,   47,   56,   40,   62,   76,   75,   65,   41,   44,
         106,  127,   39,   67,   48,   18,  118,    8,  109,   25,   17,   55,   68,    3,  105,   60,
          42,   43,   61,   66,   57,  128,   15,   27,  135,   94,   80,   13,  130,   88,  115,  134,
          36,   16,   50,  110,    6,  120,   86,  132,  117,   54,   49,   26,   37,  129,   32,   81,
         101,  125,  108,   52,  119,   23,  111,  104,   46,   69,  138,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_139 {
    use crate::GF139;
    use crate::prime_field::*;

    #[test]
    fn gf139() {
        let zero = GF139::zero;
        let one = GF139::one;
        for x in GF139::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF139::elts() {
            for y in GF139::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF139::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..140 {
            assert_eq!(x, PrimeFieldElt::from(i % 139));
            x = x + one;
        }
        for i in 0..139 {
            for j in 0..139 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF139>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF149 {}
impl PrimeField for GF149 {
    const CHARACTERISTIC : u8 = 149;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   75,   50,  112,   30,   25,   64,   56,  116,   15,  122,   87,   23,   32,   10,
          28,  114,   58,  102,   82,   71,   61,   13,  118,    6,   86,  138,   16,   36,    5,  125,
          14,  140,   57,  132,   29,  145,   51,  107,   41,   40,  110,   52,  105,   53,   81,  130,
          59,   73,    3,   38,   43,   45,   69,   84,    8,   34,   18,   48,   77,   22,  137,  123,
           7,   94,   70,  129,  103,   54,   66,   21,   89,   49,  147,    2,  100,   60,  128,   83,
          95,   46,   20,   79,   55,  142,   26,   12,  127,   72,  101,  131,  115,  141,   65,   80,
         104,  106,  111,  146,   76,   90,   19,   68,   96,   44,   97,   39,  109,  108,   42,   98,
           4,  120,   17,   92,    9,  135,   24,  144,  113,  133,   11,   63,  143,   31,  136,   88,
          78,   67,   47,   91,   35,  121,  139,  117,  126,   62,   27,  134,   33,   93,   85,  124,
         119,   37,   99,   74,  148,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_149 {
    use crate::GF149;
    use crate::prime_field::*;

    #[test]
    fn gf149() {
        let zero = GF149::zero;
        let one = GF149::one;
        for x in GF149::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF149::elts() {
            for y in GF149::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF149::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..150 {
            assert_eq!(x, PrimeFieldElt::from(i % 149));
            x = x + one;
        }
        for i in 0..149 {
            for j in 0..149 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF149>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF151 {}
impl PrimeField for GF151 {
    const CHARACTERISTIC : u8 = 151;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   76,  101,   38,  121,  126,  108,   19,   84,  136,   55,   63,   93,   54,  141,
          85,   80,   42,    8,   68,   36,  103,   46,  107,  145,  122,   28,   27,  125,  146,   39,
         118,  119,   40,   82,   21,   49,    4,   31,   34,   70,   18,  144,  127,   47,   23,   45,
         129,   37,  148,   77,   61,   57,   14,   11,   89,   53,  138,   64,   73,   52,   95,   12,
          59,   79,  135,  142,   20,  116,   41,  134,   86,   60,  100,  149,    2,   51,   91,   65,
          17,  110,   35,  131,    9,   16,   72,   92,  139,   56,   99,   78,   87,   13,   98,   62,
         140,  137,   94,   90,   74,    3,  114,   22,  106,  128,  104,   24,    7,  133,   81,  117,
         120,  147,  102,  130,   69,  111,   32,   33,  112,    5,   26,  124,  123,   29,    6,   44,
         105,   48,  115,   83,  143,  109,   71,   66,   10,   97,   58,   88,   96,   15,   67,  132,
          43,   25,   30,  113,   50,   75,  150,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_151 {
    use crate::GF151;
    use crate::prime_field::*;

    #[test]
    fn gf151() {
        let zero = GF151::zero;
        let one = GF151::one;
        for x in GF151::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF151::elts() {
            for y in GF151::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF151::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..152 {
            assert_eq!(x, PrimeFieldElt::from(i % 151));
            x = x + one;
        }
        for i in 0..151 {
            for j in 0..151 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF151>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF157 {}
impl PrimeField for GF157 {
    const CHARACTERISTIC : u8 = 157;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   79,  105,  118,   63,  131,   45,   59,   35,  110,  100,  144,  145,  101,   21,
         108,   37,   96,  124,   55,   15,   50,   41,   72,   44,  151,   64,  129,   65,   89,   76,
          54,  138,   97,    9,   48,   17,   62,  153,  106,   23,   86,   84,   25,    7,   99,  147,
          36,  141,   22,  117,  154,   80,   32,   20,  143,  146,  111,    8,  123,  139,   38,    5,
          27,   29,   69,   75,  127,   66,   83,  115,   24,  114,   87,   67,   31,  104,  155,    2,
          53,  126,   90,   70,   43,  133,   42,   74,   91,   30,   82,   88,  128,  130,  152,  119,
          18,   34,  149,   46,   11,   14,  137,  125,   77,    3,   40,  135,   16,  121,   10,   58,
         150,  132,   73,   71,  134,   51,    4,   95,  140,  109,  148,   60,   19,  103,   81,   68,
          92,   28,   93,    6,  113,   85,  116,  107,  142,  102,   33,   61,  120,   49,  136,   56,
          12,   13,   57,   47,  122,   98,  112,   26,   94,   39,   52,   78,  156,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_157 {
    use crate::GF157;
    use crate::prime_field::*;

    #[test]
    fn gf157() {
        let zero = GF157::zero;
        let one = GF157::one;
        for x in GF157::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF157::elts() {
            for y in GF157::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF157::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..158 {
            assert_eq!(x, PrimeFieldElt::from(i % 157));
            x = x + one;
        }
        for i in 0..157 {
            for j in 0..157 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF157>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF163 {}
impl PrimeField for GF163 {
    const CHARACTERISTIC : u8 = 163;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   82,  109,   41,   98,  136,   70,  102,  145,   49,   89,   68,  138,   35,   87,
          51,   48,  154,  103,  106,  132,  126,   78,   34,  150,   69,  157,   99,   45,  125,  142,
         107,   84,   24,   14,   77,  141,  133,   46,   53,    4,   66,   91,   63,   29,   39,  111,
          17,   10,   75,   16,  116,   40,  160,   83,  131,  143,  104,  105,  144,  155,   71,   44,
         135,  158,   42,   73,   12,   26,    7,   62,  120,   67,  152,   50,  148,   36,   23,  130,
         108,  161,    2,   55,   33,  140,  127,   15,  113,   11,   96,   43,  101,  156,  137,  151,
          90,  121,    5,   28,  119,   92,    8,   19,   58,   59,   20,   32,   80,    3,  123,   47,
         147,   88,  153,  146,   52,  124,  134,  100,   72,   97,  159,  110,  117,   30,   22,   86,
         149,  139,   79,   56,   21,   38,  118,   64,    6,   94,   13,  129,   85,   37,   31,   57,
          60,    9,  115,  112,   76,  128,   25,   95,   74,  114,   18,   61,   93,   27,   65,  122,
          54,   81,  162,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_163 {
    use crate::GF163;
    use crate::prime_field::*;

    #[test]
    fn gf163() {
        let zero = GF163::zero;
        let one = GF163::one;
        for x in GF163::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF163::elts() {
            for y in GF163::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF163::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..164 {
            assert_eq!(x, PrimeFieldElt::from(i % 163));
            x = x + one;
        }
        for i in 0..163 {
            for j in 0..163 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF163>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF167 {}
impl PrimeField for GF167 {
    const CHARACTERISTIC : u8 = 167;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   84,   56,   42,   67,   28,   24,   21,  130,  117,   76,   14,   90,   12,   78,
          94,   59,   65,   44,  142,    8,   38,  138,    7,  147,   45,   99,    6,  144,   39,   97,
          47,   81,  113,  105,  116,  158,   22,   30,   71,  110,    4,  101,   19,   26,   69,   32,
          87,   75,  157,  131,  106,  104,  133,   82,    3,  126,   72,   17,  103,  115,  132,  114,
         107,   18,  124,    5,  140,   46,  136,   40,   58,  151,   79,   49,   11,  154,   15,   74,
         119,   33,   55,  165,    2,  112,  134,   48,   93,  152,   13,  156,  118,   88,   16,  109,
         127,   31,  121,   27,  162,   43,  149,   60,   53,   35,   52,   64,  150,   95,   41,  164,
          85,   34,   63,   61,   36,   10,   92,   80,  135,   98,  141,  148,   66,  163,   57,   96,
         137,  145,    9,   51,   62,   54,   86,  120,   70,  128,   23,  161,   68,  122,   20,  160,
          29,  129,  159,   25,  123,  102,  108,   73,   89,  155,   77,  153,   91,   50,   37,  146,
         143,  139,  100,  125,  111,   83,  166,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_167 {
    use crate::GF167;
    use crate::prime_field::*;

    #[test]
    fn gf167() {
        let zero = GF167::zero;
        let one = GF167::one;
        for x in GF167::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF167::elts() {
            for y in GF167::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF167::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..168 {
            assert_eq!(x, PrimeFieldElt::from(i % 167));
            x = x + one;
        }
        for i in 0..167 {
            for j in 0..167 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF167>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF173 {}
impl PrimeField for GF173 {
    const CHARACTERISTIC : u8 = 173;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   87,   58,  130,  104,   29,   99,   65,   77,   52,   63,  101,   40,  136,  150,
         119,  112,  125,   82,   26,   33,  118,  158,  137,   90,   20,  141,   68,    6,   75,   67,
         146,   21,   56,   89,  149,  159,   41,   71,   13,   38,  103,  169,   59,   50,   79,   81,
         155,  113,   45,   95,   10,  111,  157,  151,   34,   85,    3,   44,  124,  156,  120,   11,
          73,    8,   97,   31,   28,  168,  131,   39,  161,   64,  166,   30,  107,    9,  122,   46,
          93,   47,   19,  148,  138,   57,  171,    2,  116,   35,   25,  154,  126,   80,  127,   51,
         164,   66,  143,    7,  109,   12,  134,   42,    5,  145,  142,   76,  165,  100,  162,   53,
          17,   49,  129,  170,   88,  139,   22,   16,   62,  163,   78,  128,   60,   18,   92,   94,
         123,  114,    4,   70,  135,  160,  102,  132,   14,   24,   84,  117,  152,   27,  106,   98,
         167,  105,   32,  153,   83,   36,   15,   55,  140,  147,   91,   48,   61,   54,   23,   37,
         133,   72,  110,  121,   96,  108,   74,  144,   69,   43,  115,   86,  172,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_173 {
    use crate::GF173;
    use crate::prime_field::*;

    #[test]
    fn gf173() {
        let zero = GF173::zero;
        let one = GF173::one;
        for x in GF173::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF173::elts() {
            for y in GF173::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF173::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..174 {
            assert_eq!(x, PrimeFieldElt::from(i % 173));
            x = x + one;
        }
        for i in 0..173 {
            for j in 0..173 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF173>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF179 {}
impl PrimeField for GF179 {
    const CHARACTERISTIC : u8 = 179;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   90,   60,   45,   36,   30,  128,  112,   20,   18,  114,   15,  124,   64,   12,
          56,  158,   10,   66,    9,  162,   57,  109,   97,   43,   62,  126,   32,  142,    6,   52,
          28,   38,   79,  133,    5,  150,   33,  101,   94,  131,   81,   25,  118,    4,  144,   80,
         138,   95,  111,  172,   31,  152,   63,  166,   16,   22,   71,   88,    3,  135,   26,   54,
          14,  168,   19,  171,  129,   96,  156,   58,   92,  103,   75,   74,  106,   93,  140,   34,
          47,   42,  155,  110,  130,  139,  102,  107,   59,  177,    2,  120,   72,   77,   40,   49,
          69,   24,  137,  132,  145,   39,   86,   73,  105,  104,   76,   87,  121,   23,   83,   50,
           8,  160,   11,  165,  125,  153,   44,  176,   91,  108,  157,  163,   13,  116,   27,  148,
           7,   68,   84,   41,   99,   35,  175,   61,  154,   98,   48,   85,   78,  146,   29,  174,
          46,  100,  141,  151,  127,  173,   37,  147,   53,  117,  136,   82,   70,  122,   17,  170,
         113,  169,   21,  123,  167,  115,   55,  164,   65,  161,  159,   67,   51,  149,  143,  134,
         119,   89,  178,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_179 {
    use crate::GF179;
    use crate::prime_field::*;

    #[test]
    fn gf179() {
        let zero = GF179::zero;
        let one = GF179::one;
        for x in GF179::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF179::elts() {
            for y in GF179::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF179::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..180 {
            assert_eq!(x, PrimeFieldElt::from(i % 179));
            x = x + one;
        }
        for i in 0..179 {
            for j in 0..179 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF179>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF181 {}
impl PrimeField for GF181 {
    const CHARACTERISTIC : u8 = 181;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   91,  121,  136,  145,  151,   26,   68,  161,  163,   33,  166,   14,   13,  169,
          34,   32,  171,  162,  172,   69,  107,   63,   83,   29,    7,  114,   97,   25,  175,  146,
          17,   11,   16,  150,  176,  137,   81,   65,   86,   53,  125,   80,  144,  177,  122,  104,
         132,  133,  105,   71,   94,   41,   57,   79,  139,   54,  103,  135,  178,   92,   73,   23,
          99,   39,   96,  154,    8,   21,   75,   51,   88,   62,  159,   70,  131,  134,  123,   55,
          43,   38,  117,   24,  153,  115,   40,  129,   72,  120,  179,    2,   61,  109,   52,  141,
          66,   28,  157,   64,  143,  138,  126,   58,   47,   50,  111,   22,  119,   93,  130,  106,
         160,  173,   27,   85,  142,   82,  158,  108,   89,    3,   46,   78,  127,   42,  102,  124,
         140,   87,  110,   76,   48,   49,   77,   59,    4,   37,  101,   56,  128,   95,  116,  100,
          44,    5,   31,  165,  170,  164,   35,    6,  156,   84,   67,  174,  152,   98,  118,   74,
         112,    9,   19,   10,  149,  147,   12,  168,  167,   15,  148,   18,   20,  113,  155,   30,
          36,   45,   60,   90,  180,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_181 {
    use crate::GF181;
    use crate::prime_field::*;

    #[test]
    fn gf181() {
        let zero = GF181::zero;
        let one = GF181::one;
        for x in GF181::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF181::elts() {
            for y in GF181::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF181::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..182 {
            assert_eq!(x, PrimeFieldElt::from(i % 181));
            x = x + one;
        }
        for i in 0..181 {
            for j in 0..181 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF181>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF191 {}
impl PrimeField for GF191 {
    const CHARACTERISTIC : u8 = 191;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   96,   64,   48,  153,   32,   82,   24,   85,  172,  139,   16,  147,   41,   51,
          12,   45,  138,  181,   86,   91,  165,  108,    8,  107,  169,   92,  116,  112,  121,   37,
           6,  110,  118,  131,   69,   31,  186,   49,   43,   14,  141,   40,  178,   17,   54,  126,
           4,   39,  149,   15,  180,  173,   46,   66,   58,  124,   56,   68,  156,  119,  114,   94,
           3,  144,   55,  134,   59,   36,  161,  113,  130,  157,  111,  163,   93,  129,  120,  162,
         117,  158,    7,  168,  166,    9,   20,  101,   89,   88,  104,   21,   27,   76,   63,  189,
           2,  128,  115,  164,  170,   87,  103,  102,   90,  171,  182,   25,   23,  184,   33,   74,
          29,   71,   62,   98,   28,   80,   34,   61,   78,   30,  155,  132,   57,  136,   47,  188,
          97,   77,   72,   35,  123,  135,   67,  133,  125,  145,   18,   11,  176,   42,  152,  187,
          65,  137,  174,   13,  151,   50,  177,  148,  142,    5,  160,  122,   60,   73,   81,  185,
         154,   70,   79,   75,   99,   22,   84,  183,   83,   26,  100,  105,   10,   53,  146,  179,
         140,  150,   44,  175,   52,   19,  106,  167,  109,  159,   38,  143,  127,   95,  190,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_191 {
    use crate::GF191;
    use crate::prime_field::*;

    #[test]
    fn gf191() {
        let zero = GF191::zero;
        let one = GF191::one;
        for x in GF191::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF191::elts() {
            for y in GF191::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF191::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..192 {
            assert_eq!(x, PrimeFieldElt::from(i % 191));
            x = x + one;
        }
        for i in 0..191 {
            for j in 0..191 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF191>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF193 {}
impl PrimeField for GF193 {
    const CHARACTERISTIC : u8 = 193;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   97,  129,  145,  116,  161,  138,  169,   43,   58,  158,  177,  104,   69,  103,
         181,  159,  118,   61,   29,   46,   79,   42,  185,  139,   52,  143,  131,   20,  148,  137,
         187,  117,  176,  182,   59,  120,  127,   99,  111,  113,   23,    9,  136,  163,   21,  115,
         189,  130,  166,   53,   26,   51,  168,  186,  162,  149,   10,   36,   74,   19,  165,  144,
         190,   98,  155,  121,   88,   14,   91,   87,  126,  156,   60,  175,  160,  188,  146,   22,
         152,  112,  153,  100,  108,  109,  101,   71,   68,  180,  178,   70,  107,  110,  154,  128,
         191,    2,   65,   39,   83,   86,  123,   15,   13,  125,  122,   92,   84,   85,   93,   40,
          81,   41,  171,   47,    5,   33,   18,  133,   37,   67,  106,  102,  179,  105,   72,   38,
          95,    3,   49,   28,  174,  119,  157,  183,   44,   31,    7,   25,  142,  167,  140,   27,
          63,    4,   78,  172,   30,   57,  184,  170,   80,   82,   94,   66,   73,  134,   11,   17,
          76,    6,   56,   45,  173,   62,   50,  141,   54,    8,  151,  114,  147,  164,  132,   75,
          34,   12,   90,  124,   89,   16,   35,  135,  150,   24,   55,   32,   77,   48,   64,   96,
         192,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_193 {
    use crate::GF193;
    use crate::prime_field::*;

    #[test]
    fn gf193() {
        let zero = GF193::zero;
        let one = GF193::one;
        for x in GF193::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF193::elts() {
            for y in GF193::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF193::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..194 {
            assert_eq!(x, PrimeFieldElt::from(i % 193));
            x = x + one;
        }
        for i in 0..193 {
            for j in 0..193 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF193>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF197 {}
impl PrimeField for GF197 {
    const CHARACTERISTIC : u8 = 197;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,   99,   66,  148,   79,   33,  169,   74,   22,  138,   18,  115,   91,  183,   92,
          37,   58,   11,   83,   69,  122,    9,   60,  156,  134,  144,   73,  190,   34,   46,   89,
         117,    6,   29,  152,  104,   16,  140,   96,  133,  173,   61,   55,  103,  162,   30,  109,
          78,  193,   67,   85,   72,  171,  135,   43,   95,  159,   17,  187,   23,   42,  143,  172,
         157,   97,    3,   50,  113,   20,   76,  111,   52,   27,    8,  176,   70,   87,   48,    5,
         165,   90,  185,   19,  129,   51,  126,   77,  150,   31,   81,   13,   15,  161,  153,   56,
          39,   65,  195,    2,  132,  158,  141,   44,   36,  182,  184,  116,  166,   47,  120,   71,
         146,   68,  178,   12,  107,   32,  192,  149,  110,  127,   21,  189,  170,  145,   86,  121,
         177,   84,  147,  194,  100,   40,   25,   54,  155,  174,   10,  180,   38,  102,  154,   62,
          26,  125,  112,  130,    4,  119,   88,  167,   35,   94,  142,  136,   24,   64,  101,   57,
         181,   93,   45,  168,  191,   80,  108,  151,  163,    7,  124,   53,   63,   41,  137,  188,
          75,  128,  114,  186,  139,  160,  105,   14,  106,   82,  179,   59,  175,  123,   28,  164,
         118,   49,  131,   98,  196,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_197 {
    use crate::GF197;
    use crate::prime_field::*;

    #[test]
    fn gf197() {
        let zero = GF197::zero;
        let one = GF197::one;
        for x in GF197::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF197::elts() {
            for y in GF197::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF197::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..198 {
            assert_eq!(x, PrimeFieldElt::from(i % 197));
            x = x + one;
        }
        for i in 0..197 {
            for j in 0..197 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF197>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF199 {}
impl PrimeField for GF199 {
    const CHARACTERISTIC : u8 = 199;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,  100,  133,   50,   40,  166,   57,   25,  177,   20,  181,   83,   46,  128,  146,
         112,   82,  188,   21,   10,   19,  190,   26,  141,    8,   23,   59,   64,  151,   73,  122,
          56,  193,   41,   91,   94,  156,  110,  148,    5,   34,  109,  162,   95,  115,   13,   72,
         170,   65,    4,  160,  111,  184,  129,   76,   32,    7,  175,   27,  136,   62,   61,  139,
          28,   49,  196,  101,  120,   75,  145,  185,   47,   30,   78,   69,   55,  168,   74,  131,
         102,   86,   17,   12,  154,   96,   81,  183,  147,  161,  157,   35,  106,  107,   36,   44,
          85,  119,  132,  197,    2,   67,   80,  114,  155,  163,   92,   93,  164,   42,   38,   52,
          16,  118,  103,   45,  187,  182,  113,   97,   68,  125,   31,  144,  130,  121,  169,  152,
          14,   54,  124,   79,   98,    3,  150,  171,   60,  138,  137,   63,  172,   24,  192,  167,
         123,   70,   15,   88,   39,  195,  134,   29,  127,  186,   84,  104,   37,   90,  165,  194,
          51,   89,   43,  105,  108,  158,    6,  143,   77,  126,   48,  135,  140,  176,  191,   58,
         173,    9,  180,  189,  178,   11,  117,   87,   53,   71,  153,  116,   18,  179,   22,  174,
         142,   33,  159,  149,   66,   99,  198,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_199 {
    use crate::GF199;
    use crate::prime_field::*;

    #[test]
    fn gf199() {
        let zero = GF199::zero;
        let one = GF199::one;
        for x in GF199::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF199::elts() {
            for y in GF199::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF199::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..200 {
            assert_eq!(x, PrimeFieldElt::from(i % 199));
            x = x + one;
        }
        for i in 0..199 {
            for j in 0..199 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF199>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF211 {}
impl PrimeField for GF211 {
    const CHARACTERISTIC : u8 = 211;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,  106,  141,   53,  169,  176,  181,  132,   47,  190,   96,   88,   65,  196,  197,
          66,  149,  129,  100,   95,  201,   48,  156,   44,   76,  138,   86,   98,  131,  204,  177,
          33,   32,  180,  205,  170,  154,   50,   92,  153,  175,  206,   54,   24,  136,   78,    9,
          22,   56,   38,  120,   69,    4,   43,  188,   49,  174,  171,   93,  102,  128,  194,   67,
         122,   13,   16,   63,   90,   52,  208,  107,   85,  185,   77,  166,   25,   74,   46,  203,
         182,   99,  193,  150,  103,   72,   27,  114,   12,  147,   68,  160,   39,   59,  110,   20,
          11,  124,   28,   81,   19,  117,   60,   84,  140,  209,    2,   71,  127,  151,   94,  192,
         130,  183,   87,  200,  191,  101,  152,  172,   51,  143,   64,  199,   97,  184,  139,  108,
          61,   18,  112,   29,    8,  165,  137,  186,   45,  134,   26,  126,  104,    3,  159,  121,
         148,  195,  198,   89,  144,   17,   83,  109,  118,   40,   37,  162,   23,  168,  207,  142,
          91,  173,  155,  189,  202,  133,   75,  187,  157,    5,   36,   58,  119,  161,   57,   41,
           6,   31,  179,  178,   34,    7,   80,  113,  125,   73,  135,  167,   55,  163,   10,  116,
         111,   82,   62,  145,   14,   15,  146,  123,  115,   21,  164,   79,   30,   35,   42,  158,
          70,  105,  210,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_211 {
    use crate::GF211;
    use crate::prime_field::*;

    #[test]
    fn gf211() {
        let zero = GF211::zero;
        let one = GF211::one;
        for x in GF211::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF211::elts() {
            for y in GF211::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF211::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..212 {
            assert_eq!(x, PrimeFieldElt::from(i % 211));
            x = x + one;
        }
        for i in 0..211 {
            for j in 0..211 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF211>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF223 {}
impl PrimeField for GF223 {
    const CHARACTERISTIC : u8 = 223;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,  112,  149,   56,  134,  186,   32,   28,  124,   67,  142,   93,  103,   16,  119,
          14,  105,   62,   47,  145,   85,   71,   97,  158,  116,  163,  190,    8,  100,  171,   36,
           7,  196,  164,   51,   31,  217,  135,  183,  184,  136,  154,   83,  147,  114,  160,   19,
          79,  132,   58,   35,  193,  101,   95,   73,    4,   90,   50,  189,  197,  117,   18,  177,
         115,  199,   98,   10,   82,  181,  137,   22,  127,   55,  220,  113,  179,   84,  203,   48,
          92,  212,   68,   43,   77,   21,  153,  182,  185,  218,   57,  174,   80,   12,  121,   54,
         151,   23,   66,  214,   29,   53,  129,   13,  208,   17,  162,  198,  159,  178,  148,  221,
           2,   75,   45,   64,   25,   61,  206,   15,  210,   94,  170,  194,    9,  157,  200,   72,
         169,  102,  211,  143,   49,  166,    5,   38,   41,   70,  202,  146,  180,  155,   11,  131,
         175,   20,  139,   44,  110,    3,  168,   96,  201,   86,   42,  141,  213,  125,   24,  108,
          46,  205,  106,   26,   34,  173,  133,  219,  150,  128,  122,   30,  188,  165,   91,  144,
         204,   63,  109,   76,  140,   69,   87,   39,   40,   88,    6,  192,  172,   59,   27,  216,
         187,   52,  123,  215,   33,   60,  107,   65,  126,  152,  138,   78,  176,  161,  118,  209,
         104,  207,  120,  130,   81,  156,   99,  195,  191,   37,   89,  167,   74,  111,  222,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_223 {
    use crate::GF223;
    use crate::prime_field::*;

    #[test]
    fn gf223() {
        let zero = GF223::zero;
        let one = GF223::one;
        for x in GF223::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF223::elts() {
            for y in GF223::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF223::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..224 {
            assert_eq!(x, PrimeFieldElt::from(i % 223));
            x = x + one;
        }
        for i in 0..223 {
            for j in 0..223 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF223>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF227 {}
impl PrimeField for GF227 {
    const CHARACTERISTIC : u8 = 227;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,  114,   76,   57,   91,   38,   65,  142,  101,  159,   62,   19,   35,  146,  106,
          71,  187,  164,   12,  193,  173,   31,   79,  123,  109,  131,  185,   73,   47,   53,   22,
         149,  172,  207,   13,   82,  135,    6,  163,  210,   72,  200,  132,  129,  111,  153,   29,
         175,  139,  168,  138,  179,   30,  206,  194,  150,    4,  137,  177,  140,   67,   11,  209,
         188,    7,   86,   61,  217,  102,  120,   16,   41,   28,  181,  112,    3,  171,  195,   23,
         105,  213,   36,   93,  100,  219,   66,  167,  178,  176,  169,    5,  190,   83,  128,  184,
         201,  110,  183,  133,   84,    9,   69,  108,  203,   80,   15,  157,  103,   25,   97,   45,
          75,  225,    2,  152,  182,  130,  202,  124,   70,  212,  147,   24,  119,  158,  218,  143,
          94,   44,  117,   26,   43,   99,  144,   37,  222,   58,   51,   49,   60,  161,    8,  127,
         134,  191,   14,  122,  204,   32,   56,  224,  115,   46,  199,  186,  211,  107,  125,   10,
         166,  141,  220,   39,   18,  216,  160,   87,   50,   90,  223,   77,   33,   21,  197,   48,
          89,   59,   88,   52,  198,   74,  116,   98,   95,   27,  155,   17,   64,  221,   92,  145,
         214,   20,   55,   78,  205,  174,  180,  154,   42,   96,  118,  104,  148,  196,   54,   34,
         215,   63,   40,  156,  121,   81,  192,  208,  165,   68,  126,   85,  162,  189,  136,  170,
         151,  113,  226,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_227 {
    use crate::GF227;
    use crate::prime_field::*;

    #[test]
    fn gf227() {
        let zero = GF227::zero;
        let one = GF227::one;
        for x in GF227::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF227::elts() {
            for y in GF227::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF227::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..228 {
            assert_eq!(x, PrimeFieldElt::from(i % 227));
            x = x + one;
        }
        for i in 0..227 {
            for j in 0..227 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF227>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF229 {}
impl PrimeField for GF229 {
    const CHARACTERISTIC : u8 = 229;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,  115,  153,  172,   46,  191,  131,   86,   51,   23,  125,  210,  141,  180,  168,
          43,   27,  140,  217,  126,  120,  177,   10,  105,   55,  185,   17,   90,   79,   84,  133,
         136,  118,  128,   72,   70,  130,  223,   47,   63,  162,   60,   16,  203,   56,    5,   39,
         167,  215,  142,    9,  207,  121,  123,   25,   45,  225,  154,   66,   42,  214,  181,   40,
          68,   74,   59,  188,   64,  156,   36,  100,   35,  160,   65,  171,  226,  116,  138,   29,
         146,   82,   81,  149,   30,   97,    8,  179,  216,  211,   28,  151,  117,  197,  134,  135,
         198,   85,  222,  192,   71,  195,  119,  209,  218,   24,  175,  122,  176,  208,  127,  196,
         137,  152,  227,    2,   77,   92,   33,  102,   21,   53,  107,   54,  205,   11,   20,  110,
          34,  158,   37,    7,  144,   31,   94,   95,   32,  112,   78,  201,   18,   13,   50,  221,
         132,  199,   80,  148,  147,   83,  200,   91,  113,    3,   58,  164,   69,  194,  129,  193,
          73,  165,   41,  170,  155,  161,  189,   48,   15,  187,  163,   75,    4,  184,  204,  106,
         108,   22,  220,   87,   14,   62,  190,  224,  173,   26,  213,  169,   67,  166,  182,    6,
          99,  159,  157,  101,  111,   93,   96,  145,  150,  139,  212,   44,  174,  124,  219,   52,
         109,  103,   12,   89,  202,  186,   61,   49,   88,   19,  104,  206,  178,  143,   98,   38,
         183,   57,   76,  114,  228,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_229 {
    use crate::GF229;
    use crate::prime_field::*;

    #[test]
    fn gf229() {
        let zero = GF229::zero;
        let one = GF229::one;
        for x in GF229::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF229::elts() {
            for y in GF229::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF229::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..230 {
            assert_eq!(x, PrimeFieldElt::from(i % 229));
            x = x + one;
        }
        for i in 0..229 {
            for j in 0..229 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF229>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF233 {}
impl PrimeField for GF233 {
    const CHARACTERISTIC : u8 = 233;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,  117,   78,  175,  140,   39,  100,  204,   26,   70,  106,  136,   18,   50,  202,
         102,   96,   13,  184,   35,  111,   53,  152,   68,   28,    9,  164,   25,  225,  101,  218,
          51,  113,   48,   20,  123,   63,   92,    6,  134,  108,  172,  168,  143,  145,   76,  119,
          34,  214,   14,   32,  121,   22,   82,  161,  129,  139,  229,   79,  167,  191,  109,   37,
         142,  190,  173,   80,   24,  206,   10,  128,  178,   83,  148,   87,   46,  115,    3,   59,
          67,  210,   54,   73,   86,  159,   84,   75,  188,  144,  189,  169,   38,  228,  176,  130,
          17,  221,  107,  193,    7,   30,   16,  138,  177,  162,   11,   98,   41,   62,  197,   21,
         181,   33,  186,   77,  231,    2,  156,   47,  200,   52,  212,   36,  171,  192,  135,  222,
          71,   56,   95,  217,  203,  226,   40,  126,   12,  216,  103,   57,    5,  195,   64,   44,
          89,   45,  158,  149,   74,  147,  160,  179,   23,  166,  174,  230,  118,  187,  146,   85,
         150,   55,  105,  223,   27,  209,  153,   60,   43,   91,  196,  124,   42,   66,  154,    4,
          94,  104,   72,  151,  211,  112,  201,  219,   19,  199,  114,  157,   88,   90,   65,   61,
         125,   99,  227,  141,  170,  110,  213,  185,  120,  182,   15,  132,    8,  208,   69,  224,
         205,  165,   81,  180,  122,  198,   49,  220,  137,  131,   31,  183,  215,   97,  127,  163,
         207,   29,  133,  194,   93,   58,  155,  116,  232,    0,    0,    0,    0,    0,    0,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_233 {
    use crate::GF233;
    use crate::prime_field::*;

    #[test]
    fn gf233() {
        let zero = GF233::zero;
        let one = GF233::one;
        for x in GF233::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF233::elts() {
            for y in GF233::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF233::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..234 {
            assert_eq!(x, PrimeFieldElt::from(i % 233));
            x = x + one;
        }
        for i in 0..233 {
            for j in 0..233 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF233>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF239 {}
impl PrimeField for GF239 {
    const CHARACTERISTIC : u8 = 239;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,  120,   80,   60,   48,   40,  205,   30,  186,   24,   87,   20,   92,  222,   16,
          15,  225,   93,  151,   12,  148,  163,   52,   10,  153,   46,   62,  111,   33,    8,   54,
         127,   29,  232,   41,  166,   84,  195,  190,    6,   35,   74,  189,  201,   85,   26,  178,
           5,  200,  196,   75,   23,  230,   31,  113,  175,  130,  136,  158,    4,  192,   27,  129,
         183,  114,  134,  132,  116,   97,  140,  101,   83,  203,   42,   51,  217,  149,   95,  118,
           3,  180,  137,   72,   37,   45,  214,   11,  220,   94,  162,  218,   13,   18,   89,   78,
         122,   69,  100,  169,   98,   71,  157,  181,  131,  173,  115,  172,  135,  182,  176,   28,
         207,   55,   65,  106,   68,  143,   79,  237,    2,  160,   96,  171,  133,  174,  184,   32,
         211,   63,   57,  104,   67,  124,   66,  108,   58,   82,  168,  141,   70,  139,  170,  117,
         161,  150,  221,  226,   21,   77,  145,   19,  228,   25,  194,  202,  167,  102,   59,  236,
         121,  144,   90,   22,  188,  197,   36,  156,  138,   99,  142,  123,  107,  105,  125,   56,
         110,  212,   47,  235,   81,  103,  109,   64,  126,  208,    9,  216,  164,   43,   39,  234,
          61,  213,  154,   38,   50,  165,  204,  233,   49,   44,  155,   73,  198,    7,  210,  112,
         185,  231,  206,  128,  177,  193,   86,  229,  187,   76,   91,  227,   88,  146,   14,  224,
         223,   17,  147,  219,  152,  215,   53,  209,   34,  199,  191,  179,  159,  119,  238,    0,
           0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_239 {
    use crate::GF239;
    use crate::prime_field::*;

    #[test]
    fn gf239() {
        let zero = GF239::zero;
        let one = GF239::one;
        for x in GF239::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF239::elts() {
            for y in GF239::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF239::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..240 {
            assert_eq!(x, PrimeFieldElt::from(i % 239));
            x = x + one;
        }
        for i in 0..239 {
            for j in 0..239 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF239>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF241 {}
impl PrimeField for GF241 {
    const CHARACTERISTIC : u8 = 241;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,  121,  161,  181,  193,  201,   69,  211,  134,  217,   22,  221,  204,  155,  225,
         226,  156,   67,  203,  229,   23,   11,   21,  231,  135,  102,  125,  198,  133,  233,   70,
         113,  168,   78,   62,  154,  228,  222,   68,  235,  194,  132,  213,  126,   75,  131,  200,
         236,  182,  188,   52,   51,  191,  183,  149,   99,  148,  187,  192,  237,  162,   35,   88,
         177,   89,   84,   18,   39,    7,   31,  129,   77,  208,  114,   45,  111,   72,   34,  180,
         238,  122,   97,  151,   66,  224,  227,  205,   63,   65,  158,   98,  186,  184,  100,  137,
         118,   82,   91,   56,   94,  105,   26,  117,  146,  101,  216,  232,  212,  199,  195,   76,
         170,   32,   74,  197,  214,  103,   96,  160,  239,    2,   81,  145,  138,   27,   44,  167,
         209,   71,  165,   46,   42,   29,    9,   25,  140,   95,  124,  215,  136,  147,  185,  150,
         159,  123,  104,  141,   57,   55,  143,   83,  176,  178,   36,   14,   17,  175,   90,  144,
         119,    3,   61,  207,  169,  130,  196,  127,   33,  164,  112,  210,  234,  202,  223,  157,
         152,   64,  153,  206,   79,    4,   49,   54,   93,  142,   92,   58,   50,  190,  189,   53,
          59,    5,   41,  110,  166,  115,   28,  109,   47,    6,  173,   19,   13,   87,  179,  163,
          73,  128,  171,    8,  108,   43,  116,  139,  106,   10,  220,  230,  218,   12,   38,  174,
          85,   15,   16,   86,   37,   20,  219,   24,  107,   30,  172,   40,   48,   60,   80,  120,
         240,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_241 {
    use crate::GF241;
    use crate::prime_field::*;

    #[test]
    fn gf241() {
        let zero = GF241::zero;
        let one = GF241::one;
        for x in GF241::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF241::elts() {
            for y in GF241::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF241::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..242 {
            assert_eq!(x, PrimeFieldElt::from(i % 241));
            x = x + one;
        }
        for i in 0..241 {
            for j in 0..241 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF241>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF251 {}
impl PrimeField for GF251 {
    const CHARACTERISTIC : u8 = 251;
    const DIVISION_TABLE : [u8; 256] = [
           0,    1,  126,   84,   63,  201,   42,   36,  157,   28,  226,  137,   21,   58,   18,   67,
         204,  192,   14,  185,  113,   12,  194,  131,  136,  241,   29,   93,    9,   26,  159,   81,
         102,  213,   96,  208,    7,   95,  218,  103,  182,   49,    6,  216,   97,  106,  191,  235,
          68,   41,  246,   64,  140,   90,  172,  178,  130,  229,   13,  234,  205,  107,  166,    4,
          51,  112,  232,   15,   48,  211,  104,   99,  129,  196,  173,  164,  109,  163,  177,  197,
          91,   31,  150,  124,    3,  189,  108,  176,  174,  110,   53,   80,  221,   27,  243,   37,
          34,   44,  146,   71,  123,  169,   32,   39,   70,  153,   45,   61,   86,   76,   89,  199,
          65,   20,  240,  227,  132,  118,  117,  135,  228,  195,  179,  100,   83,  249,    2,  168,
         151,   72,   56,   23,  116,  134,  133,  119,   24,   11,  231,  186,   52,  162,  175,  165,
         190,  206,   98,  181,  212,  219,   82,  128,  180,  105,  207,  217,  214,    8,  224,   30,
         171,  198,  141,   77,   75,  143,   62,  248,  127,  101,  220,  160,   54,   74,   88,  142,
          87,   78,   55,  122,  152,  147,   40,  203,  236,   19,  139,  200,  247,   85,  144,   46,
          17,  238,   22,  121,   73,   79,  161,  111,  187,    5,  210,  183,   16,   60,  145,  154,
          35,  245,  202,   69,  148,   33,  156,  244,   43,  155,   38,  149,  170,   92,  225,  242,
         158,  222,   10,  115,  120,   57,  239,  138,   66,  237,   59,   47,  184,  233,  193,  230,
         114,   25,  223,   94,  215,  209,   50,  188,  167,  125,  250,    0,    0,    0,    0,    0,
    ];
}
#[cfg(test)]
mod tests_251 {
    use crate::GF251;
    use crate::prime_field::*;

    #[test]
    fn gf251() {
        let zero = GF251::zero;
        let one = GF251::one;
        for x in GF251::elts() {
            assert_eq!(x, x);
            assert_eq!(zero + x, x);
            assert_ne!(one + x, x);
            assert_eq!(x - x, zero);
        }
        for x in GF251::elts() {
            for y in GF251::elts() {
                assert_eq!(x + y, y + x);
                assert_eq!(x * y, y * x);
                assert_eq!(x - y, - (y - x));
            }
        }
        for x in GF251::elts() {
            assert_eq!(x / x, one);
        }
        let mut x = zero;
        for i in 0..252 {
            assert_eq!(x, PrimeFieldElt::from(i % 251));
            x = x + one;
        }
        for i in 0..251 {
            for j in 0..251 {
                if i != j {
                  assert_ne!(PrimeFieldElt::<GF251>::from(i), PrimeFieldElt::from(j));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GF4 {}
impl PrimePowerField for GF4 {
    type FieldOfIntegers = GF2;
    const IRRED_POLY : [PrimeFieldElt<Self::FieldOfIntegers>; 4] =
        [GF2::one,
         GF2::one,
         GF2::one,
         GF2::zero];
    const DEGREE : usize = 3;
}

#[derive(Clone, Copy, Debug)]
pub struct GF9 {}
impl PrimePowerField for GF9 {
    type FieldOfIntegers = GF3;
    const IRRED_POLY : [PrimeFieldElt<Self::FieldOfIntegers>; 4] =
        [GF3::one,
         GF3::zero,
         GF3::one,
         GF3::zero];
    const DEGREE : usize = 3;
}

