// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

#[derive(Debug)]
pub struct SaturatingU16 {
    pub value: u16,
}

impl SaturatingU16 {
    pub fn new(value: u16) -> SaturatingU16 {
        SaturatingU16 { value }
    }
}

impl Into<SaturatingU16> for u16 {
    fn into(self) -> SaturatingU16 {
        SaturatingU16::new(self)
    }
}

impl Into<SaturatingU16> for u8 {
    fn into(self) -> SaturatingU16 {
        SaturatingU16::new(self.into())
    }
}

impl Into<SaturatingU16> for &u16 {
    fn into(self) -> SaturatingU16 {
        SaturatingU16::new(*self)
    }
}

impl Into<SaturatingU16> for &u8 {
    fn into(self) -> SaturatingU16 {
        let converted = *self;
        SaturatingU16::new(converted.into())
    }
}
