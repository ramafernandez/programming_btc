mod math {

    #[derive(PartialEq, Debug)]
    pub enum CreationError {
        NegativeNum,
        BiggerThanPrime,
    }

    #[derive(PartialEq, Debug)]
    pub enum OperationError {
        DifferentPrime,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct FiniteField {
        num: i32,
        prime: i32,
    }

    fn modular_exp(base: i32, exponent: i32, modulus: i32) -> i32 {
        if modulus == 0 {
            return 0;
        }
        let mut c = 1;
        let tope = exponent - 1;
        for _ in 0..tope {
            c = (c * base).rem_euclid(modulus);
        }
        c
    }

    impl FiniteField {

        pub fn new(num: i32, prime: i32) -> Result<FiniteField, CreationError> {
            if num >= prime {
                Err(CreationError::BiggerThanPrime)
            }else if num < 0 {
                Err(CreationError::NegativeNum)
            } else {
            Ok(FiniteField { num: num, prime: prime })
            }
        }

        pub fn equals(self, other: &Self) -> bool {
            self.num == other.num && self.prime == other.prime
        }

        pub fn print(&self) {
            println!("num = {}", &self.num);
            println!("prime = {}", &self.prime);
        }

        pub fn add(&self, other: &Self) -> Result<Self, OperationError> {
            if self.prime != other.prime {
                return Err(OperationError::DifferentPrime)
            }
            let new = (FiniteField::new((self.num + other.num).rem_euclid(self.prime) ,self.prime)).unwrap(); 
            Ok(new)
        }

        pub fn sub(&self, other: &Self) -> Result<Self, OperationError> {
            if self.prime != other.prime {
                return Err(OperationError::DifferentPrime)
            }
            let new = (FiniteField::new(((self.num - other.num)).rem_euclid(self.prime),self.prime)).unwrap(); 
            Ok(new)
        }
        
        pub fn mul(&self, other: &Self) -> Result<Self, OperationError> {
            if self.prime != other.prime {
                return Err(OperationError::DifferentPrime)
            }
            let new = (FiniteField::new(((self.num * other.num)).rem_euclid(self.prime) ,self.prime)).unwrap(); 
            Ok(new)
        }

        pub fn pow(&self, exp: i32) -> Self{
            let n = exp.rem_euclid((self.prime - 1) as i32);
            let num = modular_exp(self.num, n, self.prime);
            (FiniteField::new(num, self.prime)).unwrap()
        }   

        pub fn div(&self, other: &Self) -> Result<Self, OperationError> {
            if self.prime != other.prime {
                return Err(OperationError::DifferentPrime)
            }
            let num: i32 = (self.num * modular_exp(other.num, self.prime - 2, self.prime)).rem_euclid(self.prime);
            Ok(FiniteField::new(num, self.prime).unwrap())
        }
    }
}

fn main() {

}
    /* 
    fn main() {
        let a = (math::FiniteField::new(33, 97)).unwrap();
        let b = (math::FiniteField::new(34, 97)).unwrap();
        println!("{}", &a.equals(&b));
        println!("{}", &a.equals(&b));
        let _ = &a.print();
        let _ = &b.print();
        let c = &a.add(&b).unwrap();
        let _ = &c.print();
        let d = &a.sub(&b).unwrap();
        let _ = &d.print();
        let e = &c.mul(&d).unwrap();
        let _ = &e.print();
        println!("pase tranqui la multiplicacion");
        let f = &a.pow(-44);
        let _ = &f.print();
        println!("pude pasar las pruebas de potencia");
        let g = &a.div(&f).unwrap();    
        let _ = &g.print();
    } */