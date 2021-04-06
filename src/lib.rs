use parcel::parsers::byte::expect_byte;
use parcel::prelude::v1::*;

// Type Metadata
type ELF32Addr = u32;
type ELF64Addr = u64;

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
#[repr(u8)]
pub enum EIClass {
    ThirtyTwoBit = 0x01,
    SixtyFourBit = 0x02,
}

impl From<EIClass> for u8 {
    fn from(src: EIClass) -> Self {
        src as u8
    }
}

struct EIClassParser;

impl<'a> parcel::Parser<'a, &'a [u8], EIClass> for EIClassParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], EIClass> {
        parcel::one_of(vec![
            expect_byte(EIClass::ThirtyTwoBit as u8).map(|_| EIClass::ThirtyTwoBit),
            expect_byte(EIClass::SixtyFourBit as u8).map(|_| EIClass::SixtyFourBit),
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
            expect_byte(EIData::Little as u8).map(|_| EIData::Little),
            expect_byte(EIData::Big as u8).map(|_| EIData::Big),
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
        expect_byte(EIVersion::One as u8)
            .map(|_| EIVersion::One)
            .parse(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
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

impl From<EIOSABI> for u8 {
    fn from(src: EIOSABI) -> Self {
        src as u8
    }
}

struct EIOSABIParser;

impl<'a> parcel::Parser<'a, &'a [u8], EIOSABI> for EIOSABIParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], EIOSABI> {
        parcel::one_of(vec![
            expect_byte(EIOSABI::SysV as u8).map(|_| EIOSABI::SysV),
            expect_byte(EIOSABI::HPUX as u8).map(|_| EIOSABI::HPUX),
            expect_byte(EIOSABI::NetBSD as u8).map(|_| EIOSABI::NetBSD),
            expect_byte(EIOSABI::Linux as u8).map(|_| EIOSABI::Linux),
            expect_byte(EIOSABI::GNUHurd as u8).map(|_| EIOSABI::GNUHurd),
            expect_byte(EIOSABI::Solaris as u8).map(|_| EIOSABI::Solaris),
            expect_byte(EIOSABI::AIX as u8).map(|_| EIOSABI::AIX),
            expect_byte(EIOSABI::IRIX as u8).map(|_| EIOSABI::IRIX),
            expect_byte(EIOSABI::FreeBSD as u8).map(|_| EIOSABI::FreeBSD),
            expect_byte(EIOSABI::Tru64 as u8).map(|_| EIOSABI::Tru64),
            expect_byte(EIOSABI::Novell as u8).map(|_| EIOSABI::Novell),
            expect_byte(EIOSABI::OpenBSD as u8).map(|_| EIOSABI::OpenBSD),
            expect_byte(EIOSABI::OpenVMS as u8).map(|_| EIOSABI::OpenVMS),
            expect_byte(EIOSABI::NonStop as u8).map(|_| EIOSABI::NonStop),
            expect_byte(EIOSABI::Aros as u8).map(|_| EIOSABI::Aros),
            expect_byte(EIOSABI::Fenix as u8).map(|_| EIOSABI::Fenix),
            expect_byte(EIOSABI::CloudABI as u8).map(|_| EIOSABI::CloudABI),
            expect_byte(EIOSABI::OpenVOS as u8).map(|_| EIOSABI::OpenVOS),
        ])
        .parse(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EIABIVersion {
    Zero = 0x00,
    One = 0x01,
}

impl From<EIABIVersion> for u8 {
    fn from(src: EIABIVersion) -> Self {
        src as u8
    }
}

struct EIABIVersionParser;

impl<'a> parcel::Parser<'a, &'a [u8], EIABIVersion> for EIABIVersionParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], EIABIVersion> {
        parcel::one_of(vec![
            expect_byte(EIABIVersion::Zero as u8).map(|_| EIABIVersion::Zero),
            expect_byte(EIABIVersion::One as u8).map(|_| EIABIVersion::One),
        ])
        .parse(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
enum Type {
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

struct TypeParser(EIData);

impl<'a> parcel::Parser<'a, &'a [u8], Type> for TypeParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], Type> {
        let class = self.0;
        parcel::one_of(vec![
            expect_u16(class, Type::None as u16).map(|_| Type::None),
            expect_u16(class, Type::Rel as u16).map(|_| Type::Rel),
            expect_u16(class, Type::Exec as u16).map(|_| Type::Exec),
            expect_u16(class, Type::Dyn as u16).map(|_| Type::Dyn),
            expect_u16(class, Type::Core as u16).map(|_| Type::Core),
            expect_u16(class, Type::LOOS as u16).map(|_| Type::LOOS),
            expect_u16(class, Type::HIOS as u16).map(|_| Type::HIOS),
            expect_u16(class, Type::LOPROC as u16).map(|_| Type::LOPROC),
            expect_u16(class, Type::HIPROC as u16).map(|_| Type::HIPROC),
        ])
        .parse(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
enum Machine {
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

struct MachineParser(EIData);

impl<'a> parcel::Parser<'a, &'a [u8], Machine> for MachineParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], Machine> {
        use std::convert::TryInto;
        let preparse_input = input;
        let endianness = self.0;

        input
            .iter()
            .take(2)
            .copied()
            .collect::<Vec<u8>>()
            .try_into()
            .map(|mcode| {
                let val = match endianness {
                    EIData::Little => u16::from_le_bytes(mcode),
                    EIData::Big => u16::from_be_bytes(mcode),
                };
                match val {
                    0x00 => Some(Machine::None),
                    0x01 => Some(Machine::M32),
                    0x02 => Some(Machine::SPARC),
                    0x03 => Some(Machine::X386),
                    0x04 => Some(Machine::M68k),
                    0x05 => Some(Machine::M88k),
                    0x06 => Some(Machine::IntelMCU),
                    0x07 => Some(Machine::Intel80860),
                    0x08 => Some(Machine::MIPS),
                    0x09 => Some(Machine::S370),
                    0x0A => Some(Machine::MIPSRS3LE),
                    0x0E => Some(Machine::PARISC),
                    0x13 => Some(Machine::I960),
                    0x14 => Some(Machine::PPC),
                    0x15 => Some(Machine::PPC64),
                    0x16 => Some(Machine::S390),
                    0x24 => Some(Machine::V800),
                    0x25 => Some(Machine::FR20),
                    0x26 => Some(Machine::RH32),
                    0x27 => Some(Machine::RCE),
                    0x28 => Some(Machine::ARM),
                    0x29 => Some(Machine::Alpha),
                    0x2A => Some(Machine::SH),
                    0x2B => Some(Machine::SPARCV9),
                    0x2C => Some(Machine::Tricore),
                    0x2D => Some(Machine::ARC),
                    0x2E => Some(Machine::H8300),
                    0x2F => Some(Machine::H8_300H),
                    0x30 => Some(Machine::H8s),
                    0x31 => Some(Machine::H8500),
                    0x32 => Some(Machine::IA64),
                    0x33 => Some(Machine::MIPSX),
                    0x34 => Some(Machine::Coldfire),
                    0x35 => Some(Machine::M68HC12),
                    0x36 => Some(Machine::MMA),
                    0x37 => Some(Machine::PCP),
                    0x38 => Some(Machine::NCPU),
                    0x39 => Some(Machine::NDR1),
                    0x3a => Some(Machine::Starcore),
                    0x3B => Some(Machine::ME16),
                    0x3C => Some(Machine::ST100),
                    0x3D => Some(Machine::TinyJ),
                    0x3e => Some(Machine::X86_64),
                    0x8C => Some(Machine::S320C600),
                    0xB9 => Some(Machine::AARCH64),
                    0xFa => Some(Machine::RISCV),
                    0xFB => Some(Machine::BPF),
                    0x101 => Some(Machine::WDC65C817),
                    _ => None,
                }
            })
            .unwrap()
            .map_or(Ok(MatchStatus::NoMatch(preparse_input)), |m| {
                Ok(MatchStatus::Match((&preparse_input[2..], m)))
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum Version {
    One = 0x01,
}

impl From<Version> for u32 {
    fn from(src: Version) -> Self {
        src as u32
    }
}

struct VersionParser(EIData);

impl<'a> parcel::Parser<'a, &'a [u8], Version> for VersionParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], Version> {
        expect_u32(self.0, 0x01).map(|_| Version::One).parse(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// EIIdent defines the elf identification fields that define whether the
/// address size, versions and abi of the file.
pub struct EIIdent {
    pub ei_class: EIClass,
    pub ei_data: EIData,
    pub ei_version: EIVersion,
    pub ei_osabi: EIOSABI,
    pub ei_abiversion: EIABIVersion,
}

/// EIIdentParser defines a parser for parsing a raw bitstream into an EIIdent.
pub struct EIIdentParser;

impl<'a> parcel::Parser<'a, &'a [u8], EIIdent> for EIIdentParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], EIIdent> {
        parcel::right(parcel::join(
            expect_bytes(&[0x7f, 0x45, 0x4c, 0x46]),
            parcel::join(
                EIClassParser,
                parcel::join(
                    EIDataParser,
                    parcel::join(
                        EIVersionParser,
                        parcel::join(EIOSABIParser, EIABIVersionParser),
                    ),
                ),
            )
            // skip padding
            .and_then(|last| {
                parcel::take_n(parcel::parsers::byte::any_byte(), 7).map(move |_| last)
            }),
        ))
        .map(
            |(ei_class, (ei_data, (ei_version, (ei_osabi, ei_abiversion))))| EIIdent {
                ei_class,
                ei_data,
                ei_version,
                ei_osabi,
                ei_abiversion,
            },
        )
        .parse(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// FileHeader represents a program file header, and contains ELF identifaction
/// information along with sizing, architechture and additional metadata about
/// other ELF headers.
pub struct FileHeader<AddrWidth> {
    ei_ident: EIIdent,
    r#type: Type,
    machine: Machine,
    version: Version,
    entry_point: AddrWidth,
    ph_offset: AddrWidth,
    sh_offset: AddrWidth,
    flags: u32,
    eh_size: u16,
    phent_size: u16,
    phnum: u16,
    shent_size: u16,
    shnum: u16,
    shstrndx: u16,
}

/// FileHeaderParser defines a parser for parsing a raw bitstream into a FileHeader.
pub struct FileHeaderParser<A> {
    address_width: std::marker::PhantomData<A>,
}

impl FileHeaderParser<u32> {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            address_width: std::marker::PhantomData,
        }
    }
}

impl FileHeaderParser<u64> {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            address_width: std::marker::PhantomData,
        }
    }
}

impl<A> FileHeaderParser<A> {
    /// identifier parses the elf magic bytes and class, returning the class if
    /// the elf file has a valid preamble.
    pub fn identifier(input: &[u8]) -> Result<EIIdent, FileErr> {
        EIIdentParser
            .parse(input)
            .map(|ms| match ms {
                MatchStatus::Match((_, ei_ident)) => Some(ei_ident),
                MatchStatus::NoMatch(_) => None,
            })
            .map_err(|_| FileErr::InvalidFile)?
            .ok_or(FileErr::InvalidFile)
    }
}

impl<'a> parcel::Parser<'a, &'a [u8], FileHeader<ELF32Addr>> for FileHeaderParser<ELF32Addr> {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], FileHeader<ELF32Addr>> {
        let ei_ident = Self::identifier(input).map_err(|e| format!("{:?}", e))?;

        parcel::join(
            TypeParser(ei_ident.ei_data),
            parcel::join(
                MachineParser(ei_ident.ei_data),
                parcel::join(
                    VersionParser(ei_ident.ei_data),
                    parcel::join(
                        match_u32(ei_ident.ei_data),
                        parcel::join(
                            match_u32(ei_ident.ei_data),
                            parcel::join(
                                match_u32(ei_ident.ei_data),
                                parcel::join(
                                    match_u32(ei_ident.ei_data),
                                    parcel::take_n(match_u16(ei_ident.ei_data), 6),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        )
        .map(
            move |(
                r#type,
                (
                    machine,
                    (version, (entry_point, (ph_offset, (sh_offset, (flags, two_byte_fields))))),
                ),
            )| {
                FileHeader {
                    ei_ident,
                    r#type,
                    machine,
                    version,
                    entry_point,
                    ph_offset,
                    sh_offset,
                    flags,
                    eh_size: two_byte_fields[0],
                    phent_size: two_byte_fields[1],
                    phnum: two_byte_fields[2],
                    shent_size: two_byte_fields[3],
                    shnum: two_byte_fields[4],
                    shstrndx: two_byte_fields[5],
                }
            },
        )
        .parse(&input[16..])
    }
}

impl<'a> parcel::Parser<'a, &'a [u8], FileHeader<ELF64Addr>> for FileHeaderParser<ELF64Addr> {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], FileHeader<ELF64Addr>> {
        let ei_ident = Self::identifier(input).map_err(|e| format!("{:?}", e))?;

        parcel::join(
            TypeParser(ei_ident.ei_data),
            parcel::join(
                MachineParser(ei_ident.ei_data),
                parcel::join(
                    VersionParser(ei_ident.ei_data),
                    parcel::join(
                        match_u64(ei_ident.ei_data),
                        parcel::join(
                            match_u64(ei_ident.ei_data),
                            parcel::join(
                                match_u64(ei_ident.ei_data),
                                parcel::join(
                                    match_u32(ei_ident.ei_data),
                                    parcel::take_n(match_u16(ei_ident.ei_data), 6),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        )
        .map(
            move |(
                r#type,
                (
                    machine,
                    (version, (entry_point, (ph_offset, (sh_offset, (flags, two_byte_fields))))),
                ),
            )| {
                FileHeader {
                    ei_ident,
                    r#type,
                    machine,
                    version,
                    entry_point,
                    ph_offset,
                    sh_offset,
                    flags,
                    eh_size: two_byte_fields[0],
                    phent_size: two_byte_fields[1],
                    phnum: two_byte_fields[2],
                    shent_size: two_byte_fields[3],
                    shnum: two_byte_fields[4],
                    shstrndx: two_byte_fields[5],
                }
            },
        )
        .parse(&input[16..])
    }
}

/// Matches a single provided static byte array, returning a match if the next
/// bytes in the array match the expected byte array. Otherwise, a `NoMatch` is
/// returned.
fn expect_bytes<'a>(expected: &'static [u8]) -> impl Parser<'a, &'a [u8], Vec<u8>> {
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

/// Matches a single provided static u16, returning a match if the next
/// two bytes in the array match the expected u16. Otherwise, a `NoMatch` is
/// returned.
fn expect_u16<'a>(endianness: EIData, expected: u16) -> impl Parser<'a, &'a [u8], u16> {
    move |input: &'a [u8]| {
        let preparse_input = input;
        match match_u16(endianness).parse(input) {
            Ok(MatchStatus::Match((rem, v))) if v == expected => {
                Ok(MatchStatus::Match((rem, expected)))
            }
            _ => Ok(MatchStatus::NoMatch(preparse_input)),
        }
    }
}

/// Matches a single provided static u32, returning a match if the next
/// four bytes in the array match the expected u32. Otherwise, a `NoMatch` is
/// returned.
fn expect_u32<'a>(endianness: EIData, expected: u32) -> impl Parser<'a, &'a [u8], u32> {
    move |input: &'a [u8]| {
        let preparse_input = input;
        match match_u32(endianness).parse(input) {
            Ok(MatchStatus::Match((rem, v))) if v == expected => {
                Ok(MatchStatus::Match((rem, expected)))
            }
            _ => Ok(MatchStatus::NoMatch(preparse_input)),
        }
    }
}

fn match_u16<'a>(endianness: EIData) -> impl Parser<'a, &'a [u8], u16> {
    use parcel::parsers::byte::any_byte;
    use std::convert::TryInto;

    parcel::take_n(any_byte(), 2).map(move |b| {
        b.try_into()
            .map(|ep| match endianness {
                EIData::Little => u16::from_le_bytes(ep),
                EIData::Big => u16::from_be_bytes(ep),
            })
            .unwrap()
    })
}

fn match_u32<'a>(endianness: EIData) -> impl Parser<'a, &'a [u8], u32> {
    use parcel::parsers::byte::any_byte;
    use std::convert::TryInto;

    parcel::take_n(any_byte(), 4).map(move |b| {
        b.try_into()
            .map(|b| b)
            .map(|ep| match endianness {
                EIData::Little => u32::from_le_bytes(ep),
                EIData::Big => u32::from_be_bytes(ep),
            })
            .unwrap()
    })
}

fn match_u64<'a>(endianness: EIData) -> impl Parser<'a, &'a [u8], u64> {
    use parcel::parsers::byte::any_byte;
    use std::convert::TryInto;

    parcel::take_n(any_byte(), 8).map(move |b| {
        b.try_into()
            .map(|ep| match endianness {
                EIData::Little => u64::from_le_bytes(ep),
                EIData::Big => u64::from_be_bytes(ep),
            })
            .unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_preamble_should_return_expected_results() {
        let thirty_two_bit_input = [
            0x7f, 0x45, 0x4c, 0x46, 0x01, 0x01, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];
        let sixty_four_bit_input = [
            0x7f, 0x45, 0x4c, 0x46, 0x02, 0x01, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];
        let invalid_input = [0xff, 0xff, 0xff, 0xff, 0xff];

        assert_eq!(
            Ok(EIClass::ThirtyTwoBit),
            FileHeaderParser::<ELF32Addr>::identifier(&thirty_two_bit_input)
                .map(|ident| ident.ei_class)
        );
        assert_eq!(
            Ok(EIClass::SixtyFourBit),
            FileHeaderParser::<ELF64Addr>::identifier(&sixty_four_bit_input)
                .map(|ident| ident.ei_class)
        );
        assert!(FileHeaderParser::<ELF64Addr>::identifier(&invalid_input).is_err());
    }

    #[test]
    fn parse_known_good_header() {
        #[rustfmt::skip]
        let input: Vec<u8> = vec![
            0x7f, 0x45, 0x4c, 0x46, // magic
            0x01, // ei_class
            0x01, // ei_data
            0x01, // ei_version
            0x00, // ei_osabi
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // padding 
            0x00, 0x00, // type
            0x03, 0x00, // machine
            0x01, 0x00, 0x00, 0x00, //version
            0x05, 0x00, 0x00, 0x00, // entry
            0x0A, 0x00, 0x00, 0x00, // phoff
            0x0B, 0x00, 0x00, 0x00, // shoff
            0x02, 0x00, 0x00, 0x00, // flags
            0x00, 0x00, // eh_size
            0x01, 0x00, // phentsize
            0x01, 0x00, // phnum
            0x01, 0x00, // shentsize
            0x01, 0x00, // shnum
            0x01, 0x00, // shstrndx
        ];

        assert_eq!(
            FileHeaderParser::<ELF32Addr>::new()
                .parse(&input)
                .unwrap()
                .unwrap(),
            FileHeader::<u32> {
                ei_ident: EIIdent {
                    ei_class: EIClass::ThirtyTwoBit,
                    ei_data: EIData::Little,
                    ei_version: EIVersion::One,
                    ei_osabi: EIOSABI::SysV,
                    ei_abiversion: EIABIVersion::One,
                },
                r#type: Type::None,
                machine: Machine::X386,
                version: Version::One,
                entry_point: 5u32,
                ph_offset: 10u32,
                sh_offset: 11u32,
                flags: 2,
                eh_size: 0,
                phent_size: 1,
                phnum: 1,
                shent_size: 1,
                shnum: 1,
                shstrndx: 1
            }
        )
    }
}
