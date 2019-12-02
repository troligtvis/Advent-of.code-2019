use crate::utils;

pub fn solve() {
    let lines = utils::lines_from_file("input_1.1.txt");

    let mut sum1: i32 = 0;
    let mut sum2: i32 = 0;
    for line in lines {
        let mass: i32 = line.parse().unwrap();
        sum1 += required_fuel(mass);
        sum2 += required_negative_fuel(mass);
    }
    println!("Day1/Part1: {:?}", sum1);
    println!("Day1/Part2: {:?}", sum2);
}

fn required_fuel(mass: i32) -> i32 {
    (mass / 3).abs() - 2
}

fn required_negative_fuel(mass: i32) -> i32 {
    let mut sum = 0;
    let mut fuel = required_fuel(mass);
    while fuel > 0 {
        sum += fuel;
        fuel = required_fuel(fuel);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_requirements() {
        assert_eq!(required_fuel(12), 2);
        assert_eq!(required_fuel(14), 2);
        assert_eq!(required_fuel(1969), 654);
        assert_eq!(required_fuel(100756), 33583);
    }

    #[test]
    fn test_fuel_with_negative() {
        assert_eq!(required_negative_fuel(14), 2);
        assert_eq!(required_negative_fuel(1969), 966);
        assert_eq!(required_negative_fuel(100756), 50346);
    }
}