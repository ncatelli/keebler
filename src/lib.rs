use parcel::prelude::v1::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    ThirtyTwo,
    SixtyFour,
}

struct ClassParser;

impl<'a> parcel::Parser<'a, &'a [u8], Class> for ClassParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], Class> {
        parcel::parsers::byte::expect_byte(0x01)
            .map(|_| Class::ThirtyTwo)
            .or(|| parcel::parsers::byte::expect_byte(0x02).map(|_| Class::SixtyFour))
            .parse(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endianness {
    Little,
    Big,
}

struct EndiannessParser;

impl<'a> parcel::Parser<'a, &'a [u8], Endianness> for EndiannessParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], Endianness> {
        parcel::parsers::byte::expect_byte(0x01)
            .map(|_| Endianness::Little)
            .or(|| parcel::parsers::byte::expect_byte(0x02).map(|_| Endianness::Big))
            .parse(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Version {
    One,
}

struct VersionParser;

impl<'a> parcel::Parser<'a, &'a [u8], Version> for VersionParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], Version> {
        parcel::parsers::byte::expect_byte(0x01)
            .map(|_| Version::One)
            .parse(input)
    }
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
pub enum Type {
    None,
    Rel,
    Exec,
    Dyn,
    Core,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Machine {
    X86,
    X86_64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntryPoint(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ELFHeader {
    class: Class,
    endianness: Endianness,
    version: Version,
    abi: ABI,
    r#type: Type,
    machine: Machine,
    vers: Version,
    entry_point: EntryPoint,
}

pub struct ELFParser;

impl<'a> parcel::Parser<'a, &'a [u8], ELFHeader> for ELFParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], ELFHeader> {
        parcel::right(parcel::join(
            expect_bytes(&[0x7f, 0x45, 0x4c, 0x46]),
            parcel::join(
                parcel::join(ClassParser, EndiannessParser),
                parcel::join(VersionParser, ABIParser),
            ),
        ))
        // skip padding
        .and_then(|last| parcel::take_n(parcel::parsers::byte::any_byte(), 8).map(move |_| last))
        .map(|((class, endianness), (version, abi))| ELFHeader {
            class,
            endianness,
            version,
            abi,
            r#type: Type::None,
            machine: Machine::X86,
            vers: version,
            entry_point: EntryPoint(0),
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
    use super::*;
    #[test]
    fn parse_known_good_header() {
        let input = [
            0x7f, 0x45, 0x4c, 0x46, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];

        assert_eq!(
            ELFParser.parse(&input).unwrap().unwrap(),
            ELFHeader {
                class: Class::ThirtyTwo,
                endianness: Endianness::Little,
                version: Version::One,
                abi: ABI::SysV,
                r#type: Type::None,
                machine: Machine::X86_64,
                vers: Version::One,
                entry_point: EntryPoint(0),
            }
        )
    }
}
