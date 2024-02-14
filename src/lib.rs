mod domain;
mod fuzzer;
mod resolver;
mod scan;

pub use domain::{errors::DomainParseError, models::Domain};
pub use fuzzer::{
    enums::FuzzerStrategy,
    errors::DomainFuzzerError,
    strategies::{
        addition::AdditionFuzzerStrategy, bitsquatting::BitsquattingFuzzerStrategy,
        dot_typo::DotTypoFuzzerStrategy, hyphen_typo::HyphenTypoFuzzerStrategy,
        omission::OmissionFuzzerStrategy, plural::PluralFuzzerStrategy,
        repetition::RepetitionFuzzerStrategy,
    },
};
pub use resolver::DomainResolver;
pub use scan::{scan, GriffinResult};

pub mod prelude {
    use super::*;

    pub use futures::StreamExt;
    pub use fuzzer::traits::*;
}
