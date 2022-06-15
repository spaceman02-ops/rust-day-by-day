fn main() {
    sort_by_bit(&[] as &[u8; 0]);
    //destroy test
    let mut input_set: Vec<HashSet<char>> = Vec::new();
    input_set.push(['A', 'b'].iter().cloned().collect());
    input_set.push(['C', 'd'].iter().cloned().collect());
    destroy(input_set);
}
//https://www.codewars.com/kata/52f5424d0531259cfc000d04
fn sort_by_bit(list: &[u8]) -> u32 {
    if list.len() < 1 {
        return 0;
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

#[cfg(test)]
mod bittests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            sort_by_bit(&[] as &[u8; 0]),
            0b0000,
            "should return 0 if empty array is provided",
        );

        assert_eq!(
            sort_by_bit(&[0]),
            0b0001,
            "should return 1 if array with zero is provided",
        );

        assert_eq!(
            sort_by_bit(&[0, 1]),
            0b0011,
            "should return 3 if array with zero and 1 is provided",
        );

        assert_eq!(
            sort_by_bit(&[1, 0]),
            0b0011,
            "should return 3 if array with zero and 1 is provided, order shouldn't matter",
        );

        assert_eq!(
            sort_by_bit(&[30, 0]),
            0b0100_0000_0000_0000_0000_0000_0000_0001,
            "should return 1073741825 if array with 30 and 0 provided"
        );
    }
}
//https://www.codewars.com/kata/5872637c2eefcb1216000081/train/rust
use std::collections::HashSet;

fn destroy(input_sets: Vec<HashSet<char>>) -> String {
    let mut string = String::new();
    let alphabet = String::from("a b c d e f g h i j k l m n o p q r s t u v w x y z");
    for c in alphabet.chars() {
        if c == ' ' {
            string.push(' ');
            continue;
        }
        let mut destroychar = false;
        for hs in input_sets.iter() {
            if hs.contains(&c) {
                destroychar = true;
            }
        }
        if destroychar {
            string.push('_');
        } else {
            string.push(c)
        }
    }
    return string;
}
//#crushing it!

#[cfg(test)]

mod destroytests {
    use super::destroy;
    use std::collections::HashSet;

    #[test]
    fn basic_test1() {
        let mut input_set: Vec<HashSet<char>> = Vec::new();
        input_set.push(['A', 'b'].iter().cloned().collect());
        input_set.push(['C', 'd'].iter().cloned().collect());
        assert_eq!(
            destroy(input_set),
            "a _ c _ e f g h i j k l m n o p q r s t u v w x y z"
        );
    }

    #[test]
    fn basic_test2() {
        let mut input_set: Vec<HashSet<char>> = Vec::new();
        input_set.push(['B', 'b'].iter().cloned().collect());
        input_set.push(['C', 'm', 'f'].iter().cloned().collect());
        assert_eq!(
            destroy(input_set),
            "a _ c d e _ g h i j k l _ n o p q r s t u v w x y z"
        );
    }
}
