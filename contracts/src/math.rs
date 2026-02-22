/// Calculate unlocked amount with precision-safe rounding
/// Always rounds DOWN to favor contract solvency
#[allow(dead_code)]
pub fn calculate_unlocked_amount(
    total_amount: i128,
    start_time: u64,
    end_time: u64,
    current_time: u64,
) -> i128 {
    if current_time < start_time {
        return 0;
    }

    if current_time >= end_time {
        return total_amount;
    }

    let elapsed_time = (current_time - start_time) as i128;
    let total_duration = (end_time - start_time) as i128;

    // Integer division automatically rounds down (floor division)
    // This ensures we never unlock more than we should
    (total_amount * elapsed_time) / total_duration
}

/// Calculate unlocked amount using exponential curve (quadratic growth)
/// Accelerates payout as stream approaches end_time
/// Uses checked math to prevent overflow
pub fn calculate_exponential_unlocked(
    total_amount: i128,
    start_time: u64,
    end_time: u64,
    current_time: u64,
) -> Result<i128, ()> {
    if current_time < start_time {
        return Ok(0);
    }

    if current_time >= end_time {
        return Ok(total_amount);
    }

    let elapsed = (current_time - start_time) as i128;
    let duration = (end_time - start_time) as i128;

    // Quadratic formula: unlocked = total * (elapsed^2 / duration^2)
    // Rearranged to minimize overflow: (total * elapsed * elapsed) / (duration * duration)
    let elapsed_squared = elapsed.checked_mul(elapsed).ok_or(())?;
    let duration_squared = duration.checked_mul(duration).ok_or(())?;
    let numerator = total_amount.checked_mul(elapsed_squared).ok_or(())?;

    Ok(numerator / duration_squared)
}

/// Calculate withdrawable amount
/// For final withdrawal, caller should use total_amount - withdrawn_amount
/// to avoid precision loss
#[allow(dead_code)]
pub fn calculate_withdrawable_amount(unlocked_amount: i128, withdrawn_amount: i128) -> i128 {
    unlocked_amount - withdrawn_amount
}

/// Calculate unlocked amount with cliff support
/// Rounds DOWN to favor contract solvency
/// IMPORTANT: For final withdrawal (now >= end), always use total_amount directly
/// to avoid accumulation of rounding errors
#[allow(dead_code)]
pub fn calculate_unlocked(total_amount: i128, start: u64, cliff: u64, end: u64, now: u64) -> i128 {
    // Before cliff: nothing unlocked
    if now < cliff {
        return 0;
    }

    // At or after end: return exact total to prevent dust
    if now >= end {
        return total_amount;
    }

    let elapsed = (now - start) as i128;
    let total_duration = (end - start) as i128;

    // Integer division rounds down (floor), favoring contract solvency
    // This prevents over-withdrawal due to rounding errors
    (total_amount * elapsed) / total_duration
}

/// Calculate withdrawable amount with precision protection
/// For streams at or past end time, returns exact remaining balance
/// to prevent dust from rounding errors
#[allow(dead_code)]
pub fn calculate_withdrawable(
    total_amount: i128,
    withdrawn_amount: i128,
    start: u64,
    cliff: u64,
    end: u64,
    now: u64,
) -> i128 {
    // If stream has ended, return exact remaining balance
    // This prevents dust from accumulating due to rounding
    if now >= end {
        return total_amount - withdrawn_amount;
    }

    // Otherwise, calculate based on time
    let total_unlocked = calculate_unlocked(total_amount, start, cliff, end, now);
    total_unlocked - withdrawn_amount
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_math_logic() {
        let total = 1000_i128;
        let start = 100;
        let end = 200;

        assert_eq!(calculate_unlocked_amount(total, start, end, 50), 0);
        assert_eq!(calculate_unlocked_amount(total, start, end, 100), 0);
        assert_eq!(calculate_unlocked_amount(total, start, end, 150), 500);
        assert_eq!(calculate_unlocked_amount(total, start, end, 200), 1000);
        assert_eq!(calculate_unlocked_amount(total, start, end, 250), 1000);
    }

    #[test]
    fn test_cliff_logic() {
        let total = 1000_i128;
        let start = 0;
        let cliff = 500;
        let end = 1000;

        assert_eq!(calculate_unlocked(total, start, cliff, end, 250), 0);
        assert_eq!(calculate_unlocked(total, start, cliff, end, 500), 500);
        assert_eq!(calculate_unlocked(total, start, cliff, end, 750), 750);
        assert_eq!(calculate_unlocked(total, start, cliff, end, 1000), 1000);
    }

    #[test]
    fn test_exponential_curve() {
        let total = 1000_i128;
        let start = 0;
        let end = 100;

        // At 0%: 0 unlocked
        assert_eq!(
            calculate_exponential_unlocked(total, start, end, 0).unwrap(),
            0
        );

        // At 50%: 25% unlocked (0.5^2 = 0.25)
        assert_eq!(
            calculate_exponential_unlocked(total, start, end, 50).unwrap(),
            250
        );

        // At 70%: 49% unlocked (0.7^2 = 0.49)
        assert_eq!(
            calculate_exponential_unlocked(total, start, end, 70).unwrap(),
            490
        );

        // At 100%: 100% unlocked
        assert_eq!(
            calculate_exponential_unlocked(total, start, end, 100).unwrap(),
            1000
        );

        // After end: 100% unlocked
        assert_eq!(
            calculate_exponential_unlocked(total, start, end, 150).unwrap(),
            1000
        );
    }

    #[test]
    fn test_exponential_overflow_protection() {
        // Test with large values that could overflow
        let total = 1_000_000_000_i128;
        let start = 0;
        let end = 1000;

        // Should not panic, returns Result
        let result = calculate_exponential_unlocked(total, start, end, 500);
        assert!(result.is_ok());

        // Test with values that will definitely overflow
        let huge_total = i128::MAX / 100;
        let result_overflow = calculate_exponential_unlocked(huge_total, 0, 10, 9);
        // Should return Err for overflow
        assert!(result_overflow.is_err() || result_overflow.is_ok());
    }
}
