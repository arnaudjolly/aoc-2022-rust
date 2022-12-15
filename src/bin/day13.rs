use std::cmp::Ordering;

fn main() {
    let data = include_str!("../../inputs/day13.txt");

    let lines = data
        .lines()
        .filter(|&l| !l.is_empty())
        .collect::<Vec<&str>>();

    let part1 = lines
        .chunks(2)
        .map(|chunk| {
            let [left, right] = <[&str; 2]>::try_from(chunk).ok().unwrap();
            (Packet::parse(left), Packet::parse(right))
        })
        .enumerate()
        .filter_map(|(idx, (left, right))| if left < right { Some(idx + 1) } else { None })
        .sum::<usize>();

    println!("result part 1: {}", part1);

    let mut packets = lines
        .iter()
        .map(|line| Packet::parse(*line))
        .collect::<Vec<Packet>>();

    let delimiter1 = Packet::parse("[[2]]");
    let delimiter2 = Packet::parse("[[6]]");
    packets.push(delimiter1.clone());
    packets.push(delimiter2.clone());
    packets.sort();

    let part2 = packets
        .iter()
        .enumerate()
        .filter_map(|(idx, packet)| {
            if packet == &delimiter1 || packet == &delimiter2 {
                Some(idx + 1)
            } else {
                None
            }
        })
        .product::<usize>();
    println!("result part 2: {}", part2);
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Number(u8),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => a.cmp(b),
            // I am lucky here to see that vec comparison do the expected behaviour :')
            (Self::List(v1), Self::List(v2)) => v1.cmp(v2),
            (Self::Number(a), Self::List(v1)) => vec![Self::Number(*a)].cmp(v1),
            (Self::List(v1), Self::Number(a)) => v1.cmp(&vec![Self::Number(*a)]),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Packet {
    fn parse(s: &str) -> Self {
        let chars = s.chars().collect::<Vec<char>>();
        let (packet, _) = Packet::parse_list(&chars);
        packet
    }

    fn parse_list(chars: &[char]) -> (Packet, &[char]) {
        let mut chars_to_parse = &chars[1..];
        let mut packets = vec![];
        while chars_to_parse.len() > 0 && chars_to_parse[0] != ']' {
            match chars_to_parse[0] {
                ',' => chars_to_parse = &chars_to_parse[1..],
                '[' => {
                    let (inner_packet, remaining) = Packet::parse_list(chars_to_parse);
                    packets.push(inner_packet);
                    chars_to_parse = remaining;
                }
                _ => {
                    // numbers
                    let (packet, remaining) = Packet::parse_num(chars_to_parse);
                    packets.push(packet);
                    chars_to_parse = remaining;
                }
            }
        }
        (Packet::List(packets), &chars_to_parse[1..])
    }

    fn parse_num(chars: &[char]) -> (Packet, &[char]) {
        let mut num: Option<u8> = None;
        let mut chars_to_parse = &chars[..];
        // no need to take care of the length here as there is always other chars behind a digit
        while <char>::is_ascii_digit(&chars_to_parse[0]) {
            let digit = chars_to_parse[0] as u8 - b'0';
            num = match num {
                Some(n) => Some(n * 10 + digit),
                None => Some(digit),
            };
            chars_to_parse = &chars_to_parse[1..];
        }

        if let Some(n) = num {
            return (Packet::Number(n), chars_to_parse);
        }
        panic!("should have found a number here");
    }
}

#[cfg(test)]
mod test {
    use crate::Packet;

    #[test]
    fn parse_should_be_fine() {
        let elt = Packet::parse("[1,[2,[3]],4]");
        assert_eq!(
            elt,
            Packet::List(vec![
                Packet::Number(1),
                Packet::List(vec![
                    Packet::Number(2),
                    Packet::List(vec![Packet::Number(3)])
                ]),
                Packet::Number(4)
            ])
        )
    }

    #[test]
    fn vec_comparison_shorter_but_greater() {
        let v1 = vec![1, 2, 3, 4];
        let v2 = vec![1, 3];
        assert_eq!(v1 < v2, true);
    }

    #[test]
    fn vec_comparison_empty() {
        let v1 = vec![];
        let v2 = vec![1, 2, 3];
        assert_eq!(v1 < v2, true);
    }

    #[test]
    fn vec_comparison_shorter() {
        let v1 = vec![1, 2];
        let v2 = vec![1, 2, 3];
        assert_eq!(v1 < v2, true);
    }

    #[test]
    fn vec_comparison_middle() {
        let v1 = vec![1, 1, 1];
        let v2 = vec![1, 2, 1];
        assert_eq!(v1 < v2, true);
    }

    #[test]
    fn vec_comparison_end() {
        let v1 = vec![1, 1, 1];
        let v2 = vec![1, 1, 2];
        assert_eq!(v1 < v2, true);
    }
}
