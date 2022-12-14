use crate::util;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let raw_input = util::read_input("inputs/day13.txt")?;
    let pairs = parse(raw_input)?;

    Ok(())
}

fn parse(input: String) -> Result<Vec<Pair>, ParsePacketError> {
    Ok(input
        .split("\n\n")
        .map(|pair_str| {
            let (p1, p2) = pair_str.split_once('\n').unwrap();
            (p1.try_into().unwrap(), p2.try_into().unwrap())
        })
        .collect())
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

impl Packet {
    fn parse(s: &str) -> Result<Self, ParsePacketError> {
        let packet_chars: Vec<_> = s.chars().collect();

        let (packet, remaining) = Self::parse_list(&packet_chars);
        println!("{remaining:?}");
        if !remaining.is_empty() {
            return Err(ParsePacketError::new(s));
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
