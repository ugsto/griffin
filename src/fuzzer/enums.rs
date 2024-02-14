use super::{
    errors::DomainFuzzerError,
    prelude::DomainFuzzer,
    strategies::{
        addition::AdditionFuzzerStrategy, bitsquatting::BitsquattingFuzzerStrategy,
        dot_typo::DotTypoFuzzerStrategy, hyphen_typo::HyphenTypoFuzzerStrategy,
        omission::OmissionFuzzerStrategy, plural::PluralFuzzerStrategy,
        repetition::RepetitionFuzzerStrategy,
    },
};
use crate::Domain;

#[derive(Debug)]
pub enum FuzzerStrategy {
    // AzertyReplacement(AzertyReplacementFuzzerStrategy), TODO
    // AzertyTypo(AzertyTypoFuzzerStrategy), TODO
    // Cyrillic(CyrillicFuzzerStrategy), TODO
    // Dictionary(DictionaryFuzzerStrategy), TODO
    // Homoglyph(HomoglyphFuzzerStrategy), TODO
    // QwertyReplacement(QwertyReplacementFuzzerStrategy), TODO
    // QwertyTypo(QwertyTypoFuzzerStrategy), TODO
    // QwertzReplacement(QwertzReplacementFuzzerStrategy), TODO
    // QwertzTypo(QwertzTypoFuzzerStrategy), TODO
    // Tld(TldFuzzerStrategy), TODO
    // VowelSwap(VowelSwapFuzzerStrategy), TODO
    Addition(AdditionFuzzerStrategy),
    Bitsquatting(BitsquattingFuzzerStrategy),
    DotTypo(DotTypoFuzzerStrategy),
    HyphenTypo(HyphenTypoFuzzerStrategy),
    Omission(OmissionFuzzerStrategy),
    Plural(PluralFuzzerStrategy),
    Repetition(RepetitionFuzzerStrategy),
}

impl FuzzerStrategy {
    pub fn new_addition() -> Self {
        Self::Addition(AdditionFuzzerStrategy::default())
    }
    pub fn new_bitsquatting() -> Self {
        Self::Bitsquatting(BitsquattingFuzzerStrategy::default())
    }
    pub fn new_dot_typo() -> Self {
        Self::DotTypo(DotTypoFuzzerStrategy::default())
    }
    pub fn new_hyphen_typo() -> Self {
        Self::HyphenTypo(HyphenTypoFuzzerStrategy::default())
    }
    pub fn new_omission() -> Self {
        Self::Omission(OmissionFuzzerStrategy::default())
    }
    pub fn new_plural() -> Self {
        Self::Plural(PluralFuzzerStrategy::default())
    }
    pub fn new_repetition() -> Self {
        Self::Repetition(RepetitionFuzzerStrategy::default())
    }
}

impl TryFrom<&str> for FuzzerStrategy {
    type Error = DomainFuzzerError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "addition" => Ok(Self::new_addition()),
            "bitsquatting" => Ok(Self::new_bitsquatting()),
            "dot-typo" => Ok(Self::new_dot_typo()),
            "hyphen-typo" => Ok(Self::new_hyphen_typo()),
            "omission" => Ok(Self::new_omission()),
            "plural" => Ok(Self::new_plural()),
            "repetition" => Ok(Self::new_repetition()),
            _ => Err(DomainFuzzerError::DomainFuzzerDoesNotExist(s.to_string())),
        }
    }
}

impl DomainFuzzer for FuzzerStrategy {
    fn fuzz<'a>(&self, domain: &'a Domain) -> Box<dyn Iterator<Item = String> + 'a> {
        match self {
            Self::Addition(strategy) => strategy.fuzz(domain),
            Self::Bitsquatting(strategy) => strategy.fuzz(domain),
            Self::DotTypo(strategy) => strategy.fuzz(domain),
            Self::HyphenTypo(strategy) => strategy.fuzz(domain),
            Self::Omission(strategy) => strategy.fuzz(domain),
            Self::Plural(strategy) => strategy.fuzz(domain),
            Self::Repetition(strategy) => strategy.fuzz(domain),
        }
    }
}
