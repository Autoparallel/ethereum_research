use super::*;

#[derive(PrimeField)]
#[PrimeFieldModulus = "131071"]
#[PrimeFieldGenerator = "3"]
#[PrimeFieldReprEndianness = "little"]
pub struct SmallField([u64; 1]);

impl FromStr for SmallField {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = str_to_fr::<8, SmallField>(s);
        Ok(SmallField::from_repr(SmallFieldRepr(bytes)).unwrap())
    }
}

impl From<SmallField> for String {
    fn from(element: SmallField) -> Self {
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
        let a = SmallField::from_str("2").unwrap();
        println!("a: {:?}", a);
        assert_eq!("2", &String::from(a));

        let b = SmallField::from_str("131073").unwrap();
        assert_eq!("2", &String::from(b));

        assert_eq!(&a, &b);
    }

    #[test]
    fn ff_add() {
        let two = SmallField::from_str("2").unwrap();
        println!("two: {:?}", two);
        let three = SmallField::from_str("3").unwrap();
        println!("three: {:?}", three);
        let five = SmallField::from_str("5").unwrap();
        println!("five: {:?}", five);
        assert_eq!(&five, &(two + three));

        let one = SmallField::from_str("1").unwrap();
        let q_minus_2 = SmallField::from_str("131069").unwrap();

        assert_eq!(&(q_minus_2 + three), &one);
    }

    #[test]
    fn ff_mul() {
        let two = SmallField::from_str("2").unwrap();
        println!("two: {:?}", two);
        let three = SmallField::from_str("3").unwrap();
        println!("three: {:?}", three);
        let six = SmallField::from_str("6").unwrap();
        println!("six: {:?}", six);
        assert_eq!(&six, &(two * three));
    }
}
