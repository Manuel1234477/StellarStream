pub fn calculate_unlocked(total_amount: i128, start: u64, cliff: u64, end: u64, now: u64) -> i128 {
    if now < cliff {
        return 0;
    }
    if now >= end {
        return total_amount;
    }

    let elapsed = (now - start) as i128;
    let total_duration = (end - start) as i128;

    (total_amount * elapsed) / total_duration
}

#[cfg(test)]
mod test {
    use super::*;

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
}
