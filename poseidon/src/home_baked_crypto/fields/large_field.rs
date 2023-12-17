use super::*;

#[derive(PrimeField)]
#[PrimeFieldModulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
#[PrimeFieldGenerator = "7"]
#[PrimeFieldReprEndianness = "little"]
pub struct LargeField([u64; 4]);

impl FromStr for LargeField {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = str_to_fr::<32, LargeField>(s);
        Ok(LargeField::from_repr(LargeFieldRepr(bytes)).unwrap())
    }
}

impl From<LargeField> for String {
    fn from(element: LargeField) -> Self {
        let mut bytes = vec![];
        for byte in element.to_repr().0.iter() {
            bytes.extend_from_slice(&byte.to_le_bytes());
        }
        BigUint::from_bytes_le(&bytes).to_str_radix(10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Checks that the field is getting read in correctly from strings.
    #[test]
    fn ff_mod() {
        let a = LargeField::from_str("2").unwrap();
        assert_eq!("2", &String::from(a));

        let b = LargeField::from_str(
            "21888242871839275222246405745257275088548364400416034343698204186575808495619",
        )
        .unwrap();
        assert_eq!("2", &String::from(b));

        assert_eq!(&a, &b);
    }

    #[test]
    fn ff_add() {
        let two = LargeField::from_str("2").unwrap();
        let three = LargeField::from_str("3").unwrap();
        let five = LargeField::from_str("5").unwrap();
        assert_eq!(&five, &(two + three));

        let one = LargeField::from_str("1").unwrap();
        let q_minus_2 = LargeField::from_str(
            "21888242871839275222246405745257275088548364400416034343698204186575808495615",
        )
        .unwrap();

        assert_eq!(&(q_minus_2 + three), &one);
    }

    #[test]
    fn ff_mul() {
        let two = LargeField::from_str("2").unwrap();
        println!("two: {:?}", two);
        let three = LargeField::from_str("3").unwrap();
        println!("three: {:?}", three);
        let six = LargeField::from_str("6").unwrap();
        println!("six: {:?}", six);
        assert_eq!(&six, &(two * three));
    }
}
