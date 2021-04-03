use std::convert::TryInto;

use parcel::prelude::v1::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    ThirtyTwo,
    SixtyFour,
}

impl From<Class> for u8 {
    fn from(src: Class) -> Self {
        match src {
            Class::ThirtyTwo => 1,
            Class::SixtyFour => 2,
        }
    }
}

struct ClassParser;

impl<'a> parcel::Parser<'a, &'a [u8], Class> for ClassParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], Class> {
        parcel::one_of(vec![
            parcel::parsers::byte::expect_byte(0x01).map(|_| Class::ThirtyTwo),
            parcel::parsers::byte::expect_byte(0x02).map(|_| Class::SixtyFour),
        ])
        .parse(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Endianness {
    Little = 0x01,
    Big = 0x02,
}

impl From<Endianness> for u8 {
    fn from(src: Endianness) -> Self {
        src as u8
    }
}

struct EndiannessParser;

impl<'a> parcel::Parser<'a, &'a [u8], Endianness> for EndiannessParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], Endianness> {
        parcel::one_of(vec![
            parcel::parsers::byte::expect_byte(Endianness::Little as u8)
                .map(|_| Endianness::Little),
            parcel::parsers::byte::expect_byte(Endianness::Big as u8).map(|_| Endianness::Big),
        ])
        .parse(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Version {
    One = 1,
}

impl From<Version> for u8 {
    fn from(src: Version) -> Self {
        src as u8
    }
}

struct VersionParser;

impl<'a> parcel::Parser<'a, &'a [u8], Version> for VersionParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], Version> {
        parcel::parsers::byte::expect_byte(Version::One as u8)
            .map(|_| Version::One)
            .parse(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ABI {
    SysV = 0x00,
    HPUX = 0x01,
    NetBSD = 0x02,
    Linux = 0x03,
    GNUHurd = 0x04,
    Solaris = 0x06,
    AIX = 0x07,
    IRIX = 0x08,
    FreeBSD = 0x09,
    Tru64 = 0x0A,
    Novell = 0x0B,
    OpenBSD = 0x0C,
    OpenVMS = 0x0D,
    NonStop = 0x0E,
    Aros = 0x0F,
    Fenix = 0x10,
    CloudABI = 0x11,
    OpenVOS = 0x12,
}

impl From<ABI> for u16 {
    fn from(src: ABI) -> Self {
        src as u16
    }
}

struct ABIParser;

impl<'a> parcel::Parser<'a, &'a [u8], ABI> for ABIParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], ABI> {
        parcel::one_of(vec![
            expect_u16(ABI::SysV as u16).map(|_| ABI::SysV),
            expect_u16(ABI::HPUX as u16).map(|_| ABI::HPUX),
            expect_u16(ABI::NetBSD as u16).map(|_| ABI::NetBSD),
            expect_u16(ABI::Linux as u16).map(|_| ABI::Linux),
            expect_u16(ABI::GNUHurd as u16).map(|_| ABI::GNUHurd),
            expect_u16(ABI::Solaris as u16).map(|_| ABI::Solaris),
            expect_u16(ABI::AIX as u16).map(|_| ABI::AIX),
            expect_u16(ABI::IRIX as u16).map(|_| ABI::IRIX),
            expect_u16(ABI::FreeBSD as u16).map(|_| ABI::FreeBSD),
            expect_u16(ABI::Tru64 as u16).map(|_| ABI::Tru64),
            expect_u16(ABI::Novell as u16).map(|_| ABI::Novell),
            expect_u16(ABI::OpenBSD as u16).map(|_| ABI::OpenBSD),
            expect_u16(ABI::OpenVMS as u16).map(|_| ABI::OpenVMS),
            expect_u16(ABI::NonStop as u16).map(|_| ABI::NonStop),
            expect_u16(ABI::Aros as u16).map(|_| ABI::Aros),
            expect_u16(ABI::Fenix as u16).map(|_| ABI::Fenix),
            expect_u16(ABI::CloudABI as u16).map(|_| ABI::CloudABI),
            expect_u16(ABI::OpenVOS as u16).map(|_| ABI::OpenVOS),
        ])
        .parse(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Type {
    None = 0x00,
    Rel = 0x01,
    Exec = 0x02,
    Dyn = 0x03,
    Core = 0x04,
    LOOS = 0xFE00,
    HIOS = 0xFEFF,
    LOPROC = 0xFF00,
    HIPROC = 0xFFFF,
}

impl From<Type> for u16 {
    fn from(src: Type) -> Self {
        src as u16
    }
}

struct TypeParser;

impl<'a> parcel::Parser<'a, &'a [u8], Type> for TypeParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], Type> {
        parcel::one_of(vec![
            expect_u16(Type::None as u16).map(|_| Type::None),
            expect_u16(Type::Rel as u16).map(|_| Type::Rel),
            expect_u16(Type::Exec as u16).map(|_| Type::Exec),
            expect_u16(Type::Dyn as u16).map(|_| Type::Dyn),
            expect_u16(Type::Core as u16).map(|_| Type::Core),
            expect_u16(Type::LOOS as u16).map(|_| Type::LOOS),
            expect_u16(Type::HIOS as u16).map(|_| Type::HIOS),
            expect_u16(Type::LOPROC as u16).map(|_| Type::LOPROC),
            expect_u16(Type::HIPROC as u16).map(|_| Type::HIPROC),
        ])
        .parse(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Machine {
    None = 0x00,
    M32 = 0x01,
    SPARC = 0x02,
    X386 = 0x03,
    M68k = 0x04,
    M88k = 0x05,
    IntelMCU = 0x06,
    Intel80860 = 0x07,
    MIPS = 0x08,
    S370 = 0x09,
    MIPSRS3LE = 0x0A,
    PARISC = 0x0E,
    I960 = 0x13,
    PPC = 0x14,
    PPC64 = 0x15,
    S390 = 0x16,
    V800 = 0x24,
    FR20 = 0x25,
    RH32 = 0x26,
    RCE = 0x27,
    ARM = 0x28,
    Alpha = 0x29,
    SH = 0x2A,
    SPARCV9 = 0x2B,
    Tricore = 0x2C,
    ARC = 0x2D,
    H8300 = 0x2E,
    H8_300H = 0x2F,
    H8s = 0x30,
    H8500 = 0x31,
    IA64 = 0x32,
    MIPSX = 0x33,
    Coldfire = 0x34,
    M68HC12 = 0x35,
    MMA = 0x36,
    PCP = 0x37,
    NCPU = 0x38,
    NDR1 = 0x39,
    Starcore = 0x3A,
    ME16 = 0x3B,
    ST100 = 0x3C,
    TinyJ = 0x3D,
    X86_64 = 0x3E,
    S320C600 = 0x8C,
    AARCH64 = 0xB7,
    RISCV = 0xF3,
    BPF = 0xF7,
    WDC65C817 = 0x101,
}

impl From<Machine> for u16 {
    fn from(src: Machine) -> Self {
        src as u16
    }
}

struct MachineParser;

impl<'a> parcel::Parser<'a, &'a [u8], Machine> for MachineParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], Machine> {
        parcel::one_of(vec![
            expect_u16(Machine::None as u16).map(|_| Machine::None),
            expect_u16(Machine::M32 as u16).map(|_| Machine::M32),
            expect_u16(Machine::SPARC as u16).map(|_| Machine::SPARC),
            expect_u16(Machine::X386 as u16).map(|_| Machine::X386),
            expect_u16(Machine::M68k as u16).map(|_| Machine::M68k),
            expect_u16(Machine::M88k as u16).map(|_| Machine::M88k),
            expect_u16(Machine::IntelMCU as u16).map(|_| Machine::IntelMCU),
            expect_u16(Machine::Intel80860 as u16).map(|_| Machine::Intel80860),
            expect_u16(Machine::MIPS as u16).map(|_| Machine::MIPS),
            expect_u16(Machine::S370 as u16).map(|_| Machine::S370),
            expect_u16(Machine::MIPSRS3LE as u16).map(|_| Machine::MIPSRS3LE),
            expect_u16(Machine::PARISC as u16).map(|_| Machine::PARISC),
            expect_u16(Machine::I960 as u16).map(|_| Machine::I960),
            expect_u16(Machine::PPC as u16).map(|_| Machine::PPC),
            expect_u16(Machine::PPC64 as u16).map(|_| Machine::PPC64),
            expect_u16(Machine::S390 as u16).map(|_| Machine::S390),
            expect_u16(Machine::V800 as u16).map(|_| Machine::V800),
            expect_u16(Machine::FR20 as u16).map(|_| Machine::FR20),
            expect_u16(Machine::RH32 as u16).map(|_| Machine::RH32),
            expect_u16(Machine::RCE as u16).map(|_| Machine::RCE),
            expect_u16(Machine::ARM as u16).map(|_| Machine::ARM),
            expect_u16(Machine::Alpha as u16).map(|_| Machine::Alpha),
            expect_u16(Machine::SH as u16).map(|_| Machine::SH),
            expect_u16(Machine::SPARCV9 as u16).map(|_| Machine::SPARCV9),
            expect_u16(Machine::Tricore as u16).map(|_| Machine::Tricore),
            expect_u16(Machine::ARC as u16).map(|_| Machine::ARC),
            expect_u16(Machine::H8300 as u16).map(|_| Machine::H8300),
            expect_u16(Machine::H8_300H as u16).map(|_| Machine::H8_300H),
            expect_u16(Machine::H8s as u16).map(|_| Machine::H8s),
            expect_u16(Machine::H8500 as u16).map(|_| Machine::H8500),
            expect_u16(Machine::IA64 as u16).map(|_| Machine::IA64),
            expect_u16(Machine::MIPSX as u16).map(|_| Machine::MIPSX),
            expect_u16(Machine::Coldfire as u16).map(|_| Machine::Coldfire),
            expect_u16(Machine::M68HC12 as u16).map(|_| Machine::M68HC12),
            expect_u16(Machine::MMA as u16).map(|_| Machine::MMA),
            expect_u16(Machine::PCP as u16).map(|_| Machine::PCP),
            expect_u16(Machine::NCPU as u16).map(|_| Machine::NCPU),
            expect_u16(Machine::NDR1 as u16).map(|_| Machine::NDR1),
            expect_u16(Machine::Starcore as u16).map(|_| Machine::Starcore),
            expect_u16(Machine::ME16 as u16).map(|_| Machine::ME16),
            expect_u16(Machine::ST100 as u16).map(|_| Machine::ST100),
            expect_u16(Machine::TinyJ as u16).map(|_| Machine::TinyJ),
            expect_u16(Machine::X86_64 as u16).map(|_| Machine::X86_64),
            expect_u16(Machine::S320C600 as u16).map(|_| Machine::S320C600),
            expect_u16(Machine::AARCH64 as u16).map(|_| Machine::AARCH64),
            expect_u16(Machine::RISCV as u16).map(|_| Machine::RISCV),
            expect_u16(Machine::BPF as u16).map(|_| Machine::BPF),
            expect_u16(Machine::WDC65C817 as u16).map(|_| Machine::WDC65C817),
        ])
        .parse(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntryPoint(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileHeader {
    class: Class,
    endianness: Endianness,
    version: Version,
    abi: ABI,
    r#type: Type,
    machine: Machine,
    vers: Version,
    entry_point: EntryPoint,
}

pub struct FileHeaderParser;

impl<'a> parcel::Parser<'a, &'a [u8], FileHeader> for FileHeaderParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], FileHeader> {
        parcel::right(parcel::join(
            expect_bytes(&[0x7f, 0x45, 0x4c, 0x46]),
            parcel::join(
                parcel::join(ClassParser, EndiannessParser),
                parcel::join(
                    parcel::join(VersionParser, ABIParser),
                    parcel::join(
                        TypeParser,
                        parcel::join(
                            MachineParser,
                            parcel::join(
                                VersionParser,
                                parcel::take_n(parcel::parsers::byte::any_byte(), 8)
                                    .map(|b| u64::from_ne_bytes(b.try_into().unwrap())),
                            ),
                        ),
                    ),
                ),
            ),
        ))
        // skip padding
        .and_then(|last| parcel::take_n(parcel::parsers::byte::any_byte(), 8).map(move |_| last))
        .map(
            |((class, endianness), ((version, abi), (r#type, (machine, (_, entrypoint)))))| {
                FileHeader {
                    class,
                    endianness,
                    version,
                    abi,
                    r#type,
                    machine,
                    vers: version,
                    entry_point: EntryPoint(entrypoint),
                }
            },
        )
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

pub fn expect_u16<'a>(expected: u16) -> impl Parser<'a, &'a [u8], u16> {
    move |input: &'a [u8]| {
        let preparse_input = input;
        let next: Vec<u8> = input.iter().take(2).copied().collect();
        if next == expected.to_ne_bytes() {
            Ok(MatchStatus::Match((&input[2..], expected)))
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
            FileHeaderParser.parse(&input).unwrap().unwrap(),
            FileHeader {
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
