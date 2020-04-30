use std::cmp;
use std::ops;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug)]
pub struct GF2 {}
#[derive(Clone, Copy, Debug)]
pub struct GF3 {}
#[derive(Clone, Copy, Debug)]
pub struct GF5 {}

trait PrimeField: std::marker::Sized + std::fmt::Debug + std::marker::Copy {
    const P: u8;

    const DIVISION_TABLE: [u8; 256];

    fn zero() -> PrimeFieldElt<Self> {
        PrimeFieldElt {
            val : 0,
            phantom : PhantomData,
        }
    }

    fn one() -> PrimeFieldElt<Self> {
        PrimeFieldElt {
            val : 1,
            phantom : PhantomData,
        }
    }

    fn elts() -> std::iter::Scan<std::ops::Range<u8>,
                                 PrimeFieldElt<Self>,
                                 fn(&mut PrimeFieldElt<Self>, u8) -> Option<PrimeFieldElt<Self>>
                            > {
        (0..Self::P).scan(Self::zero(), |acc, _| Some(*acc + Self::one()))
    }
}

impl PrimeField for GF2 {
    const P : u8 = 2;
    const DIVISION_TABLE : [u8; 256] = [
        0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
}

impl PrimeField for GF3 {
    const P : u8 = 3;
    const DIVISION_TABLE : [u8; 256] = [
        0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
}

impl PrimeField for GF5 {
    const P : u8 = 5;
    const DIVISION_TABLE : [u8; 256] = [
        0, 1, 3, 2, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
}

#[derive(Clone, Copy, Debug)]
struct PrimeFieldElt<F : PrimeField> {
    val: u8,
    phantom: PhantomData<F>,
}

impl<F: PrimeField> ops::Add for PrimeFieldElt<F> {
    type Output = PrimeFieldElt<F>;

    fn add(self, rhs: PrimeFieldElt<F>) -> PrimeFieldElt<F> {
        PrimeFieldElt {
            val : (((self.val as u16) + (rhs.val as u16) ) % (F::P as u16)) as u8,
            phantom: PhantomData,
        }
    }
}

impl<F: PrimeField> ops::Neg for PrimeFieldElt<F> {
    type Output = PrimeFieldElt<F>;

    fn neg(self) -> PrimeFieldElt<F> {
        PrimeFieldElt {
            val : (F::P - self.val) % F::P,
            phantom: PhantomData,
        }
    }
}

impl<F: PrimeField> ops::Sub for PrimeFieldElt<F> {
    type Output = PrimeFieldElt<F>;

    fn sub(self, rhs : PrimeFieldElt<F>) -> PrimeFieldElt<F> {
        self + (-rhs)
    }
}

impl<F: PrimeField> ops::Mul for PrimeFieldElt<F> {
    type Output = PrimeFieldElt<F>;

    fn mul(self, rhs: PrimeFieldElt<F>) -> PrimeFieldElt<F> {
        PrimeFieldElt {
            val : (((self.val as u16) * (rhs.val as u16) ) % (F::P as u16)) as u8,
            phantom: PhantomData,
        }
    }
}

impl<F: PrimeField> ops::Div for PrimeFieldElt<F> {
    type Output = PrimeFieldElt<F>;

    fn div(self, rhs: PrimeFieldElt<F>) -> PrimeFieldElt<F> {
        assert_ne!(rhs, F::zero(), "Division by zero");
        self * PrimeFieldElt {
            val : F::DIVISION_TABLE[rhs.val as usize],
            phantom: PhantomData,
        }
    }
}

impl<F: PrimeField> cmp::PartialEq for PrimeFieldElt<F> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl<F: PrimeField> cmp::Eq for PrimeFieldElt<F> {}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn gf2() {
        let zero = GF2::zero();
        let one = GF2::one();
        assert_eq!(zero + zero, zero);
        assert_eq!(zero + one, one);
        assert_eq!(zero - one, one);
        assert_eq!(one + one, zero);
        assert_eq!(one - one, zero);
        assert_eq!(one * one, one);
        assert_eq!(zero * one, zero);
        assert_eq!(one / one, one);
        assert_eq!(zero / one, zero);

        for x in GF2::elts() {
            if x != GF2::zero() {
                assert_eq!(x / x, GF2::one());
            }
        }
    }

    #[test]
    fn gf3() {
        let zero = GF3::zero();
        let one = GF3::one();
        assert_eq!(zero + zero, zero);
        assert_eq!(zero + one, one);
        assert_ne!(zero - one, one);
        assert_ne!(one + one, zero);
        assert_eq!(one - one, zero);
        assert_eq!(one * one, one);
        assert_eq!(zero * one, zero);
        assert_eq!(one / one, one);
        assert_eq!(zero / one, zero);

        for x in GF3::elts() {
            if x != GF3::zero() {
                assert_eq!(x / x, GF3::one());
            }
        }
    }
}
