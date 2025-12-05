use log::info;
use std::str::FromStr;
use thiserror::Error;

use crate::{AllBatteries, BatteryBank};

// Part 2 is almost the exact same as part 1, but the main code has been generalised to work for any length of joltage output.
// Also, I was getting overflows during the number combination process, so I changed from `u32` to `u64` which seems slightly hacky,
// but it works pretty quickly still.

/// The required length of the joltage output in digits.
/// By changing it back to 2, it can also solve part 1!
const LENGTH: usize = 12;

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

fn get_total_output_joltage_of_all_banks(all_battery_banks: &AllBatteries) -> u64 {
    all_battery_banks
        .iter()
        .map(|bank| get_largest_joltage_from_battery_bank(bank))
        .sum()
}

fn get_largest_joltage_from_battery_bank(battery: &BatteryBank) -> u64 {
    let joltages = battery.joltages();
    let mut output = 0;

    let mut current_index = 0;
    for i in 0..LENGTH {
        // We don't need to search the entire vec, we can start at the index of the previous highest value (since it cannot be left of it).
        // And we don't need to go all the way to the end, because the highest value will always be at least LENGTH^10 (assuming no 0 in the input)
        // so the first value will always be at least that far to the left.
        // After that we just "slide" right by 1 every iteration. It seems to work pretty well.
        let slice = &joltages[current_index..(joltages.len() - LENGTH + i + 1)];
        // Get the highest value from the slice and cache the index.
        let index = find_highest_index_from_left(slice);
        let value = slice[index] as u64;
        output = combine_integers(output, value);
        // Logging for some troubleshooting
        info!("slice: {slice:?}, index: {index}, value: {value}, output: {output}");
        // Move the current index forward by the value of the highest index + 1.
        current_index += index + 1;
    }
    output
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
fn combine_integers(a: u64, b: u64) -> u64 {
    a as u64;
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
