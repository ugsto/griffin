use std::collections::HashMap;

use crate::fuzzer::traits::Fuzzer;

use super::map::MapFuzzer;

pub struct CyrillicFuzzer {
    mapper_fuzzer: MapFuzzer,
}

impl CyrillicFuzzer {
    pub fn new() -> Self {
        let cyrillic_map = HashMap::from([
            ('a', vec!["а", "ӓ", "ӑ"]),
            ('b', vec!["ь", "ъ"]),
            ('c', vec!["с", "ҫ"]),
            ('d', vec!["ԁ", "д"]),
            ('e', vec!["е", "ё", "э"]),
            ('g', vec!["ԍ", "г"]),
            ('h', vec!["һ", "н"]),
            ('i', vec!["і"]),
            ('j', vec!["ј"]),
            ('k', vec!["кқ"]),
            ('l', vec!["ӏ"]),
            ('m', vec!["м"]),
            ('n', vec!["и"]),
            ('o', vec!["о", "ӧ", "ө"]),
            ('p', vec!["р", "ҏ"]),
            ('q', vec!["ԛ"]),
            ('r', vec!["л"]),
            ('s', vec!["ѕ"]),
            ('t', vec!["т"]),
            ('v', vec!["ѵ"]),
            ('w', vec!["ԝ", "ш"]),
            ('x', vec!["х", "ҳ"]),
            ('y', vec!["у", "ү", "ұ"]),
        ]);
        let mapper_fuzzer = MapFuzzer::new(cyrillic_map);

        Self { mapper_fuzzer }
    }
}

impl Fuzzer for CyrillicFuzzer {
    fn fuzz<'a>(&'a self, domain: &'a str) -> Box<dyn Iterator<Item = String> + 'a> {
        self.mapper_fuzzer.fuzz(domain)
    }
}
