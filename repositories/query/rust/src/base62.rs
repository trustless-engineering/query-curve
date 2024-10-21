use num_bigint::BigUint;
use num_traits::Zero;

const CHARSET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const SIXTY_TWO: u8 = 62;

pub fn from_base62(s: &str) -> Result<BigUint, String> {
    let mut num = BigUint::zero();
    let sixty_two = BigUint::from(SIXTY_TWO);

    for c in s.chars() {
        match CHARSET.find(c) {
            Some(index) => {
                num = num * &sixty_two + BigUint::from(index as u8);
            }
            None => return Err(format!("Invalid character '{}' in base62 string", c)),
        }
    }
    Ok(num)
}
