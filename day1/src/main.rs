fn main() {
    let lines = include_str!("../input.txt").lines();

    const DIAL_SIZE: u8 = 100;

    let mut arrow: u32 = 50;
    let mut password: u32 = 0;

    lines.for_each(|line| {
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
    });

    println!("{}", password);
}
