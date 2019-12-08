pub const INPUT: &str = include_str!("../res/day01.txt");

pub fn part1(input: &str) -> i32 {
    numbers(input).map(fuel_needed).sum()
}

fn fuel_needed(mass: i32) -> i32 {
    mass / 3 - 2
}

fn numbers<'a>(text: &'a str) -> impl Iterator<Item = i32> + 'a {
    text.lines().filter_map(|line| line.parse().ok())
}

pub fn part2(input: &str) -> i32 {
    numbers(input).map(total_fuel_needed).sum()
}

fn total_fuel_needed(mass: i32) -> i32 {
    match fuel_needed(mass) {
        fuel if fuel <= 0 => 0,
        fuel => fuel + total_fuel_needed(fuel),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_needed_from_example() {
        for &(mass, fuel) in &[(12, 2), (14, 2), (1969, 654), (100756, 33583)] {
            assert_eq!(fuel_needed(mass), fuel);
        }
    }

    #[test]
    fn part1_with_given_data() {
        assert_eq!(part1(INPUT), 3282386);
    }

    #[test]
    fn total_fuel_needed_example() {
        for &(mass, fuel) in &[(12, 2), (14, 2), (1969, 966), (100756, 50346)] {
            assert_eq!(total_fuel_needed(mass), fuel);
        }
    }

    #[test]
    fn part2_with_given_data() {
        assert_eq!(part2(INPUT), 4920708);
    }
}
