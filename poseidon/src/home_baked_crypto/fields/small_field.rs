use super::*;

#[derive(PrimeField)]
#[PrimeFieldModulus = "131071"]
#[PrimeFieldGenerator = "3"]
#[PrimeFieldReprEndianness = "little"]
pub struct SmallField([u64; 1]);

impl FromStr for SmallField {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chunks = str_to_fr::<1>(s);
        Ok(SmallField(chunks))
    }
}
