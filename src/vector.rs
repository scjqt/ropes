use std::{
    convert::From,
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

pub type VecF = Vector<f64>;
pub type VecI = Vector<i32>;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn into<U>(self) -> Vector<U>
    where
        U: From<T>,
    {
        Vector {
            x: self.x.into(),
            y: self.y.into(),
        }
    }

    pub fn length_squared(self) -> T
    where
        T: Copy + Mul<Output = T> + Add<Output = T>,
    {
        self * self
    }

    pub fn dot(self, other: Vector<T>) -> T
    where
        T: Mul<Output = T> + Add<Output = T>,
    {
        self * other
    }

    pub fn perp_dot(self, other: Vector<T>) -> T
    where
        T: Mul<Output = T> + Sub<Output = T>,
    {
        self.x * other.y - self.y * other.x
    }
}

impl<T> Add<Vector<T>> for Vector<T>
where
    T: Add<Output = T>,
{
    type Output = Vector<T>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> AddAssign<Vector<T>> for Vector<T>
where
    T: Add<Output = T> + Copy,
{
    fn add_assign(&mut self, rhs: Vector<T>) {
        *self = *self + rhs;
    }
}

impl<T> Sub<Vector<T>> for Vector<T>
where
    T: Sub<Output = T>,
{
    type Output = Vector<T>;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> SubAssign<Vector<T>> for Vector<T>
where
    T: Sub<Output = T> + Copy,
{
    fn sub_assign(&mut self, rhs: Vector<T>) {
        *self = *self - rhs;
    }
}

impl<T> Mul<T> for Vector<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vector<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output::new(self.x * rhs, self.y * rhs)
    }
}

impl<T> MulAssign<T> for Vector<T>
where
    T: Mul<Output = T> + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl<T> Div<T> for Vector<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Vector<T>;

    fn div(self, rhs: T) -> Self::Output {
        Self::Output::new(self.x / rhs, self.y / rhs)
    }
}

impl<T> DivAssign<T> for Vector<T>
where
    T: Div<Output = T> + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

impl<T> Neg for Vector<T>
where
    T: Neg<Output = T>,
{
    type Output = Vector<T>;

    fn neg(self) -> Self::Output {
        Self::Output::new(self.x.neg(), self.y.neg())
    }
}

impl<T> Mul<Vector<T>> for Vector<T>
where
    T: Mul<Output = T> + Add<Output = T>,
{
    type Output = T;

    fn mul(self, rhs: Vector<T>) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl<T> From<(T, T)> for Vector<T> {
    fn from(tuple: (T, T)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl<T> From<[T; 2]> for Vector<T> {
    fn from(array: [T; 2]) -> Self {
        let [x, y] = array;
        Self { x, y }
    }
}

impl<T> From<Vector<T>> for (T, T) {
    fn from(vector: Vector<T>) -> Self {
        (vector.x, vector.y)
    }
}

impl<T> From<Vector<T>> for [T; 2] {
    fn from(vector: Vector<T>) -> Self {
        [vector.x, vector.y]
    }
}

impl Vector<f64> {
    pub const ZERO: Self = Self { x: 0., y: 0. };
    pub const ONE: Self = Self { x: 1., y: 1. };
    pub const I: Self = Self { x: 1., y: 0. };
    pub const J: Self = Self { x: 0., y: 1. };

    pub fn round(self) -> Vector<i32> {
        Vector {
            x: self.x.round() as i32,
            y: self.y.round() as i32,
        }
    }

    pub fn round_half_down(self) -> Vector<i32> {
        let mut rounded = self.round();
        if self.x.fract() == 0.5 {
            rounded.x -= 1;
        }
        if self.y.fract() == 0.5 {
            rounded.y -= 1;
        }
        rounded
    }

    pub fn round_half_up(self) -> Vector<i32> {
        let mut rounded = self.round();
        if self.x.fract() == -0.5 {
            rounded.x += 1;
        }
        if self.y.fract() == -0.5 {
            rounded.y += 1;
        }
        rounded
    }

    pub fn trunc(self) -> Vector<i32> {
        Vector {
            x: self.x as i32,
            y: self.y as i32,
        }
    }

    pub fn floor(self) -> Vector<i32> {
        Vector {
            x: self.x.floor() as i32,
            y: self.y.floor() as i32,
        }
    }

    pub fn ceil(self) -> Vector<i32> {
        Vector {
            x: self.x.ceil() as i32,
            y: self.y.ceil() as i32,
        }
    }

    pub fn normalise(self) -> Self {
        let length_squared = self * self;
        if length_squared == 0. {
            return self;
        }
        self / length_squared.sqrt()
    }

    pub fn length(self) -> f64 {
        (self * self).sqrt()
    }

    pub fn lerp(self, other: Vector<f64>, t: f64) -> Self {
        Self {
            x: self.x * (1. - t) + other.x * t,
            y: self.y * (1. - t) + other.y * t,
        }
    }
}

impl Vector<i32> {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const ONE: Self = Self { x: 1, y: 1 };
    pub const I: Self = Self { x: 1, y: 0 };
    pub const J: Self = Self { x: 0, y: 1 };
}
