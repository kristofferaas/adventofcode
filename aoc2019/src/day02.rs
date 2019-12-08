pub const INPUT: &str = include_str!("../res/day02.txt");

pub fn part1(input: &str) -> Vec<u32> {
    execute(parse(input))
}

pub fn part2(input: &str, target: u32) -> u32 {
    brute_force(parse(input), target)
}

fn execute(mut tape: Vec<u32>) -> Vec<u32> {
    let mut count: usize = 0;
    loop {
        match tape[count] {
            1 => {
                let (v1, v2) = get_values(&tape, count);
                let pointer = tape[count + 3] as usize;
                tape[pointer] = v1 + v2;
            }
            2 => {
                let (v1, v2) = get_values(&tape, count);
                let pointer = tape[count + 3] as usize;
                tape[pointer] = v1 * v2;
            }
            99 => {
                break;
            }
            _ => {}
        }
        count += 4;
    }
    tape
}

fn get_values(tape: &Vec<u32>, count: usize) -> (u32, u32) {
    let (op1, op2) = (tape[count + 1], tape[count + 2]);
    (tape[op1 as usize], tape[op2 as usize])
}

fn parse(input: &str) -> Vec<u32> {
    let opcodes = input
        .trim()
        .split(',')
        .filter_map(|n| n.parse().ok())
        .collect::<Vec<u32>>();
    opcodes
}

fn brute_force(tape: Vec<u32>, target: u32) -> u32 {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut memory = tape.clone();
            memory[1] = noun;
            memory[2] = verb;
            let result = execute(memory)[0];
            if target == result {
                return 100 * noun + verb;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(
            execute(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
        assert_eq!(execute(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
        assert_eq!(execute(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
        assert_eq!(execute(vec![2, 4, 4, 5, 99, 0]), vec![2, 4, 4, 5, 99, 9801]);
        assert_eq!(
            execute(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }

    #[test]
    fn part1_with_given_data() {
        assert_eq!(part1(INPUT)[0], 4945026);
    }

    #[test]
    fn part2_with_given_data() {
        assert_eq!(part2(INPUT, 19690720), 5296);
    }
}
