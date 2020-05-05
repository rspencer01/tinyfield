use core::ops;
use core::cmp;
use core::marker;
use core::convert::TryInto;
use core::fmt;
use crate::prime_field::{GF2, GF3, PrimeField, PrimeFieldElt};


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

pub struct PrimePowerFieldElementGenerator<F : PrimePowerField> {
    next : [PrimeFieldElt<F::FieldOfIntegers> ; 4],
    start: bool
}

impl<F : PrimePowerField> Iterator for PrimePowerFieldElementGenerator<F> {
    type Item = PrimePowerFieldElt<F>;

    fn next(&mut self) -> Option<PrimePowerFieldElt<F>> {
        if self.next.iter().all(|x| *x == F::FieldOfIntegers::zero) && !self.start {
            return None;
        }
        self.start = false;
        let to_ret = self.next;
        for i in 0..F::DEGREE-1 {
            self.next[i] = self.next[i] + F::FieldOfIntegers::one;
            if self.next[i] != F::FieldOfIntegers::zero {
                break
            }
        }
        Some(PrimePowerFieldElt {
            val: to_ret,
            phantom: marker::PhantomData
        })
    }
}

pub trait PrimePowerField : marker::Sized + core::fmt::Debug + marker::Copy {
    type FieldOfIntegers : PrimeField;
    const IRRED_POLY : [PrimeFieldElt<Self::FieldOfIntegers>; 4];
    const DEGREE : usize;

    const zero : PrimePowerFieldElt<Self> =
        PrimePowerFieldElt {
            val : [Self::FieldOfIntegers::zero ; 4],
            phantom : marker::PhantomData,
        };

    const one : PrimePowerFieldElt<Self> =
        PrimePowerFieldElt {
            val : [Self::FieldOfIntegers::one,
                   Self::FieldOfIntegers::zero,
                   Self::FieldOfIntegers::zero,
                   Self::FieldOfIntegers::zero],
            phantom : marker::PhantomData,
        };

    fn elts() -> PrimePowerFieldElementGenerator<Self> {
        PrimePowerFieldElementGenerator {
            next: [Self::FieldOfIntegers::zero ; 4],
            start: true,
        }
    }
}


#[derive(Clone, Copy)]
pub struct PrimePowerFieldElt<F : PrimePowerField> {
    val: [PrimeFieldElt<F::FieldOfIntegers>; 4],
    phantom: marker::PhantomData<F>,
}

impl<F: PrimePowerField> From<u8> for PrimePowerFieldElt<F> {
    fn from(x : u8) -> PrimePowerFieldElt<F> {
        PrimePowerFieldElt {
            val : [ PrimeFieldElt::from(x)
            , F::FieldOfIntegers::zero, F::FieldOfIntegers::zero, F::FieldOfIntegers::zero],
            phantom: marker::PhantomData
        }
    }
}

impl<F: PrimePowerField> ops::Add for PrimePowerFieldElt<F> {
    type Output = PrimePowerFieldElt<F>;

    fn add(self, rhs: PrimePowerFieldElt<F>) -> PrimePowerFieldElt<F> {
        PrimePowerFieldElt {
            val: [self.val[0] + rhs.val[0],
                  self.val[1] + rhs.val[1],
                  self.val[2] + rhs.val[2],
                  self.val[3] + rhs.val[3]],
            phantom: marker::PhantomData,
        }
    }
}

impl<F: PrimePowerField> ops::Neg for PrimePowerFieldElt<F> {
    type Output = PrimePowerFieldElt<F>;

    fn neg(self) -> PrimePowerFieldElt<F> {
        PrimePowerFieldElt {
            val: [-self.val[0],
                  -self.val[1],
                  -self.val[2],
                  -self.val[3]],
            phantom: marker::PhantomData,
        }
    }
}

impl<F: PrimePowerField> ops::Sub for PrimePowerFieldElt<F> {
    type Output = PrimePowerFieldElt<F>;

    fn sub(self, rhs : PrimePowerFieldElt<F>) -> PrimePowerFieldElt<F> {
        self + (-rhs)
    }
}

impl<F: PrimePowerField> ops::Mul for PrimePowerFieldElt<F> {
    type Output = PrimePowerFieldElt<F>;

    fn mul(self, rhs: PrimePowerFieldElt<F>) -> PrimePowerFieldElt<F> {
        let mut prod_poly = [F::FieldOfIntegers::zero; 8];
        for i in 0..4 {
            for j in 0..4 {
                prod_poly[i + j] = prod_poly[i + j] + self.val[i] * rhs.val[j]
            }
        }
        for i in (F::DEGREE-1..8).rev() {
            let c = prod_poly[i];
            for j in 0..F::DEGREE {
                prod_poly[i - j] = prod_poly[i - j] - c * F::IRRED_POLY[F::DEGREE-j-1];
            }
        }
        for i in 4..8 {
            assert_eq!(prod_poly[i], F::FieldOfIntegers::zero);
        }
        PrimePowerFieldElt {
            val: prod_poly[0..4].try_into().unwrap(),
            phantom: marker::PhantomData,
        }
    }
}

impl<F: PrimePowerField> ops::Div for PrimePowerFieldElt<F> {
    type Output = PrimePowerFieldElt<F>;

    fn div(self, rhs: PrimePowerFieldElt<F>) -> PrimePowerFieldElt<F> {
        assert_ne!(rhs, F::zero, "Division by zero");
        // TODO(robert) write this properly
        self * F::elts().filter(|x| *x * rhs == F::one).next().expect("Could not find inverse")
    }
}

impl<F: PrimePowerField> PrimePowerFieldElt<F> {
    pub fn pow(self, rhs: u8) -> PrimePowerFieldElt<F> {
        // let rhs = rhs % (F::FieldOfIntegers::CHARACTERISTIC.pow(F::DEGREE as u32 - 1) - 1);
        if rhs == 0 {
            F::one
        } else {
            self * self.pow(rhs - 1)
        }
    }
}


impl<F: PrimePowerField> cmp::PartialEq for PrimePowerFieldElt<F> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl<F: PrimePowerField> cmp::Eq for PrimePowerFieldElt<F> {}

impl<F: PrimePowerField> fmt::Debug for PrimePowerFieldElt<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("({:?},{:?},{:?},{:?})", self.val[0], self.val[1], self.val[2], self.val[3]))
    }
}

#[cfg(test)]
mod tests {
    use crate::prime_power_field::*;

    #[test]
    fn gf4() {
        let zero = GF4::zero;
        let one = GF4::one;
        assert_eq!(zero + zero, zero);
        assert_eq!(zero + one, one);
        assert_eq!(zero - one, one);
        assert_eq!(one + one, zero);
        assert_eq!(one - one, zero);
        assert_eq!(one * one, one);
        assert_eq!(zero * one, zero);
        for x in GF4::elts() {
            if x != GF4::zero {
                assert_eq!(x / x, GF4::one);
            }
        }
    }

    #[test]
    fn gf9() {
        let zero = GF9::zero;
        let one = GF9::one;
        assert_eq!(zero + zero, zero);
        assert_eq!(zero + one, one);
        assert_ne!(zero - one, one);
        assert_ne!(one + one, zero);
        assert_eq!(one - one, zero);
        assert_eq!(one * one, one);
        assert_eq!(zero * one, zero);
        for x in GF9::elts() {
            if x != GF9::zero {
                assert_eq!(x / x, GF9::one);
            }
        }
    }
}
