fn main() {
    let data = include_str!("../../inputs/day01.txt");

    let mut elves_calories: Vec<usize> = data
        .lines()
        .collect::<Vec<&str>>()
        .split(|&l| l.is_empty())
        .map(|elf_calories| {
            elf_calories
                .iter()
                .map(|&elf| elf.parse::<usize>().unwrap())
                .sum()
        })
        .collect();

    // reverse order
    elves_calories.sort_by(|a, b| b.cmp(a));

    println!("Part 1: {}", elves_calories[0]);
    println!("Part 2: {}", elves_calories.iter().take(3).sum::<usize>());
}
