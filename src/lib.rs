use parcel::prelude::v1::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    ThirtyTwo,
    SixtyFour,
}

struct ClassParser;

impl<'a> parcel::Parser<'a, &'a [u8], Class> for ClassParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], Class> {
        parcel::parsers::byte::expect_byte(0x00)
            .map(|_| Class::ThirtyTwo)
            .or(|| parcel::parsers::byte::expect_byte(0x01).map(|_| Class::SixtyFour))
            .parse(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endianness {
    Little,
    Big,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Version {
    One,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ABI {
    SysV,
}

struct ABIParser;

impl<'a> parcel::Parser<'a, &'a [u8], ABI> for ABIParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], ABI> {
        parcel::parsers::byte::expect_byte(0x00)
            .map(|_| ABI::SysV)
            .parse(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ELFHeader {
    class: Class,
    endianess: Endianness,
    version: Version,
    abi: ABI,
}

impl<'a> parcel::Parser<'a, &'a [u8], ELFHeader> for ELFHeader {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], ELFHeader> {
        parcel::right(parcel::join(
            expect_bytes(&[0x7f, 0x45, 0x4c, 0x46]),
            parcel::join(ClassParser, ABIParser),
        ))
        .map(|(class, abi)| Self {
            class,
            endianess: Endianness::Little,
            version: Version::One,
            abi,
        })
        .parse(input)
    }
}

pub fn expect_bytes<'a>(expected: &'static [u8]) -> impl Parser<'a, &'a [u8], Vec<u8>> {
    move |input: &'a [u8]| {
        let preparse_input = input;
        let expected_len = expected.len();
        let next: Vec<u8> = input.iter().take(expected_len).copied().collect();
        if next == expected {
            Ok(MatchStatus::Match((&input[expected_len..], next)))
        } else {
            Ok(MatchStatus::NoMatch(preparse_input))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
