use std::fs;

// The most efficient storage of a 2-digit hexadecimal number.
type Chunk = u8;

// The number of bits in each chunk.
const CHUNK_SIZE: usize = std::mem::size_of::<Chunk>() * 8;

// The number of bits in each hexadecimal digit.
const HEX_DIGIT_SIZE: usize = CHUNK_SIZE / 2;

// The number of bits in a literal packet value group.
const GROUP_SIZE: usize = HEX_DIGIT_SIZE + 1;

struct BitReader {
    size: usize,
    cursor: usize,
    chunks: Vec<Chunk>,
}

fn get_bits(data: usize, count: usize, offset: usize) -> usize {
    (data & ((1 << count) - 1) << offset) >> offset
}

impl BitReader {
    fn decode_hex(s: &str) -> Self {
        Self {
            cursor: 0,
            size: s.len() * 4,
            chunks: (0..s.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
                .collect(),
        }
    }

    fn read(&mut self, count: usize) -> Option<usize> {
        if self.cursor >= self.size {
            return None;
        }

        let start = self.cursor / CHUNK_SIZE;
        let cursor_end = self.cursor + count - 1;
        let end = cursor_end / CHUNK_SIZE;
        let reach = cursor_end % CHUNK_SIZE + 1;

        let mut data: usize = 0;
        for i in start..=end {
            data <<= CHUNK_SIZE;
            data += self.chunks[i] as usize;
        }

        self.cursor += count;

        let offset = CHUNK_SIZE - reach;
        Some(get_bits(data, count, offset))
    }
}

fn process_literal(bits: &mut BitReader) -> usize {
    let mut value = 0;

    loop {
        let group = bits.read(GROUP_SIZE).unwrap();

        value <<= HEX_DIGIT_SIZE;
        value += get_bits(group, HEX_DIGIT_SIZE, 0);

        if get_bits(group, 1, HEX_DIGIT_SIZE) == 0 {
            break;
        }
    }

    value
}

fn process_n_bits_as_packets(bits: &mut BitReader, n: usize) -> Vec<(usize, usize)> {
    let mut values = Vec::new();
    let mut n = n;

    while n != 0 {
        let cursor_before = bits.cursor;
        values.push(process_packet(bits));
        n -= bits.cursor - cursor_before;
    }

    values
}

fn process_next_n_packets(bits: &mut BitReader, n: usize) -> Vec<(usize, usize)> {
    (0..n).map(|_| process_packet(bits)).collect()
}

fn process_packet(bits: &mut BitReader) -> (usize, usize) {
    let version = bits.read(3).unwrap();
    let type_id = bits.read(3).unwrap();

    if type_id == 4 {
        let value = process_literal(bits);
        return (version, value);
    }

    let length_type_id = bits.read(1).unwrap();

    let version_sums_and_values = if length_type_id == 0 {
        let n = bits.read(15).unwrap();
        process_n_bits_as_packets(bits, n)
    } else {
        let n = bits.read(11).unwrap();
        process_next_n_packets(bits, n)
    };

    let version_sum: usize = version
        + version_sums_and_values
            .iter()
            .map(|(version_sum, _)| *version_sum)
            .sum::<usize>();

    (version_sum, 0)
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let input = input.trim();
    let mut bits = BitReader::decode_hex(input);
    dbg!(process_packet(&mut bits).0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_reader() {
        let mut bits = BitReader::decode_hex("D2FE28");
        assert_eq!(bits.read(3), Some(0b110));
        assert_eq!(bits.read(3), Some(0b100));
        assert_eq!(bits.read(5), Some(0b10111));
        assert_eq!(bits.read(5), Some(0b11110));
        assert_eq!(bits.read(5), Some(0b00101));
        assert_eq!(bits.read(3), Some(0));
        assert_eq!(bits.read(1), None);
    }

    fn assert_packet_version_sum(s: &str, expected_sum: usize) {
        let mut bits = BitReader::decode_hex(s);
        assert_eq!(process_packet(&mut bits).0, expected_sum);
    }

    #[test]
    fn test_examples() {
        assert_packet_version_sum("8A004A801A8002F478", 16);
        assert_packet_version_sum("620080001611562C8802118E34", 12);
        assert_packet_version_sum("C0015000016115A2E0802F182340", 23);
        assert_packet_version_sum("A0016C880162017C3686B18A3D4780", 31);
    }
}
