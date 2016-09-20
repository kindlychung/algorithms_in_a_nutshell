/// A Z^m number system includes integers in the interval [0, m) when m > 0 or (m, 0] when m < 0. 
/// The code defines a trait Mod to represent numbers in this system and the +, -, and * operator for it.
use std::ops::{Add, Mul, Sub, Rem};

trait Modulo<Rhs = Self> {
    type Output;

    fn modulo(self, n: Rhs) -> Self::Output;
}

impl<T> Modulo<T> for T
    where T: Add<Output = T> + Rem<Output = T> + Copy
{
    type Output = T;

    fn modulo(self, n: T) -> T {
        ((self % n) + n) % n
    }
}


// Give the combination of these traits a name!
trait ZMod:  Modulo<Output = Self> + Mul<Output = Self> + Sub<Output = Self> + Add<Output = Self> + Rem<Output = Self> + Copy {}
impl<T> ZMod for T
    where T: Modulo<Output = T>    + Mul<Output = T>    + Sub<Output = T>    + Add<Output = T>    + Rem<Output = T>    + Copy
{}


struct Mod<T> {
    modulo: T,
    i: T,
}

impl<T> Mod<T>
    where T: ZMod
{
    fn new(modulo: T, i: T) -> Mod<T> {
        Mod {
            modulo: modulo,
            i: i.modulo(modulo),
        }
    }
}

impl<T> Add for Mod<T>
    where T: ZMod
{
    type Output = Mod<T>;
    fn add(self, other: Mod<T>) -> Mod<T> {
        Mod::new(self.modulo, self.i + other.i)
    }
}

impl<T> Sub for Mod<T>
    where T: ZMod
{
    type Output = Mod<T>;
    fn sub(self, other: Mod<T>) -> Mod<T> {
        Mod::new(self.modulo, self.i - other.i)
    }
}

impl<T> Mul for Mod<T>
    where T: ZMod
{
    type Output = Mod<T>;
    fn mul(self, other: Mod<T>) -> Mod<T> {
        Mod::new(self.modulo, self.i * other.i)
    }
}

#[test]
fn t0() {
    let x = Mod::new(-5i8, 3i8);
    let y = Mod::new(-5i8, 8i8);
    assert_eq!(-4, (x + y).i);
}

#[test]
fn t1() {
    let x = Mod::new(-5i8, 3i8);
    let y = Mod::new(-5i8, 8i8);
    assert_eq!(-4, (x + y).i);
}


#[test]
fn t2() {
    let x = Mod::new(-5i8, 3i8);
    let y = Mod::new(-5i8, 8i8);
    assert_eq!(0, (x - y).i);
}

#[test]
fn t3() {
    let x = Mod::new(-5i8, 3i8);
    let y = Mod::new(-5i8, 8i8);
    assert_eq!(-1, (x * y).i);
}

#[test]
fn t4() {
    let x = Mod::new(-5i16, 3i16);
    let y = Mod::new(-5i16, 8i16);
    assert_eq!(-4, (x + y).i);
}

#[test]
fn t5() {
    let x = Mod::new(-5i16, 3i16);
    let y = Mod::new(-5i16, 8i16);
    assert_eq!(0, (x - y).i);
}

#[test]
fn t6() {
    let x = Mod::new(-5i16, 3i16);
    let y = Mod::new(-5i16, 8i16);
    assert_eq!(-1, (x * y).i);
}

#[test]
fn t7() {
    let x = Mod::new(-5, 3);
    let y = Mod::new(-5, 8);
    assert_eq!(-4, (x + y).i);
}

#[test]
fn t8() {
    let x = Mod::new(-5, 3);
    let y = Mod::new(-5, 8);
    assert_eq!(0, (x - y).i);
}

#[test]
fn t9() {
    let x = Mod::new(-5, 3);
    let y = Mod::new(-5, 8);
    assert_eq!(-1, (x * y).i);
}

#[test]
fn t10() {
    let x = Mod::new(5u8, 3u8);
    let y = Mod::new(5u8, 8u8);
    assert_eq!(1, (x + y).i);
}

#[test]
fn t11() {
    let x = Mod::new(5u8, 3u8);
    let y = Mod::new(5u8, 8u8);
    assert_eq!(0, (x - y).i);
}

#[test]
fn t12() {
    let x = Mod::new(5u8, 3u8);
    let y = Mod::new(5u8, 8u8);
    assert_eq!(4, (x * y).i);
}