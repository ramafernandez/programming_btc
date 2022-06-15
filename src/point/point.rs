use crate::finite_fields::campos_finitos::*;
use core::panic;
use std::{
    ops::{Add, AddAssign, Mul},
    result,
};

#[derive(PartialEq, Debug)]
pub enum CreationError {
    NotInCurve,
}

/* use std::FieldElement::INFINITY; */

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Point {
    a: FieldElement,
    b: FieldElement,
    x: Option<FieldElement>,
    y: Option<FieldElement>,
}

impl Point {
    pub fn new(
        x: Option<FieldElement>,
        y: Option<FieldElement>,
        a: FieldElement,
        b: FieldElement,
    ) -> Result<Point, CreationError> {
        match x {
            None => Ok(Point {
                a,
                b,
                x: None,
                y: None,
            }),
            Some(num) => {
                if y.unwrap().num().pow(2) != num.num().pow(3) + a.num() * num.num() + b.num() {
                    Ok(Point { a, b, x, y })
                } else {
                    Err(CreationError::NotInCurve)
                }
            }
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        assert_eq!(
            self.a.num(),
            rhs.a.num(),
            "los puntos no estan en la misma curva"
        );
        assert_eq!(
            self.b.num(),
            rhs.b.num(),
            "los puntos no estan en la misma curva"
        );
        //si uno de los dos esta en el infinito devuelvo el otro
        if self.x == None {
            return rhs;
        }
        if rhs.x == None {
            return self;
        }

        let selfx: FieldElement;

        match self.x {
            Some(a) => selfx = a,
            None => panic!("punto en el infinito aaa"),
        }

        let selfy: FieldElement;

        match self.y {
            Some(a) => selfy = a,
            None => panic!("punto en el infinito aaa"),
        }

        let rhsx: FieldElement;

        match rhs.x {
            Some(a) => rhsx = a,
            None => panic!("punto en el infinito aaa"),
        }

        let rhsy: FieldElement;

        match rhs.y {
            Some(a) => rhsy = a,
            None => panic!("punto en el infinito aaa"),
        }

        //si tienen el mismo x pero distinto y, la suma esta en el infinito (la recta que cruza a ambos no esta en la curva)
        if selfx == rhsx && selfy != rhsy {
            return Point {
                a: self.a,
                b: self.b,
                x: None, //la implementacion de infinite solo existe en float, usamos None
                y: None,
            };
        }

        //tienen diferente x
        if selfx != rhsx {
            let s = (rhsy - selfy) / (rhsx - selfx);
            let x = s.pow(2) - selfx - rhsx;
            let y = s * (selfx - x) - selfy;
            return Point {
                a: self.a,
                b: self.b,
                x: Some(x),
                y: Some(y),
            };
        }

        let two: FieldElement;
        match FieldElement::new(2, self.a.prime()) {
            Ok(a) => two = a,
            Err(e) => panic!("aaaaaaa"),
        };

        let three: FieldElement;
        match FieldElement::new(3, self.a.prime()) {
            Ok(a) => three = a,
            Err(e) => panic!("aaaaaaa"),
        };

        //si los puntos son iguales
        if self == rhs {
            let s = (three * (selfx).pow(2) + self.a) / (two * selfy); //pendiente: le pongo un fieldElement de valor num 2?
            let x = s.pow(2) - two * selfx; //misma duda que arriba
            let y = s * (selfx - x) - selfy;
            return Point {
                a: self.a,
                b: self.b,
                x: Some(x),
                y: Some(y),
            };
        }
        self
    }
}

impl Mul<Point> for i32 {
    type Output = Point;
    fn mul(self, rhs: Point) -> Self::Output {
        if self == 0 {
            return Point {
                a: rhs.a,
                b: rhs.b,
                x: None,
                y: None,
            };
        }
        let mut result: Point = Point {
            a: rhs.a,
            b: rhs.b,
            x: None,
            y: None,
        };
        let mut s = self.clone();
        let mut actual = rhs.clone();
        while s != 0 {
            if (s & 1) == true as i32 {
                result = result + actual;
            }
            actual = actual + actual; //ya se, queda implementar el AddAssign
            s = s >> 1;
        }
        result
    }
}

#[cfg(test)]
mod point_tests {
    use super::Point;
    use crate::finite_fields::campos_finitos::FieldElement;

    #[test]
    fn test_on_curve() {
        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();
        let x = Some(FieldElement::new(192, prime).unwrap());
        let y = Some(FieldElement::new(105, prime).unwrap());
        assert!(Point::new(x, y, a, b).is_ok());
    }

    #[test]
    fn test_add0() {
        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();
        let x1 = Some(FieldElement::new(192, prime).unwrap());
        let y1 = Some(FieldElement::new(105, prime).unwrap());
        let x2 = Some(FieldElement::new(17, prime).unwrap());
        let y2 = Some(FieldElement::new(56, prime).unwrap());
        let x3 = Some(FieldElement::new(170, prime).unwrap());
        let y3 = Some(FieldElement::new(142, prime).unwrap());

        let p1 = Point::new(x1, y1, a, b).unwrap();
        let p2 = Point::new(x2, y2, a, b).unwrap();
        let p3 = Point::new(x3, y3, a, b).unwrap();
        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn test_add1() {
        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();
        let x1 = Some(FieldElement::new(47, prime).unwrap());
        let y1 = Some(FieldElement::new(71, prime).unwrap());
        let x2 = Some(FieldElement::new(117, prime).unwrap());
        let y2 = Some(FieldElement::new(141, prime).unwrap());
        let x3 = Some(FieldElement::new(60, prime).unwrap());
        let y3 = Some(FieldElement::new(139, prime).unwrap());

        let p1 = Point::new(x1, y1, a, b).unwrap();
        let p2 = Point::new(x2, y2, a, b).unwrap();
        let p3 = Point::new(x3, y3, a, b).unwrap();
        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn test_add2() {
        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();
        let x1 = Some(FieldElement::new(143, prime).unwrap());
        let y1 = Some(FieldElement::new(98, prime).unwrap());
        let x2 = Some(FieldElement::new(76, prime).unwrap());
        let y2 = Some(FieldElement::new(66, prime).unwrap());
        let x3 = Some(FieldElement::new(47, prime).unwrap());
        let y3 = Some(FieldElement::new(71, prime).unwrap());

        let p1 = Point::new(x1, y1, a, b).unwrap();
        let p2 = Point::new(x2, y2, a, b).unwrap();
        let p3 = Point::new(x3, y3, a, b).unwrap();
        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn test_mul0() {
        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();
        let x1 = Some(FieldElement::new(192, prime).unwrap());
        let y1 = Some(FieldElement::new(105, prime).unwrap());
        let x2 = Some(FieldElement::new(49, prime).unwrap());
        let y2 = Some(FieldElement::new(71, prime).unwrap());
        let p1 = Point::new(x1, y1, a, b).unwrap();
        let p2 = Point::new(x2, y2, a, b).unwrap();
        assert_eq!(2 * p1, p2);
    }

    #[test]
    fn test_mul1() {
        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();
        let x1 = Some(FieldElement::new(47, prime).unwrap());
        let y1 = Some(FieldElement::new(71, prime).unwrap());
        let x2 = Some(FieldElement::new(116, prime).unwrap());
        let y2 = Some(FieldElement::new(55, prime).unwrap());
        let p1 = Point::new(x1, y1, a, b).unwrap();
        let p2 = Point::new(x2, y2, a, b).unwrap();
        assert_eq!(8 * p1, p2);
    }
}
