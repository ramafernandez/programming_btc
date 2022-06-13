use std::ops::{Add, Div, Mul, Sub};
#[derive(PartialEq, Debug)]
pub enum CreationError {
    NegativeNum,
    BiggerThanPrime,
}

#[derive(PartialEq, Debug)]
pub enum OperationError {
    DifferentPrime,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct FieldElement {
    num: i32,
    prime: i32,
}

pub fn modular_exp(base: i32, exponent: i32, modulus: i32) -> i32 {
    if modulus == 0 {
        return 0;
    }
    let mut c = 1;
    for _ in 0..exponent {
        c = (c * base).rem_euclid(modulus);
    }
    c
}

impl FieldElement {
    pub fn new(num: i32, prime: i32) -> Result<FieldElement, CreationError> {
        if num >= prime {
            return Err(CreationError::BiggerThanPrime);
        }
        match num {
            x if x < 0 => Err(CreationError::NegativeNum),
            x => Ok(FieldElement { num: x, prime }),
        }
    }

    pub fn num(self) -> i32 {
        self.num
    }

    pub fn prime(self) -> i32 {
        self.prime
    }

    pub fn pow(&self, exp: i32) -> Self {
        let n = exp.rem_euclid((self.prime - 1) as i32);    
        let num = modular_exp(self.num, n, self.prime);
        FieldElement {
            num,
            prime: self.prime,
        }
    }
}

impl Add for FieldElement {
    type Output = FieldElement;
    fn add(self, rhs: FieldElement) -> Self::Output {
        assert_eq!(self.prime, rhs.prime);
        FieldElement {
            num: (self.num + rhs.num).rem_euclid(self.prime),
            prime: self.prime,
        }
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;

    fn sub(self, rhs: FieldElement) -> Self::Output {
        assert_eq!(self.prime, rhs.prime);
        FieldElement {
            num: (self.num - rhs.num).rem_euclid(self.prime),
            prime: self.prime,
        }
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: FieldElement) -> Self::Output {
        assert_eq!(self.prime, rhs.prime);
        FieldElement {
            num: (self.num * rhs.num).rem_euclid(self.prime),
            prime: self.prime,
        }
    }
}

impl Div for FieldElement {
    type Output = FieldElement;

    fn div(self, rhs: FieldElement) -> Self::Output {
        assert_eq!(self.prime, rhs.prime);
        assert_ne!(rhs.num, 0);
        FieldElement {
            num: (self.num * modular_exp(rhs.num, self.prime - 2, self.prime))
                .rem_euclid(self.prime),
            prime: self.prime,
        }
    }
}

#[cfg(test)]
mod finite_fields_tests {
    use super::FieldElement;

    #[test]
    fn test_ne() {
        let a = FieldElement { num: 2, prime: 31 };
        let b = FieldElement { num: 2, prime: 31 };
        let c = FieldElement { num: 15, prime: 31 };
        assert_eq!(a, b);
        assert_ne!(a, c);
        //assert!(!(a != b));
    }

    #[test]
    fn test_add() {
        let a = FieldElement { num: 2, prime: 31 };
        let b = FieldElement { num: 15, prime: 31 };
        assert_eq!(a + b, FieldElement { num: 17, prime: 31 });
        let c = FieldElement { num: 17, prime: 31 };
        let d = FieldElement { num: 21, prime: 31 };
        assert_eq!(c + d, FieldElement { num: 7, prime: 31 });
    }

    #[test]
    fn test_sub() {
        let a = FieldElement { num: 29, prime: 31 };
        let b = FieldElement { num: 4, prime: 31 };
        assert_eq!(a - b, FieldElement { num: 25, prime: 31 });
        let c = FieldElement { num: 15, prime: 31 };
        let d = FieldElement { num: 30, prime: 31 };
        assert_eq!(c - d, FieldElement { num: 16, prime: 31 });
    }

    #[test]
    fn test_mul() {
        let a = FieldElement { num: 24, prime: 31 };
        let b = FieldElement { num: 19, prime: 31 };
        assert_eq!(a * b, FieldElement { num: 22, prime: 31 });
    }

    #[test]
    fn test_pow() {
        let a = FieldElement { num: 17, prime: 31 };
        assert_eq!(a.pow(3), FieldElement { num: 15, prime: 31 });
    }

    #[test]
    fn test_div() {
        let a = FieldElement { num: 3, prime: 31 };
        let b = FieldElement { num: 24, prime: 31 };
        assert_eq!(a / b, FieldElement { num: 4, prime: 31 });
    }
}
