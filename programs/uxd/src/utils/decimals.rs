use fixed::types::I80F48;

use crate::error::UxdError;

// E.g: number is 2,000,000,000 (9 decimals), target is 6 decimals, result is 2,000,000
pub fn change_decimals_place(
    number: I80F48,
    actual_decimals: u8,
    target_decimals: u8,
) -> Result<I80F48, UxdError> {
    if target_decimals == actual_decimals {
        return Ok(number);
    }

    // E.g actual_decimals = 9 and target_decimals = 6 equals 3
    let decimals_diff: u32 = actual_decimals.abs_diff(target_decimals).into();

    // E.g decimals_diff = 3 equals 1_000
    // E.g decimals_diff = 6 equals 1_000_000
    let decimals_pow = I80F48::checked_from_num(
        10u64
            .checked_pow(decimals_diff)
            .ok_or(UxdError::MathError)?,
    )
    .ok_or(UxdError::MathError)?;

    if target_decimals < actual_decimals {
        return Ok(number
            .checked_div(decimals_pow)
            .ok_or(UxdError::MathError)?);
    }

    number.checked_mul(decimals_pow).ok_or(UxdError::MathError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_decimals_place_same_decimals() -> Result<(), UxdError> {
        assert_eq!(
            change_decimals_place(I80F48::from_num(1_234), 9, 9)?,
            I80F48::from_num(1_234)
        );

        Ok(())
    }

    #[test]
    fn test_change_decimals_place_expand() -> Result<(), UxdError> {
        assert_eq!(
            change_decimals_place(I80F48::from_num(1), 6, 9)?,
            I80F48::from_num(1_000)
        );

        assert_eq!(
            change_decimals_place(I80F48::from_num(1_000), 6, 9)?,
            I80F48::from_num(1_000_000)
        );

        assert_eq!(
            change_decimals_place(I80F48::from_num(1_000_000), 6, 9)?,
            I80F48::from_num(1_000_000_000)
        );

        Ok(())
    }

    #[test]
    fn test_change_decimals_place_shrink_without_precision_loss() -> Result<(), UxdError> {
        assert_eq!(
            change_decimals_place(I80F48::from_num(1_000_000), 9, 6)?,
            I80F48::from_num(1_000)
        );

        assert_eq!(
            change_decimals_place(I80F48::from_num(1_000), 9, 6)?,
            I80F48::from_num(1)
        );

        Ok(())
    }

    #[test]
    fn test_change_decimals_place_shrink_with_precision_loss() -> Result<(), UxdError> {
        assert_eq!(
            change_decimals_place(I80F48::from_num(1_234_567_890), 9, 6)?.floor(),
            I80F48::from_num(1_234_567)
        );

        assert_eq!(
            change_decimals_place(I80F48::from_num(1_234_567_890), 9, 6)?.ceil(),
            I80F48::from_num(1_234_568)
        );

        Ok(())
    }

    #[test]
    fn test_change_decimals_place_number_too_big() {
        assert!(change_decimals_place(I80F48::MAX, u8::MIN, u8::MAX).is_err());
    }
}
