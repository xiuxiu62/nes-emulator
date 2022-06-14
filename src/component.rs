use num::One;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug)]
pub struct Component<T>(T);

impl<T: Copy + One + Min + Max + Add<Output = T> + Sub<Output = T> + Ord + PartialOrd>
    Component<T>
{
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn get(&self) -> T {
        self.0
    }

    pub fn set(&mut self, value: T) {
        self.0 = value;
    }

    pub fn increment(&mut self) {
        if self.0 < <T as Max>::max() {
            *self += num::one();
        }
    }

    pub fn decrement(&mut self) {
        if self.0 > <T as Min>::min() {
            *self -= num::one();
        }
    }
}

impl<T: Add<Output = T>> Add<T> for Component<T> {
    type Output = T;

    fn add(self, rhs: T) -> Self::Output {
        self.0 + rhs
    }
}

impl<T: Copy + Add<Output = T>> AddAssign<T> for Component<T> {
    fn add_assign(&mut self, rhs: T) {
        *self = Self(self.0 + rhs);
    }
}

impl<T: Sub<Output = T>> Sub<T> for Component<T> {
    type Output = T;

    fn sub(self, rhs: T) -> Self::Output {
        self.0 - rhs
    }
}

impl<T: Copy + Sub<Output = T>> SubAssign<T> for Component<T> {
    fn sub_assign(&mut self, rhs: T) {
        *self = Self(self.0 - rhs);
    }
}

pub trait Min {
    fn min() -> Self;
}

pub trait Max {
    fn max() -> Self;
}

impl Min for u8 {
    fn min() -> u8 {
        u8::MIN
    }
}

impl Max for u8 {
    fn max() -> u8 {
        u8::MAX
    }
}

impl Min for u16 {
    fn min() -> u16 {
        u16::MIN
    }
}

impl Max for u16 {
    fn max() -> u16 {
        u16::MAX
    }
}

#[cfg(test)]
mod tests {
    use crate::Component;

    #[test]
    fn component_members_work() {
        let mut component: Component<u8> = Component::new(0);

        component.set(10);
        (0..10).for_each(|_| component.increment());
        (0..8).for_each(|_| component.decrement());

        assert_eq!(component.get(), 12);
    }

    #[test]
    fn component_cannot_underflow() {
        let mut component: Component<u8> = Component::new(u8::MIN);
        component.decrement();

        assert_eq!(component.get(), u8::MIN)
    }

    #[test]
    fn component_cannot_overflow() {
        let mut component: Component<u8> = Component::new(u8::MAX);
        component.increment();

        assert_eq!(component.get(), u8::MAX)
    }
}
