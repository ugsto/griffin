use std::collections::HashMap;

use crate::fuzzer::traits::Fuzzer;

use super::map::MapFuzzer;

pub struct HomoglyphFuzzer {
    mapper_fuzzer: MapFuzzer,
}

impl HomoglyphFuzzer {
    pub fn new() -> Self {
        let cyrillic_map = HashMap::from([
            ('0', vec!["o"]),
            ('1', vec!["l", "i"]),
            ('3', vec!["8"]),
            ('6', vec!["9"]),
            ('8', vec!["3"]),
            ('9', vec!["6"]),
            ('b', vec!["d", "lb"]),
            ('c', vec!["e"]),
            ('d', vec!["b", "cl", "dl"]),
            ('e', vec!["c"]),
            ('g', vec!["q"]),
            ('h', vec!["lh"]),
            ('i', vec!["1", "l"]),
            ('k', vec!["lc"]),
            ('l', vec!["1", "i"]),
            ('m', vec!["n", "nn", "rn", "rr"]),
            ('n', vec!["m", "r"]),
            ('o', vec!["0"]),
            ('q', vec!["g"]),
            ('w', vec!["vv"]),
        ]);
        let mapper_fuzzer = MapFuzzer::new(cyrillic_map);

        Self { mapper_fuzzer }
    }
}

impl Fuzzer for HomoglyphFuzzer {
    fn fuzz<'a>(&'a self, domain: &'a str) -> impl Iterator<Item = String> + 'a {
        self.mapper_fuzzer.fuzz(domain)
    }
}
