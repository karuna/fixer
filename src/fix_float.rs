use crate::field::{FieldValue, FieldValueReader, FieldValueWriter};

// FIXFloat is a FIX Float Value, implements FieldValue
pub type FIXFloat = f64;

pub trait FIXFloatTrait {
    fn float64(&self) -> f64;
}

impl FIXFloatTrait for FIXFloat {
    fn float64(&self) -> f64 {
        *self
    }
}

impl FieldValueReader for FIXFloat {
    fn read(&mut self, input: &[u8]) -> Result<(), ()> {
        let f = fast_float::parse(input).map_err(|_| ())?;

        for chr in input.iter() {
            if *chr != '.' as u8 && *chr != '-' as u8 && !('0' as u8..='9' as u8).contains(chr) {
                return Err(());
            }
        }

        *self = f;

        Ok(())
    }
}

impl FieldValueWriter for FIXFloat {
    fn write(&self) -> Vec<u8> {
        format!("{}", self).into_bytes()
    }
}

impl FieldValue for FIXFloat {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_float_write() {
        struct TestCase<'a> {
            field: FIXFloat,
            val: &'a str,
        }
        let tests = vec![TestCase {
            field: 5.0,
            val: "5",
        }];
        for test in tests.iter() {
            let b = test.field.write();
            assert_eq!(b, test.val.as_bytes(), "got {:?}; want {}", b, test.val);
        }
    }

    #[test]
    fn test_float_read() {
        struct TestCase<'a> {
            bytes: &'a [u8],
            value: f64,
            expect_error: bool,
        }
        let tests = vec![
            TestCase {
                bytes: "15".as_bytes(),
                value: 15.0,
                expect_error: false,
            },
            TestCase {
                bytes: "99.9".as_bytes(),
                value: 99.9,
                expect_error: false,
            },
            TestCase {
                bytes: "0.00".as_bytes(),
                value: 0.0,
                expect_error: false,
            },
            TestCase {
                bytes: "-99.9".as_bytes(),
                value: -99.9,
                expect_error: false,
            },
            TestCase {
                bytes: "-99.9.9".as_bytes(),
                value: 0.0,
                expect_error: true,
            },
            TestCase {
                bytes: "blah".as_bytes(),
                value: 0.0,
                expect_error: true,
            },
            TestCase {
                bytes: "1.a1".as_bytes(),
                value: 0.0,
                expect_error: true,
            },
            TestCase {
                bytes: "+200.00".as_bytes(),
                value: 0.0,
                expect_error: true,
            },
        ];
        for test in tests.iter() {
            let mut field = FIXFloat::default();
            let err = field.read(test.bytes);
            assert_eq!(test.expect_error, err.is_err());
            assert_eq!(test.value, field.float64());
        }
    }
}
