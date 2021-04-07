use parcel::parsers::byte::expect_byte;
use parcel::prelude::v1::*;

// Type Metadata
type Elf32Addr = u32;
type Elf64Addr = u64;

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

/// EiClass contains a 1-byte value representing whether a type is 32 or 64-bit
/// respectively.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EiClass {
    ThirtyTwoBit = 0x01,
    SixtyFourBit = 0x02,
}

impl From<EiClass> for u8 {
    fn from(src: EiClass) -> Self {
        src as u8
    }
}

struct EiClassParser;

impl<'a> parcel::Parser<'a, &'a [u8], EiClass> for EiClassParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], EiClass> {
        parcel::one_of(vec![
            expect_byte(EiClass::ThirtyTwoBit as u8).map(|_| EiClass::ThirtyTwoBit),
            expect_byte(EiClass::SixtyFourBit as u8).map(|_| EiClass::SixtyFourBit),
        ])
        .parse(input)
    }
}

/// EiData stores a 1-byte value representing if the header is in little-endian
/// or big-endian format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EiData {
    Little = 0x01,
    Big = 0x02,
}

impl From<EiData> for u8 {
    fn from(src: EiData) -> Self {
        src as u8
    }
}

/// UnknownDataEncoding is an explicit type for specifying that the data
/// encoding is unknown. This will typically be used when parsing the ident
/// field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UnknownDataEncoding;

/// LittleEndianDataEncoding is an explicit type for specifying that the data
/// encoding is little-endian.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LittleEndianDataEncoding;

/// BigEndianDataEncoding is an explicit type for specifying that the data
/// encoding is big-endian.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BigEndianDataEncoding;

struct EiDataParser;

impl<'a> parcel::Parser<'a, &'a [u8], EiData> for EiDataParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], EiData> {
        parcel::one_of(vec![
            expect_byte(EiData::Little as u8).map(|_| EiData::Little),
            expect_byte(EiData::Big as u8).map(|_| EiData::Big),
        ])
        .parse(input)
    }
}

/// EiVersion represents which version of ELF header is being used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EiVersion {
    One = 1,
}

impl From<EiVersion> for u8 {
    fn from(src: EiVersion) -> Self {
        src as u8
    }
}
struct EiVersionParser;

impl<'a> parcel::Parser<'a, &'a [u8], EiVersion> for EiVersionParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], EiVersion> {
        expect_byte(EiVersion::One as u8)
            .map(|_| EiVersion::One)
            .parse(input)
    }
}

/// EiOSABI represents the target systems ABI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EiOSABI {
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

impl From<EiOSABI> for u8 {
    fn from(src: EiOSABI) -> Self {
        src as u8
    }
}

struct EiOSABIParser;

impl<'a> parcel::Parser<'a, &'a [u8], EiOSABI> for EiOSABIParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], EiOSABI> {
        parcel::one_of(vec![
            expect_byte(EiOSABI::SysV as u8).map(|_| EiOSABI::SysV),
            expect_byte(EiOSABI::HPUX as u8).map(|_| EiOSABI::HPUX),
            expect_byte(EiOSABI::NetBSD as u8).map(|_| EiOSABI::NetBSD),
            expect_byte(EiOSABI::Linux as u8).map(|_| EiOSABI::Linux),
            expect_byte(EiOSABI::GNUHurd as u8).map(|_| EiOSABI::GNUHurd),
            expect_byte(EiOSABI::Solaris as u8).map(|_| EiOSABI::Solaris),
            expect_byte(EiOSABI::AIX as u8).map(|_| EiOSABI::AIX),
            expect_byte(EiOSABI::IRIX as u8).map(|_| EiOSABI::IRIX),
            expect_byte(EiOSABI::FreeBSD as u8).map(|_| EiOSABI::FreeBSD),
            expect_byte(EiOSABI::Tru64 as u8).map(|_| EiOSABI::Tru64),
            expect_byte(EiOSABI::Novell as u8).map(|_| EiOSABI::Novell),
            expect_byte(EiOSABI::OpenBSD as u8).map(|_| EiOSABI::OpenBSD),
            expect_byte(EiOSABI::OpenVMS as u8).map(|_| EiOSABI::OpenVMS),
            expect_byte(EiOSABI::NonStop as u8).map(|_| EiOSABI::NonStop),
            expect_byte(EiOSABI::Aros as u8).map(|_| EiOSABI::Aros),
            expect_byte(EiOSABI::Fenix as u8).map(|_| EiOSABI::Fenix),
            expect_byte(EiOSABI::CloudABI as u8).map(|_| EiOSABI::CloudABI),
            expect_byte(EiOSABI::OpenVOS as u8).map(|_| EiOSABI::OpenVOS),
        ])
        .parse(input)
    }
}

