use std::collections::HashMap;

fn main() {
    let data = include_str!("../../inputs/day06.txt");
    let lines: Vec<&str> = data.lines().collect();
    let line = lines[0];
    let part1 = part1(line);
    println!("result(part1) = {part1}");

    let part2 = part2(line);
    println!("result(part2) = {part2}");
}

fn find_marker_index(word: &str, marker_size: usize) -> Option<usize> {
    let mut last_seen_chars = HashMap::new();
    let mut last_duplicate_index = None;

    for (i, ch) in word.chars().enumerate() {
        let last_time_we_saw_ch = last_seen_chars.insert(ch, i);
        last_duplicate_index = last_duplicate_index.max(last_time_we_saw_ch);
        if i >= last_duplicate_index.unwrap_or_default() + marker_size {
            return Some(i + 1);
        }
    }
    None
}

fn part1(line: &str) -> usize {
    println!("\nPuzzle (part 1): \n{line}");
    find_marker_index(line, 4).unwrap()
}

fn part2(line: &str) -> usize {
    println!("\nPuzzle (part 2): \n{line}");
    find_marker_index(line, 14).unwrap()
}

#[cfg(test)]
mod test {
    use crate::find_marker_index;

    #[test]
    fn part_1_first_sample() {
        assert_eq!(
            find_marker_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4),
            Some(7)
        );
    }

    #[test]
    fn part_1_second_sample() {
        assert_eq!(
            find_marker_index("bvwbjplbgvbhsrlpgdmjqwftvncz", 4),
            Some(5)
        );
    }

    #[test]
    fn part_1_third_sample() {
        assert_eq!(
            find_marker_index("nppdvjthqldpwncqszvftbrmjlhg", 4),
            Some(6)
        );
    }
    #[test]
    fn part_1_fourth_sample() {
        assert_eq!(
            find_marker_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            Some(10)
        );
    }
    #[test]
    fn part_1_fifth_sample() {
        assert_eq!(
            find_marker_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4),
            Some(11)
        );
    }
}
