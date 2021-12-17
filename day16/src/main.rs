use std::i64;
use std::io;

#[derive(Debug)]

enum Packet {
    Literal(u8, u64),
    Operator(u8, u8, Vec<Packet>),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Packet::Literal(v1, x) => match other {
                Packet::Literal(v2, y) => v1 == v2 && x == y,
                _ => false,
            },
            Packet::Operator(v1, t1, ops) => match other {
                Packet::Operator(v2, t2, ops2) => {
                    if v1 != v2 || t1 != t2 || ops.len() != ops2.len() {
                        false
                    } else {
                        ops.iter().zip(ops2.iter()).all(|x| x.0 == x.1)
                    }
                }
                _ => false,
            },
        }
    }
}

impl Packet {
    fn version_sum(&self) -> u64 {
        match self {
            Packet::Literal(v, _) => (*v).into(),
            Packet::Operator(v, _, subp) => {
                let v2: u64 = (*v).into();
                v2 + subp.iter().map(|p| p.version_sum()).sum::<u64>()
            }
        }
    }

    fn value(&self) -> u64 {
        match self {
            Packet::Literal(_, literal) => *literal,
            Packet::Operator(_, opcode, subp) => match opcode {
                0 => subp.iter().map(|p| p.value()).sum::<u64>(),
                1 => subp.iter().map(|p| p.value()).product::<u64>(),
                2 => subp.iter().map(|p| p.value()).min().unwrap(),
                3 => subp.iter().map(|p| p.value()).max().unwrap(),
                5 => {
                    if subp[0].value() > subp[1].value() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if subp[0].value() < subp[1].value() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if subp[0].value() == subp[1].value() {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("Invalid opcode {}", opcode),
            },
        }
    }
}

fn to_binary(value: &str) -> String {
    value
        .chars()
        .map(|x| {
            let parsed: u8 = i64::from_str_radix(&x.to_string()[..], 16)
                .expect("Invalid hex")
                .try_into()
                .unwrap();
            format!(
                "{}{}{}{}",
                ((parsed >> 3) & 1),
                ((parsed >> 2) & 1),
                ((parsed >> 1) & 1),
                ((parsed >> 0) & 1)
            )
        })
        .collect::<Vec<String>>()
        .join("")
}

fn binary_to_int(value: &str) -> u64 {
    let mut v: u64 = 0;
    for c in value.chars() {
        v = (v << 1) | if c == '1' { 1 } else { 0 };
    }
    v
}

fn parse_packet(value: &str) -> (Packet, usize) {
    let bin = to_binary(value);
    parse_packet_bin(&bin[..])
}

fn parse_packet_bin(bin: &str) -> (Packet, usize) {
    let packet_version: u8 = binary_to_int(&bin[0..3]).try_into().unwrap();
    let packet_type: u8 = binary_to_int(&bin[3..6]).try_into().unwrap();
    let mut pos: usize = 6;
    let packet = match packet_type {
        4 => {
            let mut value: u64 = 0;
            loop {
                let continue_bit = bin.chars().nth(pos).unwrap();
                let bits = binary_to_int(&bin[pos + 1..pos + 5]);
                value = (value << 4) | bits;
                pos += 5;
                if continue_bit == '0' {
                    break;
                }
            }
            Packet::Literal(packet_version, value)
        }
        _ => {
            let length_bit = bin.chars().nth(pos).unwrap();
            pos += 1;
            match length_bit {
                '0' => {
                    let length_of_subpackets = binary_to_int(&bin[pos..pos + 15]);
                    pos += 15;
                    let mut length_remaining: usize = length_of_subpackets.try_into().unwrap();
                    let mut subpackets: Vec<Packet> = Vec::new();
                    while length_remaining != 0 {
                        let (subpacket, consumed) = parse_packet_bin(&bin[pos..]);
                        pos += consumed;
                        length_remaining -= consumed;
                        subpackets.push(subpacket);
                    }
                    Packet::Operator(packet_version, packet_type, subpackets)
                }
                '1' => {
                    let num_subpackets = binary_to_int(&bin[pos..pos + 11]);
                    pos += 11;
                    let mut subpackets: Vec<Packet> = Vec::new();
                    for _ in 0..num_subpackets {
                        let (subpacket, consumed) = parse_packet_bin(&bin[pos..]);
                        pos += consumed;
                        subpackets.push(subpacket);
                    }
                    Packet::Operator(packet_version, packet_type, subpackets)
                }
                _ => panic!("Unimplemented: length_bit {}", length_bit),
            }
        }
    };
    (packet, pos)
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer).expect("Unable to read stdin");
    let hex_in = buffer.trim();

    let (packet, _) = parse_packet(hex_in);
    println!("Part 1: {}", packet.version_sum());
    println!("Part 2: {}", packet.value());
}

#[cfg(test)]
mod tests {
    #[test]
    fn literal_packet_parses() {
        assert_eq!(crate::to_binary("D2FE28"), "110100101111111000101000");
        let (packet, pos) = crate::parse_packet("D2FE28");
        assert_eq!(packet, crate::Packet::Literal(6, 2021));
        assert_eq!(pos, 21);
    }

    #[test]
    fn op1_parses() {
        assert_eq!(
            crate::to_binary("38006F45291200"),
            "00111000000000000110111101000101001010010001001000000000"
        );
        let (packet, pos) = crate::parse_packet("38006F45291200");
        assert_eq!(
            packet,
            crate::Packet::Operator(
                1,
                6,
                vec![crate::Packet::Literal(6, 10), crate::Packet::Literal(2, 20)]
            )
        );
        assert_eq!(pos, 49);
    }

    #[test]
    fn op2_parses() {
        assert_eq!(
            crate::to_binary("EE00D40C823060"),
            "11101110000000001101010000001100100000100011000001100000"
        );
        let (packet, pos) = crate::parse_packet("EE00D40C823060");
        assert_eq!(
            packet,
            crate::Packet::Operator(
                7,
                3,
                vec![
                    crate::Packet::Literal(2, 1),
                    crate::Packet::Literal(4, 2),
                    crate::Packet::Literal(1, 3),
                ]
            )
        );
        assert_eq!(pos, 51);
    }

    #[test]
    fn version_sum() {
        assert_eq!(
            crate::parse_packet("8A004A801A8002F478").0.version_sum(),
            16
        );
        assert_eq!(
            crate::parse_packet("620080001611562C8802118E34")
                .0
                .version_sum(),
            12
        );
        assert_eq!(
            crate::parse_packet("C0015000016115A2E0802F182340")
                .0
                .version_sum(),
            23
        );
        assert_eq!(
            crate::parse_packet("A0016C880162017C3686B18A3D4780")
                .0
                .version_sum(),
            31
        );
    }
}