/// EIABIVersion represents the abi version and is often left null.
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

struct TypeParser<E> {
    endianness: std::marker::PhantomData<E>,
}

impl<'a, E> TypeParser<E> {
    fn new() -> Self {
        Self {
            endianness: std::marker::PhantomData,
        }
    }

    fn parse_type(&self, data: EiData, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], Type> {
        parcel::one_of(vec![
            expect_u16(data, Type::None as u16).map(|_| Type::None),
            expect_u16(data, Type::Rel as u16).map(|_| Type::Rel),
            expect_u16(data, Type::Exec as u16).map(|_| Type::Exec),
            expect_u16(data, Type::Dyn as u16).map(|_| Type::Dyn),
            expect_u16(data, Type::Core as u16).map(|_| Type::Core),
            expect_u16(data, Type::LOOS as u16).map(|_| Type::LOOS),
            expect_u16(data, Type::HIOS as u16).map(|_| Type::HIOS),
            expect_u16(data, Type::LOPROC as u16).map(|_| Type::LOPROC),
            expect_u16(data, Type::HIPROC as u16).map(|_| Type::HIPROC),
        ])
        .parse(input)
    }
}

impl<'a> parcel::Parser<'a, &'a [u8], Type> for TypeParser<LittleEndianDataEncoding> {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], Type> {
        self.parse_type(EiData::Little, input)
    }
}

impl<'a> parcel::Parser<'a, &'a [u8], Type> for TypeParser<BigEndianDataEncoding> {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], Type> {
        self.parse_type(EiData::Big, input)
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

impl std::convert::TryFrom<u16> for Machine {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Machine::None),
            0x01 => Ok(Machine::M32),
            0x02 => Ok(Machine::SPARC),
            0x03 => Ok(Machine::X386),
            0x04 => Ok(Machine::M68k),
            0x05 => Ok(Machine::M88k),
            0x06 => Ok(Machine::IntelMCU),
            0x07 => Ok(Machine::Intel80860),
            0x08 => Ok(Machine::MIPS),
            0x09 => Ok(Machine::S370),
            0x0A => Ok(Machine::MIPSRS3LE),
            0x0E => Ok(Machine::PARISC),
            0x13 => Ok(Machine::I960),
            0x14 => Ok(Machine::PPC),
            0x15 => Ok(Machine::PPC64),
            0x16 => Ok(Machine::S390),
            0x24 => Ok(Machine::V800),
            0x25 => Ok(Machine::FR20),
            0x26 => Ok(Machine::RH32),
            0x27 => Ok(Machine::RCE),
            0x28 => Ok(Machine::ARM),
            0x29 => Ok(Machine::Alpha),
            0x2A => Ok(Machine::SH),
            0x2B => Ok(Machine::SPARCV9),
            0x2C => Ok(Machine::Tricore),
            0x2D => Ok(Machine::ARC),
            0x2E => Ok(Machine::H8300),
            0x2F => Ok(Machine::H8_300H),
            0x30 => Ok(Machine::H8s),
            0x31 => Ok(Machine::H8500),
            0x32 => Ok(Machine::IA64),
            0x33 => Ok(Machine::MIPSX),
            0x34 => Ok(Machine::Coldfire),
            0x35 => Ok(Machine::M68HC12),
            0x36 => Ok(Machine::MMA),
            0x37 => Ok(Machine::PCP),
            0x38 => Ok(Machine::NCPU),
            0x39 => Ok(Machine::NDR1),
            0x3a => Ok(Machine::Starcore),
            0x3B => Ok(Machine::ME16),
            0x3C => Ok(Machine::ST100),
            0x3D => Ok(Machine::TinyJ),
            0x3e => Ok(Machine::X86_64),
            0x8C => Ok(Machine::S320C600),
            0xB9 => Ok(Machine::AARCH64),
            0xFA => Ok(Machine::RISCV),
            0xFB => Ok(Machine::BPF),
            0x101 => Ok(Machine::WDC65C817),
            _ => Err(format!("cannot convert {} to Machine variant", value)),
        }
    }
}

struct MachineParser<E> {
    endianness: std::marker::PhantomData<E>,
}

impl<E> MachineParser<E> {
    fn new() -> Self {
        Self {
            endianness: std::marker::PhantomData,
        }
    }
}

