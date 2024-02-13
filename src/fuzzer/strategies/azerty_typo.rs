use std::collections::HashMap;

use crate::fuzzer::traits::Fuzzer;

use super::insertion::InsertionFuzzer;

pub struct AzertyTypoFuzzer {
    insertion_fuzzer: InsertionFuzzer,
}

impl AzertyTypoFuzzer {
    pub fn new() -> Self {
        let azerty_typo_map = HashMap::from([
            ('1', vec!['2', 'a']),
            ('2', vec!['3', 'z', 'a', '1']),
            ('3', vec!['4', 'e', 'z', '2']),
            ('4', vec!['5', 'r', 'e', '3']),
            ('5', vec!['6', 't', 'r', '4']),
            ('6', vec!['7', 'y', 't', '5']),
            ('7', vec!['8', 'u', 'y', '6']),
            ('8', vec!['9', 'i', 'u', '7']),
            ('9', vec!['0', 'o', 'i', '8']),
            ('0', vec!['p', 'o', '9']),
            ('a', vec!['2', 'z', 'q', '1']),
            ('z', vec!['3', 'e', 's', 'q', 'a', '2']),
            ('e', vec!['4', 'r', 'd', 's', 'z', '3']),
            ('r', vec!['5', 't', 'f', 'd', 'e', '4']),
            ('t', vec!['6', 'y', 'g', 'f', 'r', '5']),
            ('y', vec!['7', 'u', 'h', 'g', 't', '6']),
            ('u', vec!['8', 'i', 'j', 'h', 'y', '7']),
            ('i', vec!['9', 'o', 'k', 'j', 'u', '8']),
            ('o', vec!['0', 'p', 'l', 'k', 'i', '9']),
            ('p', vec!['l', 'o', '0', 'm']),
            ('q', vec!['z', 's', 'w', 'a']),
            ('s', vec!['e', 'd', 'x', 'w', 'q', 'z']),
            ('d', vec!['r', 'f', 'c', 'x', 's', 'e']),
            ('f', vec!['t', 'g', 'v', 'c', 'd', 'r']),
            ('g', vec!['y', 'h', 'b', 'v', 'f', 't']),
            ('h', vec!['u', 'j', 'n', 'b', 'g', 'y']),
            ('j', vec!['i', 'k', 'n', 'h', 'u']),
            ('k', vec!['o', 'l', 'j', 'i']),
            ('l', vec!['k', 'o', 'p', 'm']),
            ('m', vec!['l', 'p']),
            ('w', vec!['s', 'x', 'q']),
            ('x', vec!['w', 's', 'd', 'c']),
            ('c', vec!['x', 'd', 'f', 'v']),
            ('v', vec!['c', 'f', 'g', 'b']),
            ('b', vec!['v', 'g', 'h', 'n']),
            ('n', vec!['b', 'h', 'j']),
        ]);
        let insertion_fuzzer = InsertionFuzzer::new(azerty_typo_map);

        Self { insertion_fuzzer }
    }
}

impl Fuzzer for AzertyTypoFuzzer {
    fn fuzz<'a>(&'a self, domain: &'a str) -> Box<dyn Iterator<Item = String> + 'a> {
        self.insertion_fuzzer.fuzz(domain)
    }
}
