use std::ops::*;

#[derive(Debug, Clone, Copy)]
pub struct ComplexNum {
    pub real: f32,
    pub img: f32
}

impl ComplexNum {
    pub fn new(x: f32, y: f32) -> ComplexNum {
        ComplexNum{real: x, img: y}
    }

    pub fn conjugate(self) -> ComplexNum {
        Self::new(self.real, -self.img)
    }

    pub fn i() -> ComplexNum {
        Self::new(0.0, 1.0)
    }
}

impl Add for ComplexNum {

    // (a + bi) + (c + di) = (a+c)+(b+d)i 

    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {real: self.real + other.real, img: self.img + other.img}
    }
}

impl Add<i32> for ComplexNum {
    type Output = Self;

    fn add(self, other: i32) -> Self {
        Self {real: self.real + other as f32, img: self.img}
    }
}

impl Add<f32> for ComplexNum {
    type Output = Self;

    fn add(self, other: f32) -> Self {
        Self {real: self.real + other, img: self.img}
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

impl Mul<f32> for ComplexNum {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {real: self.real * other, img: self.img * other}
    }
}

impl Div for ComplexNum {

    // TODO: Put the formula

    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        let numerator = self*other;
        let denominator = other.real*other.real + other.img*other.img;
        //TODO: Handle 0 div error.
        Self::new(numerator.real/denominator, numerator.img/denominator)
    }
}

//functions 

pub fn exp(base: f32, power: ComplexNum) -> ComplexNum {

    // a^(b + ci) = a^b(cos(clna) + sin(clna)i)

    ComplexNum::new((base.ln()*power.img).cos(), (base.ln()*power.img).sin()) * base.powf(power.real)
}
