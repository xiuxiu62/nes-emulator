use num::{
    traits::{WrappingAdd, WrappingSub},
    One,
};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Default)]
#[repr(transparent)]
pub struct SubComponent<T>(T);

impl<T> SubComponent<T>
where
    T: Copy
        + One
        + Min
        + Max
        + Ord
        + Add<Output = T>
        + Sub<Output = T>
        + WrappingAdd<Output = T>
        + WrappingSub<Output = T>,
{
    #[inline]
    pub fn new(value: T) -> Self {
        Self(value)
    }

    #[inline]
    pub fn get(&self) -> T {
        self.0
    }

    #[inline]
    pub fn set(&mut self, value: T) {
        self.0 = value;
    }

    #[inline]
    pub fn increment(&mut self) {
        self.wrapping_add(num::one());
    }

    #[inline]
    pub fn decrement(&mut self) {
        self.wrapping_sub(num::one());
    }

    #[inline]
    pub fn wrapping_add(&mut self, value: T) {
        self.set(self.0.wrapping_add(&value));
    }

    #[inline]
    pub fn wrapping_sub(&mut self, value: T) {
        self.set(self.0.wrapping_sub(&value));
    }
    #[inline]
    pub fn reset(&mut self) {
        self.set(<T as Min>::min())
    }
}

impl<T: Add<Output = T>> Add<T> for SubComponent<T> {
    type Output = T;

    fn add(self, rhs: T) -> Self::Output {
        self.0 + rhs
    }
}

impl<T: Copy + Add<Output = T>> AddAssign<T> for SubComponent<T> {
    fn add_assign(&mut self, rhs: T) {
        *self = Self(self.0 + rhs);
    }
}

impl<T: Sub<Output = T>> Sub<T> for SubComponent<T> {
    type Output = T;

    fn sub(self, rhs: T) -> Self::Output {
        self.0 - rhs
    }
}

impl<T: Copy + Sub<Output = T>> SubAssign<T> for SubComponent<T> {
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
    use super::SubComponent;

    #[test]
    fn sub_component_members_work() {
        let mut sub_component = SubComponent::new(0_u8);

        sub_component.set(10);
        (0..10).for_each(|_| sub_component.increment());
        (0..8).for_each(|_| sub_component.decrement());

        assert_eq!(sub_component.get(), 12);
    }

    #[test]
    fn sub_component_can_underflow() {
        let mut sub_component = SubComponent::new(u8::MIN);
        sub_component.decrement();

        assert_eq!(sub_component.get(), u8::MAX)
    }

    #[test]
    fn sub_component_can_overflow() {
        let mut sub_component = SubComponent::new(u8::MAX);
        sub_component.increment();

        assert_eq!(sub_component.get(), u8::MIN)
    }
}
