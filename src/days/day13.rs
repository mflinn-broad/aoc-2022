use crate::util;
use std::cmp::Ordering;
use std::error::Error;
use std::iter::zip;

pub fn run() -> Result<(), Box<dyn Error>> {
    let raw_input = util::read_input("inputs/day13.txt")?;
    let pairs = parse(raw_input.clone())?;
    println!("part 1: {}", part_1(&pairs));
    let packets = parse_p2(raw_input.clone())?;
    println!("part 2: {}", part_2(packets));
    Ok(())
}

fn parse(input: String) -> Result<Vec<Pair>, ParsePacketError> {
    Ok(input
        .trim()
        .split("\n\n")
        .map(|pair_str| {
            let (p1, p2) = pair_str.split_once('\n').unwrap();
            (p1.try_into().unwrap(), p2.try_into().unwrap())
        })
        .collect())
}

fn parse_p2(input: String) -> Result<Vec<Packet>, ParsePacketError> {
    Ok(input.lines()
        .filter(|line| !line.is_empty())
        .map(|packet_str| packet_str.try_into().unwrap())
        .collect()
    )
}

fn part_1(pairs: &[Pair]) -> usize {
    let mut i_sum = 0;
    for (i, (a, b)) in pairs.iter().enumerate() {
        if let Some(true) = a.in_order(b) {
            i_sum += i + 1;
        }
    }
    i_sum
}

fn part_2(packets: Vec<Packet>) -> usize {
    let mut packets = packets;
    let dividers: [Packet; 2] = [Packet::List(vec![Packet::Num(2)]), Packet::List(vec![Packet::Num(6)])];
    packets.push(dividers[0].clone());
    packets.push(dividers[1].clone());
    packets.sort_by(|a, b| a.partial_cmp(b).unwrap());

    packets.iter().enumerate()
        .filter(|(_, packet)| **packet == dividers[0] || **packet == dividers[1])
        .map(|(i, _)| i + 1)
        .product()
}

type Pair = (Packet, Packet);

#[derive(Debug, Clone)]
enum Packet {
    Num(u16),
    List(Vec<Packet>),
}

impl TryFrom<&str> for Packet {
    type Error = ParsePacketError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Packet::parse(value)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.in_order(other) {
            Some(val) => {
                if val {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }
            None => {
                Some(Ordering::Equal)
            }
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.in_order(other).is_none()
    }
}

impl Packet {
    fn parse(s: &str) -> Result<Self, ParsePacketError> {
        let packet_chars: Vec<_> = s.chars().collect();

        let (packet, remaining) = Self::parse_list(&packet_chars);
        if !remaining.is_empty() {
            let remaining: String = remaining.iter().collect();
            return Err(ParsePacketError::new(&remaining));
        }
        Ok(packet)
    }

    fn parse_list(chars: &[char]) -> (Packet, &[char]) {
        let mut list_contents = &chars[1..];
        let mut packets = Vec::new();

        loop {
            match list_contents[0] {
                ']' => break, // reached the end of the list
                ',' => list_contents = &list_contents[1..],
                '[' => {
                    let (packet, remaining) = Packet::parse_list(list_contents);
                    packets.push(packet);
                    list_contents = remaining;
                }
                _ => {
                    let (num, remaining) = Packet::parse_num(list_contents);
                    packets.push(num);
                    list_contents = remaining;
                }
            }
        }

        (Packet::List(packets), &list_contents[1..])
    }

    fn parse_num(chars: &[char]) -> (Packet, &[char]) {
        let mut i = 0;
        while i < chars.len() && chars[i].is_ascii_digit() {
            i += 1;
        }
        let num = chars[0..i].iter().collect::<String>().parse().unwrap();
        (Packet::Num(num), &chars[i..])
    }

    fn in_order(&self, other: &Packet) -> Option<bool> {
        match (self, other) {
            (Packet::Num(x), Packet::Num(y)) => {
                if x < y {
                    return Some(true);
                } else if x > y {
                    return Some(false);
                } else {
                    return None;
                }
            }
            (Packet::List(l1), Packet::List(l2)) => {
                for (a, b) in zip(l1, l2) {
                    if a.in_order(b).is_some() {
                        return a.in_order(b);
                    }
                }
                if l1.len() < l2.len() {
                    return Some(true);
                } else if l1.len() > l2.len() {
                    return Some(false);
                } else {
                    return None;
                }
            }
            (Packet::Num(val), x) => {
                let new_packet = vec![Packet::Num(*val)];
                let new_list_packet = Packet::List(new_packet);
                return new_list_packet.in_order(x);
            }
            (x, Packet::Num(val)) => {
                let new_packet = vec![Packet::Num(*val)];
                let new_list_packet = Packet::List(new_packet);
                return x.in_order(&new_list_packet);
            }
        }
    }
}

#[derive(Debug, Clone)]
struct ParsePacketError {
    data: String,
}

impl std::error::Error for ParsePacketError {}

impl std::fmt::Display for ParsePacketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unable to parse data as packet: {}", self.data)
    }
}

impl ParsePacketError {
    fn new(packet_str: &str) -> Self {
        Self {
            data: packet_str.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = util::read_input("inputs/day13.txt").unwrap();
        b.iter(|| {
            let pairs = parse(input.clone()).unwrap();
            part_1(&pairs);
        })
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = util::read_input("inputs/day13.txt").unwrap();
        b.iter(|| {
            let packets = parse_p2(input.clone()).unwrap();
            part_2(packets);
        })
    }
}
