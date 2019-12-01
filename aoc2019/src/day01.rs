pub fn part1() -> u32 {
    numbers(&my_input()).map(fuel_simple).sum()
}

pub fn part2() -> u32 {
    numbers(&my_input()).map(fuel_self_lifting).sum()
}

fn numbers<'a>(text: &'a str) -> impl Iterator<Item = u32> + 'a {
    text.lines().filter_map(|line| line.parse::<u32>().ok())
}

fn fuel_simple(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

fn fuel_self_lifting(mass: u32) -> u32 {
    match fuel_simple(mass) {
        0 => 0,
        fuel => fuel + fuel_self_lifting(fuel),
    }
}

fn my_input() -> String {
    std::fs::read_to_string("./res/day01.txt").expect("Cannot read my input")
}
