fn main() {
    let data = include_str!("../../inputs/day04.txt");
    let part1 = part1(data);
    println!("result(part1) = {part1}");

    let part2 = part2(data);
    println!("result(part2) = {part2}");
}

fn part1(data: &str) -> usize {
    data.lines()
        .filter(|line| {
            let numbers: Vec<usize> = line
                .splitn(4, |c| c == ',' || c == '-')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            let [min1, max1, min2, max2] = <[usize; 4]>::try_from(numbers).ok().unwrap();
            (min1 <= min2 && max2 <= max1) || (min2 <= min1 && max1 <= max2)
        })
        .count()
}

fn part2(data: &str) -> usize {
    data.lines()
        .filter(|line| {
            let numbers: Vec<usize> = line
                .splitn(4, |c| c == ',' || c == '-')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            let [min1, max1, min2, max2] = <[usize; 4]>::try_from(numbers).ok().unwrap();
            (min1 <= min2 && min2 <= max1) || (min2 <= min1 && min1 <= max2)
        })
        .count()
}
