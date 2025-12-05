use std::str::FromStr;

use thiserror::Error;

use crate::{AllBatteries, BatteryBank};

#[derive(Debug, Error)]
pub enum PartError {
    #[error(transparent)]
    ParsingError(#[from] crate::ParseError),
}

pub fn run(input: &str) -> Result<u64, PartError> {
    let all_battery_banks = AllBatteries::from_str(input)?;
    let total_output_joltage = get_total_output_joltage_of_all_banks(&all_battery_banks);
    Ok(total_output_joltage as u64)
}

fn get_total_output_joltage_of_all_banks(all_battery_banks: &AllBatteries) -> u32 {
    all_battery_banks
        .iter()
        .map(|bank| get_largest_joltage_from_battery_bank(bank))
        .sum()
}

fn get_largest_joltage_from_battery_bank(battery: &BatteryBank) -> u32 {
    let joltages = battery.joltages();
    // We can exclude the final joltage from the slice since it cannot be the first element.
    let first_slice = &joltages[..joltages.len() - 1];
    let first_index = find_highest_index_from_left(first_slice);
    // Then we get a slice from the first index (but not including it) to the end of the slice.
    let second_slice = &joltages[first_index + 1..];
    let second_index = find_highest_index_from_left(second_slice);

    let a = first_slice[first_index];
    let b = second_slice[second_index];

    combine_integers(a, b)
}

fn find_highest_index_from_left(range: &[u32]) -> usize {
    let mut best_index = 0;
    let mut best_value = 0;

    for (i, value) in range.iter().enumerate() {
        if *value == 9 {
            return i; // Special case since 9 is the highest value.
        }
        if *value > best_value {
            best_index = i;
            best_value = *value;
        }
    }

    best_index
}
/// Combines two integers `a` and `b` into `ab` without using string conversion.
/// works as long as both `a` and `b` are less than 10.
fn combine_integers(a: u32, b: u32) -> u32 {
    a * 10 + b
}

#[cfg(test)]
mod tests {
    use crate::AllBatteries;

    use super::*;

    fn setup() -> AllBatteries {
        let batteries = vec![
            BatteryBank(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]),
            BatteryBank(vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]),
            BatteryBank(vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),
            BatteryBank(vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]),
        ];
        AllBatteries(batteries)
    }

    #[test]
    fn test_find_highest_index_from_left() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(find_highest_index_from_left(&input), 8);
    }

    #[test]
    fn test_find_highest_index_from_left_early_exit() {
        let input = vec![1, 2, 3, 9, 5, 6, 7, 8, 9];
        assert_eq!(find_highest_index_from_left(&input), 3);
    }

    #[test]
    fn test_combine_integers() {
        assert_eq!(combine_integers(1, 2), 12);
        assert_eq!(combine_integers(9, 8), 98);
        assert_eq!(combine_integers(5, 5), 55);
    }

    #[test]
    fn test_get_largest_joltage_from_battery_bank() {
        let battery_bank = BatteryBank(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let largest_joltage = get_largest_joltage_from_battery_bank(&battery_bank);
        assert_eq!(largest_joltage, 89);
    }

    #[test]
    fn test_get_total_output_joltage_of_all_banks() {
        let all_battery_banks = setup();
        let total_output_joltage = get_total_output_joltage_of_all_banks(&all_battery_banks);
        assert_eq!(total_output_joltage, 98 + 89 + 78 + 92);
    }
}