impl<'a> parcel::Parser<'a, &'a [u8], Machine> for MachineParser<LittleEndianDataEncoding> {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], Machine> {
        use std::convert::TryInto;
        let preparse_input = input;

        input
            .iter()
            .take(2)
            .copied()
            .collect::<Vec<u8>>()
            .try_into()
            .map(|mcode| std::convert::TryFrom::try_from(u16::from_le_bytes(mcode)))
            .unwrap()
            .map_or(Ok(MatchStatus::NoMatch(preparse_input)), |m| {
                Ok(MatchStatus::Match((&preparse_input[2..], m)))
            })
    }
}

impl<'a> parcel::Parser<'a, &'a [u8], Machine> for MachineParser<BigEndianDataEncoding> {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], Machine> {
        use std::convert::TryInto;
        let preparse_input = input;

        input
            .iter()
            .take(2)
            .copied()
            .collect::<Vec<u8>>()
            .try_into()
            .map(|mcode| std::convert::TryFrom::try_from(u16::from_be_bytes(mcode)))
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

struct VersionParser<E> {
    endianness: std::marker::PhantomData<E>,
}

impl<E> VersionParser<E> {
    fn new() -> Self {
        Self {
            endianness: std::marker::PhantomData,
        }
    }
}

impl<'a> parcel::Parser<'a, &'a [u8], Version> for VersionParser<LittleEndianDataEncoding> {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], Version> {
        expect_u32(EiData::Little, 0x01)
            .map(|_| Version::One)
            .parse(input)
    }
}

impl<'a> parcel::Parser<'a, &'a [u8], Version> for VersionParser<BigEndianDataEncoding> {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], Version> {
        expect_u32(EiData::Big, 0x01)
            .map(|_| Version::One)
            .parse(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// EiIdent defines the elf identification fields that define whether the
/// address size, versions and abi of the file.
pub struct EiIdent {
    pub ei_class: EiClass,
    pub ei_data: EiData,
    pub ei_version: EiVersion,
    pub ei_osabi: EiOSABI,
    pub ei_abiversion: EIABIVersion,
}

/// EiIdentParser defines a parser for parsing a raw bitstream into an EiIdent.
struct EiIdentParser;

impl<'a> parcel::Parser<'a, &'a [u8], EiIdent> for EiIdentParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], EiIdent> {
        parcel::right(parcel::join(
            expect_bytes(&[0x7f, 0x45, 0x4c, 0x46]),
            parcel::join(
                EiClassParser,
                parcel::join(
                    EiDataParser,
                    parcel::join(
                        EiVersionParser,
                        parcel::join(EiOSABIParser, EIABIVersionParser),
                    ),
                ),
            )
            // skip padding
            .and_then(|last| {
                parcel::take_n(parcel::parsers::byte::any_byte(), 7).map(move |_| last)
            }),
        ))
        .map(
            |(ei_class, (ei_data, (ei_version, (ei_osabi, ei_abiversion))))| EiIdent {
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
    ei_ident: EiIdent,
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
pub struct FileHeaderParser<A, E> {
    address_width: std::marker::PhantomData<A>,
    endianness: std::marker::PhantomData<E>,
}

impl<E> FileHeaderParser<u32, E> {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            address_width: std::marker::PhantomData,
            endianness: std::marker::PhantomData,
        }
    }
}

impl<E> FileHeaderParser<u64, E> {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            address_width: std::marker::PhantomData,
            endianness: std::marker::PhantomData,
        }
    }
}

impl<A, E> FileHeaderParser<A, E> {
    /// identifier parses the elf magic bytes and class, returning the class if
    /// the elf file has a valid preamble.
    pub fn identifier(input: &[u8]) -> Result<EiIdent, FileErr> {
        EiIdentParser
            .parse(input)
            .map(|ms| match ms {
                MatchStatus::Match((_, ei_ident)) => Some(ei_ident),
                MatchStatus::NoMatch(_) => None,
            })
            .map_err(|_| FileErr::InvalidFile)?
            .ok_or(FileErr::InvalidFile)
    }
}

