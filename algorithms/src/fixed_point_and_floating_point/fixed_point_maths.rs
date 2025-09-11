/// As per the smart contract "The price of y per x as a 128.128-binary fixed-point number"
/// 256 bits total (stored as a U256)
/// Top 128 bits = integer part
/// Bottom 128 bits = fractional part
/// Binary point is between the 128th and 129th bit
/// So as an example lets take the number 3.5
/// Integer = 3, fractional = 0.5 
/// However represent as u256 (128:128) this would be represented as in the EVM contract
/// Our max precision for this struct is 2^128 (as we can only store 128 bits of data either side of the fixed point)
/// TODO: By using templating and generics we could create a function that takes any fixed point number with a number of limbs and do this automatically
#[derive(Debug, PartialEq)]
pub struct LFJValue {
    integer_part: u128,
    fractional_part: u128
}

impl LFJValue {
    // This function takes a u256 and separates it into 2 individual components, integer (the first 128 bits) and fractional (the last 128 bits)
    /// Benefits:
    /// 1. Semantically clear and consistent - by doing this, as long as a u256 is passed we can parse any 128 fixed point number
    /// 2. No reliance on anything other than maths, so should work for any input
    /// Disadvantages:
    /// 1. Requires error handling due to parsing
    /// 2. try from can have some run time overhead but can be optimised out
    pub fn from_raw_u256(raw: U256) -> (u128, u128) {
        // Mask for lower 128 bits (fractional part)
        let frac_mask = U256::from(u128::MAX);
        // Extract integer part (upper 128 bits)
        let integer_part = u128::try_from(raw >> 128).expect("Could not cast to u128 (overflow)");
        // Extract fractional part (lower 128 bits)
        let fractional_part = raw & frac_mask;
        let test = u128::try_from(fractional_part).expect("Could not cast to u128 (overflow)");

        (integer_part, fractional_part)
    }

    /// Limbs are used to described a larger number into smaller discrete parts, in our case this is a 256 bit number with 4 limbs as per alloy.
    /// First 2 are fractional parts
    /// Second two are integer parts
    /// [18446744073709551615, 18446744073709551615, 18446744073709551615, 18446744073709551615]
    /// Advantages:
    /// 1. No error handling
    /// 2. Can me more optimal - accessing internal representations of methods of the alloy rather than performing mathematical bit masking and shifting
    /// Disadvantages: 
    /// 1. Less semantically clear (order of MSB 3 -> 2 -> 1 -> 0)
    /// 2. If the number of limbs changes or alloy u256 changes then this function will become deprecated
    pub fn from_u256_limbs(raw: U256) -> Self {
        let limbs = raw.as_limbs();
        // Combine upper 128 bits: 3 is the MSB and therefore needs to be shifted to the left and combined with the lower part of the integer
        // The pipe is a bitwise or
        let integer_part = ((limbs[3] as u128) << 64) | (limbs[2] as u128);

        // Combine lower 128 bits: Same as integer, index 1 is the MSB for the fractional parts and as a result need shifting and combining
        let fractional_part = ((limbs[1] as u128) << 64) | (limbs[0] as u128);

        Self {
            integer_part,
            fractional_part,
        }
    }

    /// Converting self back to fixed point u256 for use within  an evm contract
    pub fn to_fixed_point_u256(&self) -> U256{
        let int_part = U256::from(self.integer_part) << 128;
        let frac_part = U256::from(self.fractional_part);
        int_part + frac_part
    }
}

impl Display for LFJValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.integer_part, self.fractional_part)
    }
}
