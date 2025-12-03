fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let mut sum: u64 = 0;
    for id_pair in input.split(",") {
        if id_pair.len() == 0 {
            continue;
        }

        println!("--- {} ---", id_pair);
        let mut id_pair_split = id_pair.split("-");
        if id_pair_split.clone().count() != 2 {
            panic!("invalid id pair found!");
        }
        let lower_id = id_pair_split.next().unwrap().trim();
        let upper_id = id_pair_split.next().unwrap().trim();

        let lower_len: u8 = lower_id.len() as u8;
        let upper_len: u8 = upper_id.len() as u8;

        let lower_even = lower_len.is_multiple_of(2);
        let upper_even = upper_len.is_multiple_of(2);

        // invalid IDs are only even amounts of characters
        // still need to process if we go from i.e. length 3 to length 5 though
        if lower_len == upper_len && !lower_len.is_multiple_of(2) {
            continue;
        }

        // given range 10-20, we only need to check 11.
        // since the first half is 1, the second half must be 1 for the id to be invalid
        // given range 10-1000, we need to check all of 1-10 first half
        // (11 first half would put us out of range) - can we check that early?
        // - no but if we go over the top then we're done for this range
        let lower_num = lower_id.trim().parse::<u64>().unwrap();
        let upper_num = upper_id.trim().parse::<u64>().unwrap();

        let range = lower_num..=upper_num;

        println!("({} - {})", lower_id, upper_id);

        // 1-110 we take 1 and go to 10, doubling each (11, 22, 33, 44, ...)
        // 1-90 we go from 1 to 9
        // 9-90 we still need to check 1
        // 998-1020 we take 99, but need to go up to 10. So we have to go through every number?
        //    I don't think so
        // might just need to check for if the range passes a boundary for if the id becomes
        // odd/even in length
        let lower_first_half_count = (lower_len) / 2;
        let upper_first_half_count = (upper_len) / 2;

        println!("({} - {})", lower_id, upper_id);

        let lower;
        if !lower_even {
            lower = u64::pow(10, lower_first_half_count as u32);
        } else {
            let lower_first_half = &lower_id[..lower_first_half_count as usize];
            lower = lower_first_half.parse::<u64>().unwrap();
        }

        let upper;
        if !upper_even {
            upper = u64::pow(10, (upper_first_half_count) as u32) - 1;
        } else {
            let upper_first_half = &upper_id[..upper_first_half_count as usize];
            upper = upper_first_half.parse::<u64>().unwrap();
        }

        for current in lower..=upper {
            let full_str = format!("{current}{current}");
            println!("     {current}{current}");
            let full = full_str.parse::<u64>().unwrap();

            println!("==== {}, ({}, {}) {}", full, range.start(), range.end(), full_str);

            if range.contains(&full) {
                sum += full;
            }

        }
    }

    return sum;
}

fn part2(input: &str) -> u64 {
    _ = input;

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(part1("11-22"), 33); // 11 and 22
        assert_eq!(part1("95-115"), 99);
        assert_eq!(part1("998-1012"), 1010);
        assert_eq!(part1("1188511880-1188511890"), 1188511885);
        assert_eq!(part1("222220-222224"), 222222);
        assert_eq!(part1("1698522-1698528"), 0);
        assert_eq!(part1("446443-446449"), 446446);
        assert_eq!(part1("38593856-38593862"), 38593859);
        assert_eq!(part1("565653-565659"), 0);
        assert_eq!(part1("824824821-824824827"), 0);
        assert_eq!(part1("2121212118-2121212124"), 0);

        assert_eq!(part1("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"), 1227775554);
    }
}
