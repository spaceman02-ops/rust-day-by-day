fn main() {}

fn sort_by_bit(list: &[u8]) -> u32 {
    if list.len() < 1 {
        return 1;
    }
    let mut vec = Vec::new();
    for num in 0..32 {
        if list.contains(&num) {
            vec.push(1)
        } else {
            vec.push(0)
        }
    }
    vec.reverse();
    return convert(&vec);
}
fn convert(bits: &[u32]) -> u32 {
    bits.iter().fold(0, |result, &bit| (result << 1) ^ bit)
}
