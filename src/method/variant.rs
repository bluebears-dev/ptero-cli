use std::{convert::TryFrom, error::Error, fmt};

use super::complex::{eluv::ELUVMethodVariant, extended_line::ExtendedLineMethodVariant};

#[derive(Debug)]
pub struct VariantError(u8);

impl VariantError {
    fn new(variant: u8) -> Self {
        VariantError(variant)
    }
}

#[cfg(not(tarpaulin_include))]
impl fmt::Display for VariantError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid variant equal to {} has been provided", self.0)
    }
}

impl Error for VariantError {}


impl TryFrom<u8> for ELUVMethodVariant {
    type Error = VariantError;

    /// Coverts a value to ELUV method variant enum.
    ///
    /// # Examples
    /// ## Convert number to variant
    /// ```
    /// use ptero::method::complex::eluv::ELUVMethodVariant;
    /// use std::convert::TryFrom;
    ///
    /// assert_eq!(ELUVMethodVariant::try_from(1).unwrap(), ELUVMethodVariant::Variant1);
    /// assert_eq!(ELUVMethodVariant::try_from(2).unwrap(), ELUVMethodVariant::Variant2);
    /// assert_eq!(ELUVMethodVariant::try_from(3).unwrap(), ELUVMethodVariant::Variant3);
    /// ```    
    /// ## Returns error if invalid number
    /// ```should_panic
    /// use ptero::method::complex::eluv::ELUVMethodVariant;
    /// use ptero::method::variant::VariantError;
    /// use std::convert::TryFrom;
    ///
    /// ELUVMethodVariant::try_from(4).unwrap();
    /// ```    
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ELUVMethodVariant::Variant1),
            2 => Ok(ELUVMethodVariant::Variant2),
            3 => Ok(ELUVMethodVariant::Variant3),
            _ => Err(VariantError::new(value))
        }
    }
}

impl TryFrom<u8> for ExtendedLineMethodVariant {
    type Error = VariantError;

    /// Coverts a value to Extended Line method variant enum.
    ///
    /// # Examples
    /// ## Convert number to variant
    /// ```
    /// use ptero::method::complex::extended_line::ExtendedLineMethodVariant;
    /// use std::convert::TryFrom;
    ///
    /// assert_eq!(ExtendedLineMethodVariant::try_from(1).unwrap(), ExtendedLineMethodVariant::Variant1);
    /// assert_eq!(ExtendedLineMethodVariant::try_from(2).unwrap(), ExtendedLineMethodVariant::Variant2);
    /// assert_eq!(ExtendedLineMethodVariant::try_from(3).unwrap(), ExtendedLineMethodVariant::Variant3);
    /// ```    
    /// ## Returns error if invalid number
    /// ```should_panic
    /// use ptero::method::complex::extended_line::ExtendedLineMethodVariant;
    /// use ptero::method::variant::VariantError;
    /// use std::convert::TryFrom;
    ///
    /// ExtendedLineMethodVariant::try_from(4).unwrap();
    /// ```    
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ExtendedLineMethodVariant::Variant1),
            2 => Ok(ExtendedLineMethodVariant::Variant2),
            3 => Ok(ExtendedLineMethodVariant::Variant3),
            _ => Err(VariantError::new(value))
        }
    }
}