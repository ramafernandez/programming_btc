use std::ops::Add;

#[derive(PartialEq, Debug)]
pub enum CreationError {
    NotInCurve,
}

use std::f32::INFINITY;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Point {
    a: f32,
    b: f32,
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, a: f32, b: f32) -> Result<Point, CreationError> {
        if y.powf(2.0) != x.powf(3.0) + a * x + b {
            return Err(CreationError::NotInCurve);
        }
        Ok(Point { a, b, x, y })
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        assert_eq!(self.a, rhs.a, "los puntos no estan en la misma curva");
        assert_eq!(self.b, rhs.b, "los puntos no estan en la misma curva");
        //si uno de los dos esta en el infinito devuelvo el otro
        if self.x == INFINITY {
            return rhs;
        }
        if rhs.x == INFINITY {
            return self;
        }

        //si tienen el mismo x pero distinto y, la suma esta en el infinito (la recta que cruza a ambos no esta en la curva)
        if self.x == rhs.x && self.y != rhs.y {
            return Point {
                a: self.a,
                b: self.b,
                x: INFINITY,
                y: INFINITY,
            };
        }

        //tienen diferente x
        if self.x != rhs.x {
            let s = (rhs.y - self.y) / (rhs.x - self.x);
            let x = s.powf(2_f32) - self.x - rhs.x;
            let y = s * (self.x - x) - self.y;
            return Point {
                a: self.a,
                b: self.b,
                x,
                y,
            };
        }

        //si los puntos son iguales
        if self == rhs {
            let s = (3.0 * (self.x).powf(2.0) + self.a) / (2.0 * self.y); //pendiente
            let x = s.powf(2.0) - 2.0 * self.x;
            let y = s * (self.x - x) - self.y;
            return Point {
                a: self.a,
                b: self.b,
                x,
                y,
            };
        }
        self
    }
}

#[cfg(test)]
mod point_tests {
    use std::f32::INFINITY;

    use super::Point;

    #[test]
    fn test_ne() {
        let a = Point {
            a: 5.0,
            b: 7.0,
            x: 3.0,
            y: -7.0,
        };
        let b = Point {
            a: 5.0,
            b: 7.0,
            x: 18.0,
            y: 77.0,
        };
        assert_ne!(a, b);
        assert_eq!(a, a);
    }

    #[test]
    fn test_add0() {
        let a = Point {
            a: 5.0,
            b: 7.0,
            x: INFINITY,
            y: INFINITY,
        };
        let b = Point {
            a: 5.0,
            b: 7.0,
            x: 2.0,
            y: 5.0,
        };
        let c = Point {
            a: 5.0,
            b: 7.0,
            x: 2.0,
            y: -5.0,
        };

        assert_eq!(a + b, b);
        assert_eq!(b + a, b);
        assert_eq!(b + c, a);
    }

    #[test]
    fn test_add1() {
        let a = Point {
            a: 5.0,
            b: 7.0,
            x: 3.0,
            y: 7.0,
        };
        let b = Point {
            a: 5.0,
            b: 7.0,
            x: -1.0,
            y: -1.0,
        };
        assert_eq!(
            a + b,
            Point {
                a: 5.0,
                b: 7.0,
                x: 2.0,
                y: -5.0
            }
        );
    }

    #[test]
    fn test_add2() {
        let a = Point {
            a: 5.0,
            b: 7.0,
            x: -1.0,
            y: -1.0,
        };
        assert_eq!(
            a + a,
            Point {
                a: 5.0,
                b: 7.0,
                x: 18.0,
                y: 77.0
            }
        );
    }
}
