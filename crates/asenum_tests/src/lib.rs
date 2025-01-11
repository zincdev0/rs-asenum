//! Small derive macro to safely convert the numerical value of an enum to the enum.

#![no_std]

use asenum::Convert;

#[derive(Convert, Debug, Eq, PartialEq)]
enum SomeEnum {
    Lit = 1,
    Big = 2,
}

#[test]
fn from_value() {
    assert_eq!(SomeEnum::from_value(1), Some(SomeEnum::Lit));
    assert_eq!(SomeEnum::from_value(2), Some(SomeEnum::Big));
    assert_eq!(SomeEnum::from_value(3), None);
}
