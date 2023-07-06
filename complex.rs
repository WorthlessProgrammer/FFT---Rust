use std::ops::*;

#[derive(Debug, Clone, Copy)]
pub struct ComplexNum {
    real: f32,
    img: f32
}

impl ComplexNum {
    pub fn new(x: f32, y: f32) -> ComplexNum {
        ComplexNum{real: x, img: y}
    }

    pub fn conjugate(self) -> ComplexNum {
        Self::new(self.real, -self.img)
    }
}

impl Add for ComplexNum {

    // (a + bi) + (c + di) = (a+c)+(b+d)i 

    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {real: self.real + other.real, img: self.img + other.img}
    }
}

impl AddAssign for ComplexNum {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            real: self.real + other.real,
            img: self.img + other.img,
        };
    }
}

impl Sub for ComplexNum {

    // (a + bi) + (c + di) = (a-c)+(b-d)i 

    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {real: self.real - other.real, img: self.img - other.img}
    }
}

impl Mul for ComplexNum {

    // (a + bi) * (c + di) = (ac-bd)+(ad+bc)i 

    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(
            self.real*other.real - self.img*other.img,
            self.real*other.img + self.img*other.real
        )
    }
}

impl Div for ComplexNum {

    // TODO: Put the formula

    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        let numerator = self*other;
        let denominator = other.real*other.real + other.img*other.img;
        Self::new(numerator.real/denominator, numerator.img/denominator)
    }
}