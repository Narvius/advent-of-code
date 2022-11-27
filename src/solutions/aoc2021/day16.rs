/// Parse the packet from the BITS string, and find it's version number sum.
pub fn one(input: &str) -> crate::Result<u32> {
    Ok(parse_bits_string(to_bits_string(input).as_str())?.version_sum())
}

/// Parse the packet from the BITS string, and evaluate it.
pub fn two(input: &str) -> crate::Result<u128> {
    Ok(parse_bits_string(to_bits_string(input).as_str())?.evaluate())
}

/// The contents of a packet.
enum PacketValue {
    /// A literal value. Only valid for packets with a `type_id` of `4`.
    Literal(u128),
    /// A list of packets contained within this one.
    Subpackets(Vec<Packet>),
}

/// A packet as described by the puzzle.
struct Packet {
    version: u32,
    type_id: u32,
    value: PacketValue,
}

impl Packet {
    /// Finds the version sum for this packet; that is, the version number of it and all sub-packets
    /// summed up.
    fn version_sum(&self) -> u32 {
        self.version
            + match &self.value {
                PacketValue::Literal(_) => 0,
                PacketValue::Subpackets(ps) => ps.iter().map(|p| p.version_sum()).sum(),
            }
    }

    /// Evaluates this packet. Each packet describes either an operation or a literal; evaluating
    /// it reduces that packet down to a final result number.
    fn evaluate(&self) -> u128 {
        fn greater_than(a: u128, b: u128) -> u128 {
            u128::from(a > b)
        }
        fn less_than(a: u128, b: u128) -> u128 {
            u128::from(a < b)
        }
        fn equal_to(a: u128, b: u128) -> u128 {
            u128::from(a == b)
        }

        match &self.value {
            PacketValue::Literal(n) => *n,
            PacketValue::Subpackets(ps) => {
                let operation = match self.type_id {
                    0 => <u128 as std::ops::Add>::add,
                    1 => <u128 as std::ops::Mul>::mul,
                    2 => <u128 as std::cmp::Ord>::min,
                    3 => <u128 as std::cmp::Ord>::max,
                    5 => greater_than,
                    6 => less_than,
                    7 => equal_to,
                    _ => unreachable!(),
                };
                ps.iter().map(|p| p.evaluate()).reduce(operation).unwrap()
            }
        }
    }
}

/// Parses a BITS string into one packet.
fn parse_bits_string(s: &str) -> crate::Result<Packet> {
    // Parses a list of up to `max_packets` packets from `s`, starting at `i`. `i` will be mutated
    // and contain the first unread bit position after the call is done.
    fn recursive_parse(s: &str, i: &mut usize, max_packets: Option<usize>) -> Option<Vec<Packet>> {
        let mut packets = vec![];

        // Parses the next `bits` bits of `s` as a number, then advances `i` by that much.
        fn parse_next(s: &str, i: &mut usize, bits: usize) -> Option<u32> {
            let result = u32::from_str_radix(&s[*i..*i + bits], 2).ok()?;
            *i += bits;
            Some(result)
        }

        while *i < s.len() {
            if max_packets.map(|max| max <= packets.len()).unwrap_or(false) {
                return Some(packets);
            }

            // Version number. We're summing this for the result.
            let version = parse_next(s, i, 3)?;

            // Packet type ID. Decides further processing.
            let type_id = parse_next(s, i, 3)?;

            let value = if type_id == 4 {
                // Keep consuming 5-bit chunks until the leading bit is zero. The remaining four bits
                // from each chunk are concatenated and treated as one value.
                let mut again = true;
                let mut literal = 0;
                while again {
                    again = parse_next(s, i, 1)? > 0;
                    literal *= 16;
                    literal += parse_next(s, i, 4)? as u128;
                }
                PacketValue::Literal(literal)
            } else if parse_next(s, i, 1)? == 0 {
                // Length type 0: Next 15 bits are the length in bits of children. So we call
                // recursively with JUST that slice.
                let bit_length = parse_next(s, i, 15)? as usize;
                let mut sub_i = 0;
                let val = PacketValue::Subpackets(recursive_parse(
                    &s[*i..*i + bit_length],
                    &mut sub_i,
                    None,
                )?);
                *i += sub_i;
                val
            } else {
                // Length type 1: Next 11 bits are the number of immediate children.
                let child_count = parse_next(s, i, 11)? as usize;
                PacketValue::Subpackets(recursive_parse(s, i, Some(child_count))?)
            };

            packets.push(Packet {
                version,
                type_id,
                value,
            })
        }

        Some(packets)
    }

    let mut i = 0;
    Ok(recursive_parse(s, &mut i, Some(1))
        .ok_or("parse failed")?
        .into_iter()
        .next()
        .ok_or("parse failed")?)
}

/// Expands the hexadecimal string into a BITS string (containing only 0s and 1s).
fn to_bits_string(s: &str) -> String {
    let mut bits = String::with_capacity(s.len() * 4);
    for &c in s.as_bytes() {
        bits.push_str(&match c {
            c if (b'0'..=b'9').contains(&c) => format!("{:04b}", c - b'0'),
            c if (b'A'..=b'F').contains(&c) => format!("{:04b}", 10 + c - b'A'),
            _ => String::new(),
        })
    }
    bits
}
