use parcel::prelude::v1::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FileErr {
    InvalidFile,
}

impl std::fmt::Debug for FileErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFile => write!(f, "not an elf formatted file"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EIClass {
    ThirtyTwo,
    SixtyFour,
}

impl From<EIClass> for u8 {
    fn from(src: EIClass) -> Self {
        match src {
            EIClass::ThirtyTwo => 1,
            EIClass::SixtyFour => 2,
        }
    }
}

struct EIClassParser;

impl<'a> parcel::Parser<'a, &'a [u8], EIClass> for EIClassParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], EIClass> {
        parcel::one_of(vec![
            parcel::parsers::byte::expect_byte(0x01).map(|_| EIClass::ThirtyTwo),
            parcel::parsers::byte::expect_byte(0x02).map(|_| EIClass::SixtyFour),
        ])
        .parse(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EIData {
    Little = 0x01,
    Big = 0x02,
}

impl From<EIData> for u8 {
    fn from(src: EIData) -> Self {
        src as u8
    }
}

struct EIDataParser;

impl<'a> parcel::Parser<'a, &'a [u8], EIData> for EIDataParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], EIData> {
        parcel::one_of(vec![
            parcel::parsers::byte::expect_byte(EIData::Little as u8).map(|_| EIData::Little),
            parcel::parsers::byte::expect_byte(EIData::Big as u8).map(|_| EIData::Big),
        ])
        .parse(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EIVersion {
    One = 1,
}

impl From<EIVersion> for u8 {
    fn from(src: EIVersion) -> Self {
        src as u8
    }
}

struct EIVersionParser;

impl<'a> parcel::Parser<'a, &'a [u8], EIVersion> for EIVersionParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], EIVersion> {
        parcel::parsers::byte::expect_byte(EIVersion::One as u8)
            .map(|_| EIVersion::One)
            .parse(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum EIOSABI {
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

impl From<EIOSABI> for u16 {
    fn from(src: EIOSABI) -> Self {
        src as u16
    }
}

struct EIOSABIParser;

impl<'a> parcel::Parser<'a, &'a [u8], EIOSABI> for EIOSABIParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], EIOSABI> {
        parcel::one_of(vec![
            expect_u16(EIOSABI::SysV as u16).map(|_| EIOSABI::SysV),
            expect_u16(EIOSABI::HPUX as u16).map(|_| EIOSABI::HPUX),
            expect_u16(EIOSABI::NetBSD as u16).map(|_| EIOSABI::NetBSD),
            expect_u16(EIOSABI::Linux as u16).map(|_| EIOSABI::Linux),
            expect_u16(EIOSABI::GNUHurd as u16).map(|_| EIOSABI::GNUHurd),
            expect_u16(EIOSABI::Solaris as u16).map(|_| EIOSABI::Solaris),
            expect_u16(EIOSABI::AIX as u16).map(|_| EIOSABI::AIX),
            expect_u16(EIOSABI::IRIX as u16).map(|_| EIOSABI::IRIX),
            expect_u16(EIOSABI::FreeBSD as u16).map(|_| EIOSABI::FreeBSD),
            expect_u16(EIOSABI::Tru64 as u16).map(|_| EIOSABI::Tru64),
            expect_u16(EIOSABI::Novell as u16).map(|_| EIOSABI::Novell),
            expect_u16(EIOSABI::OpenBSD as u16).map(|_| EIOSABI::OpenBSD),
            expect_u16(EIOSABI::OpenVMS as u16).map(|_| EIOSABI::OpenVMS),
            expect_u16(EIOSABI::NonStop as u16).map(|_| EIOSABI::NonStop),
            expect_u16(EIOSABI::Aros as u16).map(|_| EIOSABI::Aros),
            expect_u16(EIOSABI::Fenix as u16).map(|_| EIOSABI::Fenix),
            expect_u16(EIOSABI::CloudABI as u16).map(|_| EIOSABI::CloudABI),
            expect_u16(EIOSABI::OpenVOS as u16).map(|_| EIOSABI::OpenVOS),
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
        use std::convert::TryInto;
        let preparse_input = input;

        // Should be safe to unwrap.
        let mcode = input
            .iter()
            .take(2)
            .copied()
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        match u16::from_ne_bytes(mcode) {
            0x00 => Some(Machine::None),
            _ => None,
        }
        .map_or(Ok(MatchStatus::NoMatch(preparse_input)), |m| {
            Ok(MatchStatus::Match((&preparse_input[2..], m)))
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Version {
    One = 0x01,
}

impl From<Version> for u32 {
    fn from(src: Version) -> Self {
        src as u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryPoint {
    ThirtyTwo(u32),
    SixtyFour(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileHeader {
    ei_class: EIClass,
    ei_data: EIData,
    ei_version: EIVersion,
    ei_osabi: EIOSABI,
    r#type: Type,
    machine: Machine,
    version: Version,
    entry_point: EntryPoint,
}

pub struct FileHeaderParser;

impl FileHeaderParser {
    /// preamble parses the elf magic bytes and class, returning the class if
    /// the elf file has a valid preamble.
    pub fn preamble(input: &[u8]) -> Result<EIClass, FileErr> {
        parcel::right(parcel::join(
            expect_bytes(&[0x7f, 0x45, 0x4c, 0x46]),
            EIClassParser,
        ))
        .parse(input)
        .map(|ms| match ms {
            MatchStatus::Match((_, class)) => Some(class),
            MatchStatus::NoMatch(_) => None,
        })
        .map_err(|_| FileErr::InvalidFile)?
        .ok_or(FileErr::InvalidFile)
    }
}

impl<'a> parcel::Parser<'a, &'a [u8], FileHeader> for FileHeaderParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], FileHeader> {
        let ei_class = Self::preamble(input).map_err(|e| format!("{:?}", e))?;

        parcel::join(EIDataParser, parcel::join(EIVersionParser, EIOSABIParser))
            // skip padding
            .and_then(|last| {
                parcel::take_n(parcel::parsers::byte::any_byte(), 7).map(move |_| last)
            })
            .map(move |(ei_data, (ei_version, ei_osabi))| FileHeader {
                ei_class,
                ei_data,
                ei_version,
                ei_osabi,
                r#type: Type::None,
                machine: Machine::X86_64,
                version: Version::One,
                entry_point: EntryPoint::SixtyFour(0),
            })
            .parse(&input[5..])
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
    fn parse_preamble_should_return_expected_results() {
        let thirty_two_bit_input = [0x7f, 0x45, 0x4c, 0x46, 0x01];
        let sixty_four_bit_input = [0x7f, 0x45, 0x4c, 0x46, 0x02];
        let invalid_input = [0xff, 0xff, 0xff, 0xff, 0xff];

        assert_eq!(
            Ok(EIClass::ThirtyTwo),
            FileHeaderParser::preamble(&thirty_two_bit_input)
        );
        assert_eq!(
            Ok(EIClass::SixtyFour),
            FileHeaderParser::preamble(&sixty_four_bit_input)
        );
        assert!(FileHeaderParser::preamble(&invalid_input).is_err());
    }

    #[test]
    fn parse_known_good_header() {
        let input = [
            0x7f, 0x45, 0x4c, 0x46, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];

        assert_eq!(
            FileHeaderParser.parse(&input).unwrap().unwrap(),
            FileHeader {
                ei_class: EIClass::ThirtyTwo,
                ei_data: EIData::Little,
                ei_version: EIVersion::One,
                ei_osabi: EIOSABI::SysV,
                r#type: Type::None,
                machine: Machine::X86_64,
                version: Version::One,
                entry_point: EntryPoint::SixtyFour(0),
            }
        )
    }
}
