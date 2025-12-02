const DIAL_SIZE: i32 = 100;

use core::str::Lines;

fn main() {
    let lines = include_str!("../input.txt").lines();
    println!("Part 1: {}", part1(lines.clone()));
    println!("Part 2: {}", part2(lines));
}

fn part1(lines: Lines) -> u32 {
    let mut arrow: u32 = 50;
    let mut password: u32 = 0;

    for line in lines {
        if line.len() == 0 { continue; }
        let mut line_iter = line.chars();
        let is_subtract = line_iter.next().unwrap() == 'L';
        let number_str: String = line_iter.collect();
        let number: u32 = (number_str.parse::<u32>().unwrap() % 100) as u32;

        if is_subtract {
            arrow = (arrow as i32 - number as i32).rem_euclid(DIAL_SIZE as i32) as u32;
        } else {
            arrow = (arrow as i32 + number as i32).rem_euclid(DIAL_SIZE as i32) as u32;
        }

        if arrow == 0 {
            password += 1;
        }
    };

    return password;
}

fn part2(lines: Lines) -> i32 {
    let mut arrow: i32 = 50;
    let mut password: i32 = 0;

    for line in lines {
        if line.len() == 0 { continue; }

        let original_arrow = arrow;

        let mut line_iter = line.chars();
        let is_subtract = line_iter.next().unwrap() == 'L';
        let number_str: String = line_iter.take_while(|c| c.is_ascii_digit()).collect();
        let original_number: i32 = number_str.parse::<i32>().unwrap();

        let mut num_ticks = original_number / DIAL_SIZE;
        let number = original_number - (num_ticks * DIAL_SIZE);

        if is_subtract {
            arrow -= number;
        } else {
            arrow += number;
        }

        if arrow < 0 {
            arrow += 100;
            if original_arrow != 0 {
                num_ticks += 1;
            }
        } else if arrow >= 100 {
            arrow -= 100;
            num_ticks += 1;
        } else if arrow == 0 && original_arrow != 0 {
            num_ticks += 1;
        }

        password += num_ticks;
    };

    return password;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_l50() {
        let lines = "L50";
        let password = part2(lines.lines());
        assert_eq!(password, 1);
    }

    #[test]
    fn test_r50() {
        let lines = "R50";
        let password = part2(lines.lines());
        assert_eq!(password, 1);
    }
    #[test]
    fn test_l150() {
        let lines = "L150";
        let password = part2(lines.lines());
        assert_eq!(password, 2);
    }

    #[test]
    fn test_r150() {
        let lines = "R150";
        let password = part2(lines.lines());
        assert_eq!(password, 2);
    }

    #[test]
    fn test_l100() {
        let lines = "L100";
        let password = part2(lines.lines());
        assert_eq!(password, 1);
    }

    #[test]
    fn test_r100() {
        let lines = "R100";
        let password = part2(lines.lines());
        assert_eq!(password, 1);
    }

    #[test]
    fn test_l99() {
        let lines = "L100";
        let password = part2(lines.lines());
        assert_eq!(password, 1);
    }

    #[test]
    fn test_r99() {
        let lines = "R100";
        let password = part2(lines.lines());
        assert_eq!(password, 1);
    }

    #[test]
    fn test_l1() {
        let lines = "L1";
        let password = part2(lines.lines());
        assert_eq!(password, 0);
    }

    #[test]
    fn test_r1() {
        let lines = "R1";
        let password = part2(lines.lines());
        assert_eq!(password, 0);
    }

    #[test]
    fn test_0_to_0_l() {
        let lines = "L50\nL100";
        let password = part2(lines.lines());
        assert_eq!(password, 2);
    }

    #[test]
    fn test_0_to_0_r() {
        let lines = "R50\nR100";
        let password = part2(lines.lines());
        assert_eq!(password, 2);
    }

    #[test]
    fn test_0_l() {
        let lines = "L50\nL1";
        let password = part2(lines.lines());
        assert_eq!(password, 1);
    }

    #[test]
    fn test_0_r() {
        let lines = "R50\nR1";
        let password = part2(lines.lines());
        assert_eq!(password, 1);
    }
}
