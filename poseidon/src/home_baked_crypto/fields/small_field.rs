use super::*;

#[derive(PrimeField)]
#[PrimeFieldModulus = "131071"]
#[PrimeFieldGenerator = "3"]
#[PrimeFieldReprEndianness = "little"]
pub struct SmallField([u64; 1]);

impl FromStr for SmallField {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chunks = str_to_fr::<1, SmallField>(s);
        Ok(SmallField(chunks))
    }
}

impl From<SmallField> for String {
    fn from(element: SmallField) -> Self {
        let mut bytes = vec![];
        for chunk in element.0.iter() {
            bytes.extend_from_slice(&chunk.to_le_bytes());
        }
        BigUint::from_bytes_le(&bytes).to_str_radix(10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Checks that the field is getting read in correctly from strings.
    #[test]
    fn test_ff() {
        let a = SmallField::from_str("2").unwrap();
        assert_eq!("2", &String::from(a));

        let b = SmallField::from_str("131073").unwrap();
        assert_eq!("2", &String::from(b));

        assert_eq!(&a, &b);
    }
}