impl<'a, E> parcel::Parser<'a, &'a [u8], FileHeader<Elf32Addr>> for FileHeaderParser<Elf32Addr, E> {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], FileHeader<Elf32Addr>> {
        let ei_ident = Self::identifier(input).map_err(|e| format!("{:?}", e))?;

        parcel::join(
            TypeParser::<LittleEndianDataEncoding>::new(),
            parcel::join(
                MachineParser::<LittleEndianDataEncoding>::new(),
                parcel::join(
                    VersionParser::<LittleEndianDataEncoding>::new(),
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

impl<'a> parcel::Parser<'a, &'a [u8], FileHeader<Elf64Addr>>
    for FileHeaderParser<Elf64Addr, LittleEndianDataEncoding>
{
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], FileHeader<Elf64Addr>> {
        let ei_ident = Self::identifier(input).map_err(|e| format!("{:?}", e))?;
        let encoding = EiData::Little;

        parcel::join(
            TypeParser::<LittleEndianDataEncoding>::new(),
            parcel::join(
                MachineParser::<LittleEndianDataEncoding>::new(),
                parcel::join(
                    VersionParser::<LittleEndianDataEncoding>::new(),
                    parcel::join(
                        match_u64(encoding),
                        parcel::join(
                            match_u64(encoding),
                            parcel::join(
                                match_u64(encoding),
                                parcel::join(
                                    match_u32(encoding),
                                    parcel::take_n(match_u16(encoding), 6),
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

impl<'a> parcel::Parser<'a, &'a [u8], FileHeader<Elf64Addr>>
    for FileHeaderParser<Elf64Addr, BigEndianDataEncoding>
{
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], FileHeader<Elf64Addr>> {
        let ei_ident = Self::identifier(input).map_err(|e| format!("{:?}", e))?;
        let encoding = EiData::Big;

        parcel::join(
            TypeParser::<BigEndianDataEncoding>::new(),
            parcel::join(
                MachineParser::<BigEndianDataEncoding>::new(),
                parcel::join(
                    VersionParser::<BigEndianDataEncoding>::new(),
                    parcel::join(
                        match_u64(encoding),
                        parcel::join(
                            match_u64(encoding),
                            parcel::join(
                                match_u64(encoding),
                                parcel::join(
                                    match_u32(encoding),
                                    parcel::take_n(match_u16(encoding), 6),
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
fn expect_u16<'a>(endianness: EiData, expected: u16) -> impl Parser<'a, &'a [u8], u16> {
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
fn expect_u32<'a>(endianness: EiData, expected: u32) -> impl Parser<'a, &'a [u8], u32> {
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

/// Matches any given u16 by endianness returning a corresponding u16 value.
fn match_u16<'a>(endianness: EiData) -> impl Parser<'a, &'a [u8], u16> {
    use parcel::parsers::byte::any_byte;
    use std::convert::TryInto;

    parcel::take_n(any_byte(), 2).map(move |b| {
        b.try_into()
            .map(|ep| match endianness {
                EiData::Little => u16::from_le_bytes(ep),
                EiData::Big => u16::from_be_bytes(ep),
            })
            .unwrap()
    })
}

/// Matches any given u32 by endianness returning a corresponding u32 value.
fn match_u32<'a>(endianness: EiData) -> impl Parser<'a, &'a [u8], u32> {
    use parcel::parsers::byte::any_byte;
    use std::convert::TryInto;

    parcel::take_n(any_byte(), 4).map(move |b| {
        b.try_into()
            .map(|ep| match endianness {
                EiData::Little => u32::from_le_bytes(ep),
                EiData::Big => u32::from_be_bytes(ep),
            })
            .unwrap()
    })
}

/// Matches any given u64 by endianness returning a corresponding u64 value.
fn match_u64<'a>(endianness: EiData) -> impl Parser<'a, &'a [u8], u64> {
    use parcel::parsers::byte::any_byte;
    use std::convert::TryInto;

    parcel::take_n(any_byte(), 8).map(move |b| {
        b.try_into()
            .map(|ep| match endianness {
                EiData::Little => u64::from_le_bytes(ep),
                EiData::Big => u64::from_be_bytes(ep),
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
            Ok(EiClass::ThirtyTwoBit),
            FileHeaderParser::<Elf32Addr, UnknownDataEncoding>::identifier(&thirty_two_bit_input)
                .map(|ident| ident.ei_class)
        );
        assert_eq!(
            Ok(EiClass::SixtyFourBit),
            FileHeaderParser::<Elf64Addr, UnknownDataEncoding>::identifier(&sixty_four_bit_input)
                .map(|ident| ident.ei_class)
        );
        assert!(
            FileHeaderParser::<Elf64Addr, UnknownDataEncoding>::identifier(&invalid_input).is_err()
        );
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
            FileHeaderParser::<Elf32Addr, LittleEndianDataEncoding>::new()
                .parse(&input)
                .unwrap()
                .unwrap(),
            FileHeader::<u32> {
                ei_ident: EiIdent {
                    ei_class: EiClass::ThirtyTwoBit,
                    ei_data: EiData::Little,
                    ei_version: EiVersion::One,
                    ei_osabi: EiOSABI::SysV,
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
