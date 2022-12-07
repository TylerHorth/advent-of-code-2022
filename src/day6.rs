use aoc_runner_derive::aoc;

fn bit(char: u8) -> u32 {
    match char {
        b'a'..=b'z' => 1 << (char - b'a'),
        _ => panic!("outside range: {}", char)
    }
}

fn message_offset<const N: usize>(data: &[u8]) -> usize {
    let mut bitset = 0u32;

    for i in 0..N {
        bitset ^= bit(data[i]);
    }

    for i in N..data.len() {
        if bitset.count_ones() == N as u32 {
            return i;
        }

        bitset ^= bit(data[i]);
        bitset ^= bit(data[i - N]);
    }

    panic!("no solution");
}

#[aoc(day6, part1)]
fn part1(data: &[u8]) -> usize {
    message_offset::<4>(data)
}

#[aoc(day6, part2)]
fn part2(data: &[u8]) -> usize {
    message_offset::<14>(data)
}