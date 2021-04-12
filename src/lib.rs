use parcel::parsers::byte::expect_byte;
use parcel::prelude::v1::*;

// Type Metadata

/// AddressWidth represents a variant of address size. This should, for the
/// most part be either u32 or u64 for ELF.
pub trait AddressWidth {}

type Elf32Addr = u32;

impl AddressWidth for Elf32Addr {}

type Elf64Addr = u64;

impl AddressWidth for Elf64Addr {}

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

impl std::fmt::Display for EiClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            EiClass::ThirtyTwoBit => "ELF32",
            EiClass::SixtyFourBit => "ELF64",
        };

        write!(f, "{}", repr)
    }
}

impl From<EiClass> for u8 {
    fn from(src: EiClass) -> Self {
        src as u8
    }
}

impl From<Elf32Addr> for EiClass {
    fn from(_: Elf32Addr) -> Self {
        EiClass::ThirtyTwoBit
    }
}

impl From<Elf64Addr> for EiClass {
    fn from(_: Elf64Addr) -> Self {
        EiClass::SixtyFourBit
    }
}

/// EiClassParser functiona as a wrapper struct for parsing binary data into
/// an EiClass.
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

impl std::fmt::Display for EiData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            EiData::Little => "little endian",
            EiData::Big => "big endian",
        };

        write!(f, "{}", repr)
    }
}

impl From<EiData> for u8 {
    fn from(src: EiData) -> Self {
        src as u8
    }
}

impl From<LittleEndianDataEncoding> for EiData {
    fn from(_: LittleEndianDataEncoding) -> Self {
        Self::Little
    }
}

impl From<BigEndianDataEncoding> for EiData {
    fn from(_: BigEndianDataEncoding) -> Self {
        Self::Big
    }
}

/// DataEncoding is a 0 method trait that is used for implementing a
/// genericized Endianness encoding
pub trait DataEncoding {}

/// UnknownDataEncoding is an explicit type for specifying that the data
/// encoding is unknown. This will typically be used when parsing the ident
/// field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnknownDataEncoding;

/// LittleEndianDataEncoding is an explicit type for specifying that the data
/// encoding is little-endian.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct LittleEndianDataEncoding;

impl DataEncoding for LittleEndianDataEncoding {}

/// BigEndianDataEncoding is an explicit type for specifying that the data
/// encoding is big-endian.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct BigEndianDataEncoding;

impl DataEncoding for BigEndianDataEncoding {}

/// EiDataParser attempts to parse if a binary is little or big endian.
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
    One = 0x01,
}

impl std::fmt::Display for EiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl From<EiVersion> for u8 {
    fn from(src: EiVersion) -> Self {
        src as u8
    }
}

/// EiVersionParser should only match a single version, the 0x01 byte.
struct EiVersionParser;

impl<'a> parcel::Parser<'a, &'a [u8], EiVersion> for EiVersionParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], EiVersion> {
        expect_byte(EiVersion::One as u8)
            .map(|_| EiVersion::One)
            .parse(input)
    }
}

/// EiOsAbi represents the target systems ABI.
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EiOsAbi {
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

impl From<EiOsAbi> for u8 {
    fn from(src: EiOsAbi) -> Self {
        src as u8
    }
}

/// EiOsAbiParser parses an EiOsAbi value.
struct EiOsAbiParser;

impl<'a> parcel::Parser<'a, &'a [u8], EiOsAbi> for EiOsAbiParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], EiOsAbi> {
        parcel::one_of(vec![
            expect_byte(EiOsAbi::SysV as u8).map(|_| EiOsAbi::SysV),
            expect_byte(EiOsAbi::HPUX as u8).map(|_| EiOsAbi::HPUX),
            expect_byte(EiOsAbi::NetBSD as u8).map(|_| EiOsAbi::NetBSD),
            expect_byte(EiOsAbi::Linux as u8).map(|_| EiOsAbi::Linux),
            expect_byte(EiOsAbi::GNUHurd as u8).map(|_| EiOsAbi::GNUHurd),
            expect_byte(EiOsAbi::Solaris as u8).map(|_| EiOsAbi::Solaris),
            expect_byte(EiOsAbi::AIX as u8).map(|_| EiOsAbi::AIX),
            expect_byte(EiOsAbi::IRIX as u8).map(|_| EiOsAbi::IRIX),
            expect_byte(EiOsAbi::FreeBSD as u8).map(|_| EiOsAbi::FreeBSD),
            expect_byte(EiOsAbi::Tru64 as u8).map(|_| EiOsAbi::Tru64),
            expect_byte(EiOsAbi::Novell as u8).map(|_| EiOsAbi::Novell),
            expect_byte(EiOsAbi::OpenBSD as u8).map(|_| EiOsAbi::OpenBSD),
            expect_byte(EiOsAbi::OpenVMS as u8).map(|_| EiOsAbi::OpenVMS),
            expect_byte(EiOsAbi::NonStop as u8).map(|_| EiOsAbi::NonStop),
            expect_byte(EiOsAbi::Aros as u8).map(|_| EiOsAbi::Aros),
            expect_byte(EiOsAbi::Fenix as u8).map(|_| EiOsAbi::Fenix),
            expect_byte(EiOsAbi::CloudABI as u8).map(|_| EiOsAbi::CloudABI),
            expect_byte(EiOsAbi::OpenVOS as u8).map(|_| EiOsAbi::OpenVOS),
        ])
        .parse(input)
    }
}

impl std::fmt::Display for EiOsAbi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            EiOsAbi::SysV => "UNIX - System V",
            EiOsAbi::HPUX => "UNIX - HP-UX",
            EiOsAbi::NetBSD => "UNIX - NetBSD",
            EiOsAbi::Linux => "Linux C6000",
            EiOsAbi::GNUHurd => "Unix - GNU",
            EiOsAbi::Solaris => "UNIX - Solaris",
            EiOsAbi::AIX => "UNIX - AIX",
            EiOsAbi::IRIX => "UNIX - IRIX",
            EiOsAbi::FreeBSD => "UNIX - FreeBSD",
            EiOsAbi::Tru64 => "UNIX - TRU64",
            EiOsAbi::Novell => "Novell - Modesto",
            EiOsAbi::OpenBSD => "Unix - OpenBSD",
            EiOsAbi::OpenVMS => "VMS - OpenVMS",
            EiOsAbi::NonStop => "HP - Non-Stop Kernel",
            EiOsAbi::Aros => "Aros",
            EiOsAbi::Fenix => "FenixOS",
            EiOsAbi::CloudABI => "Nuxi CloudABI",
            EiOsAbi::OpenVOS => "Stratus Technologies OpenVOS",
        };

        write!(f, "{}", repr)
    }
}

/// EiAbiVersion represents the abi version and is often left null.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EiAbiVersion {
    Zero = 0x00,
    One = 0x01,
}

impl std::fmt::Display for EiAbiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u32)
    }
}

impl From<EiAbiVersion> for u8 {
    fn from(src: EiAbiVersion) -> Self {
        src as u8
    }
}

/// EiAbiVersionParser attempts to parse an EiAbiVersion.
struct EiAbiVersionParser;

impl<'a> parcel::Parser<'a, &'a [u8], EiAbiVersion> for EiAbiVersionParser {
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], EiAbiVersion> {
        parcel::one_of(vec![
            expect_byte(EiAbiVersion::Zero as u8).map(|_| EiAbiVersion::Zero),
            expect_byte(EiAbiVersion::One as u8).map(|_| EiAbiVersion::One),
        ])
        .parse(input)
    }
}

/// Type represents the type of ELF header for example executable or
/// dynamically-linkable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Type {
    None = 0x00,
    Rel = 0x01,
    Exec = 0x02,
    Dyn = 0x03,
    Core = 0x04,
    LoOs = 0xFE00,
    HiOs = 0xFEFF,
    LoProc = 0xFF00,
    HiProc = 0xFFFF,
}

impl From<Type> for u16 {
    fn from(src: Type) -> Self {
        src as u16
    }
}

/// TypeParse takes a DataEncoding parameter representing endianness and
/// attempts to parse a Type.
pub struct TypeParser<E>
where
    E: DataEncoding,
{
    endianness: std::marker::PhantomData<E>,
}

impl<'a, E> TypeParser<E>
where
    E: DataEncoding,
{
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
            expect_u16(data, Type::LoOs as u16).map(|_| Type::LoOs),
            expect_u16(data, Type::HiOs as u16).map(|_| Type::HiOs),
            expect_u16(data, Type::LoProc as u16).map(|_| Type::LoProc),
            expect_u16(data, Type::HiProc as u16).map(|_| Type::HiProc),
        ])
        .parse(input)
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Type::None => "None (None)",
            Type::Rel => "REL (Relocatable file)",
            Type::Exec => "EXEC (Executable file)",
            Type::Dyn => "DYN (Shared object file)",
            Type::Core => "CORE (Core file)",
            Type::LoOs => "OS Specific: (LoOs)",
            Type::HiOs => "OS Specific: (HiOs)",
            Type::LoProc => "Processor Specific: (LoProc)",
            Type::HiProc => " Processor Specific: (HiProc)",
        };

        write!(f, "{}", repr)
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

/// Machine represents a machine architecture for a given binary represented as
/// a u16.
#[allow(clippy::clippy::upper_case_acronyms)]
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
    MCS6502 = 0xFE,
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
            0xFE => Ok(Machine::MCS6502),
            0x101 => Ok(Machine::WDC65C817),
            _ => Err(format!("cannot convert {} to Machine variant", value)),
        }
    }
}

impl std::fmt::Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Machine::None => "None",
            Machine::M32 => "WE32100",
            Machine::SPARC => "Sparc",
            Machine::X386 => "Intel 80386",
            Machine::M68k => "MC68000",
            Machine::M88k => "MC88000",
            Machine::IntelMCU => "Intel MCU",
            Machine::Intel80860 => "Intel 80860",
            Machine::MIPS => "MIPS R3000",
            Machine::S370 => "IBM System/370",
            Machine::MIPSRS3LE => "MIPS R4000 big-endian",
            Machine::PARISC => "HPPA",
            Machine::I960 => "Intel 80960",
            Machine::PPC => "PowerPC",
            Machine::PPC64 => "PowerPC64",
            Machine::S390 => "IBM S/390",
            Machine::V800 => "Renesas V850 (using RH850 ABI)",
            Machine::FR20 => "Fujitsu FR20",
            Machine::RH32 => "TRW RH32",
            Machine::RCE => "Motorola M*Core",
            Machine::ARM => "ARM",
            Machine::Alpha => "Digital Alpha (old)",
            Machine::SH => "Renesas / SuperH SH",
            Machine::SPARCV9 => "Sparc v9",
            Machine::Tricore => "Siemens Tricore",
            Machine::ARC => "ARC",
            Machine::H8300 => "Renesas H8/300",
            Machine::H8_300H => "Renesas H8/300H",
            Machine::H8s => "Renesas H8S",
            Machine::H8500 => "Renesas H8/500",
            Machine::IA64 => "Intel IA-64",
            Machine::MIPSX => "Stanford MIPS-X",
            Machine::Coldfire => "Motorola Coldfire",
            Machine::M68HC12 => "Motorola MC68HC12 Microcontroller",
            Machine::MMA => "Fujitsu Multimedia Accellerator",
            Machine::PCP => "Siemens PCP",
            Machine::NCPU => "Sony nCPU embedded RISC processor",
            Machine::NDR1 => "Denso NDR1 microprocessor",
            Machine::Starcore => "Motorola Star*Core processor",
            Machine::ME16 => "Toyota ME16 processor",
            Machine::ST100 => "STMicroelectronics ST100 processor",
            Machine::TinyJ => "Advanced Logic Corp. TinyJ embedded processor",
            Machine::X86_64 => "Advanced Micro Devices X86-64",
            Machine::S320C600 => "Texas Instruments TMS320C6000 DSP family",
            Machine::AARCH64 => "AArch64",
            Machine::RISCV => "RISC-V",
            Machine::BPF => "Berkeley Packet Filter",
            Machine::MCS6502 => "MOS Technology MCS 6502 processor",
            Machine::WDC65C817 => "WDC 65816/65C816",
        };

        write!(f, "{}", repr)
    }
}

/// MachineParser implements a machine parser for parsing a 2-byte machine for
/// each of the given data encodings.
pub struct MachineParser<E>
where
    E: DataEncoding,
{
    endianness: std::marker::PhantomData<E>,
}

impl<E> MachineParser<E>
where
    E: DataEncoding,
{
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

/// Version represent an ELF version. This should always be one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Version {
    One = 0x01,
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u32)
    }
}

impl From<Version> for u32 {
    fn from(src: Version) -> Self {
        src as u32
    }
}

/// VersionParser attempts to parse a single byte representing the version of
/// ELF for any given endianness.
pub struct VersionParser<E>
where
    E: DataEncoding,
{
    endianness: std::marker::PhantomData<E>,
}

impl<E> VersionParser<E>
where
    E: DataEncoding,
{
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

/// EiIdent defines the elf identification fields that define whether the
/// address size, versions and abi of the file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EiIdent {
    pub ei_class: EiClass,
    pub ei_data: EiData,
    pub ei_version: EiVersion,
    pub ei_osabi: EiOsAbi,
    pub ei_abiversion: EiAbiVersion,
}

/// EiIdentParser defines a parser for parsing a raw bitstream into an EiIdent.
pub struct EiIdentParser;

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
                        parcel::join(EiOsAbiParser, EiAbiVersionParser),
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

/// FileHeader represents a program file header, and contains ELF identifaction
/// information along with sizing, architechture and additional metadata about
/// other ELF headers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileHeader<AddrWidth> {
    pub r#type: Type,
    pub machine: Machine,
    pub version: Version,
    pub entry_point: AddrWidth,
    pub ph_offset: AddrWidth,
    pub sh_offset: AddrWidth,
    pub flags: u32,
    pub eh_size: u16,
    pub phent_size: u16,
    pub phnum: u16,
    pub shent_size: u16,
    pub shnum: u16,
    pub shstrndx: u16,
}

/// FileHeaderParser defines a parser for parsing a raw bitstream into a FileHeader.
pub struct FileHeaderParser<A, E> {
    address_width: std::marker::PhantomData<A>,
    endianness: std::marker::PhantomData<E>,
}

impl<E> FileHeaderParser<u32, E> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            address_width: std::marker::PhantomData,
            endianness: std::marker::PhantomData,
        }
    }
}

impl<E> FileHeaderParser<u64, E> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<A, E> Default for FileHeaderParser<A, E> {
    fn default() -> Self {
        Self {
            address_width: std::marker::PhantomData,
            endianness: std::marker::PhantomData,
        }
    }
}

impl<'a, E> parcel::Parser<'a, &'a [u8], FileHeader<Elf32Addr>> for FileHeaderParser<Elf32Addr, E>
where
    EiData: From<E>,
    E: DataEncoding + Default + 'static,
    TypeParser<E>: Parser<'a, &'a [u8], Type>,
    MachineParser<E>: Parser<'a, &'a [u8], Machine>,
    VersionParser<E>: Parser<'a, &'a [u8], Version>,
{
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], FileHeader<Elf32Addr>> {
        let encoding = EiData::from(E::default());

        parcel::join(
            TypeParser::<E>::new(),
            parcel::join(
                MachineParser::<E>::new(),
                parcel::join(
                    VersionParser::<E>::new(),
                    parcel::join(
                        match_u32(encoding),
                        parcel::join(
                            match_u32(encoding),
                            parcel::join(
                                match_u32(encoding),
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
                (
                    r#type,
                    machine,
                    version,
                    entry_point,
                    ph_offset,
                    sh_offset,
                    flags,
                    two_byte_fields[0],
                    two_byte_fields[1],
                    two_byte_fields[2],
                    two_byte_fields[3],
                    two_byte_fields[4],
                    two_byte_fields[5],
                )
            },
        )
        .map(
            move |(
                r#type,
                machine,
                version,
                entry_point,
                ph_offset,
                sh_offset,
                flags,
                eh_size,
                phent_size,
                phnum,
                shent_size,
                shnum,
                shstrndx,
            )| {
                FileHeader {
                    r#type,
                    machine,
                    version,
                    entry_point,
                    ph_offset,
                    sh_offset,
                    flags,
                    eh_size,
                    phent_size,
                    phnum,
                    shent_size,
                    shnum,
                    shstrndx,
                }
            },
        )
        .parse(&input[16..])
    }
}

impl<'a, E> parcel::Parser<'a, &'a [u8], FileHeader<Elf64Addr>> for FileHeaderParser<Elf64Addr, E>
where
    EiData: From<E>,
    E: DataEncoding + Default + 'static,
    TypeParser<E>: Parser<'a, &'a [u8], Type>,
    MachineParser<E>: Parser<'a, &'a [u8], Machine>,
    VersionParser<E>: Parser<'a, &'a [u8], Version>,
{
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], FileHeader<Elf64Addr>> {
        let encoding = EiData::from(E::default());

        parcel::join(
            TypeParser::<E>::new(),
            parcel::join(
                MachineParser::<E>::new(),
                parcel::join(
                    VersionParser::<E>::new(),
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
                (
                    r#type,
                    machine,
                    version,
                    entry_point,
                    ph_offset,
                    sh_offset,
                    flags,
                    two_byte_fields[0],
                    two_byte_fields[1],
                    two_byte_fields[2],
                    two_byte_fields[3],
                    two_byte_fields[4],
                    two_byte_fields[5],
                )
            },
        )
        .map(
            move |(
                r#type,
                machine,
                version,
                entry_point,
                ph_offset,
                sh_offset,
                flags,
                eh_size,
                phent_size,
                phnum,
                shent_size,
                shnum,
                shstrndx,
            )| {
                FileHeader {
                    r#type,
                    machine,
                    version,
                    entry_point,
                    ph_offset,
                    sh_offset,
                    flags,
                    eh_size,
                    phent_size,
                    phnum,
                    shent_size,
                    shnum,
                    shstrndx,
                }
            },
        )
        .parse(&input[16..])
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum ProgramHeaderType {
    Null = 0x00,
    Load = 0x01,
    Dynamic = 0x02,
    Interp = 0x03,
    Note = 0x04,
    ShLib = 0x05,
    PhDr = 0x06,
    Tls = 0x07,
    LoOs = 0x60000000,
    HiOs = 0x6FFFFFFF,
    LoProc = 0x70000000,
    HiProc = 0x7FFFFFFF,
}

pub struct ProgramHeaderTypeParser<E>
where
    E: DataEncoding,
{
    endianness: std::marker::PhantomData<E>,
}

impl<'a, E> ProgramHeaderTypeParser<E>
where
    E: DataEncoding,
{
    fn new() -> Self {
        Self {
            endianness: std::marker::PhantomData,
        }
    }

    fn parse_type(
        &self,
        data: EiData,
        input: &'a [u8],
    ) -> parcel::ParseResult<'a, &'a [u8], ProgramHeaderType> {
        parcel::one_of(vec![
            expect_u32(data, ProgramHeaderType::Null as u32).map(|_| ProgramHeaderType::Null),
            expect_u32(data, ProgramHeaderType::Load as u32).map(|_| ProgramHeaderType::Load),
            expect_u32(data, ProgramHeaderType::Dynamic as u32).map(|_| ProgramHeaderType::Dynamic),
            expect_u32(data, ProgramHeaderType::Interp as u32).map(|_| ProgramHeaderType::Interp),
            expect_u32(data, ProgramHeaderType::Note as u32).map(|_| ProgramHeaderType::Note),
            expect_u32(data, ProgramHeaderType::ShLib as u32).map(|_| ProgramHeaderType::ShLib),
            expect_u32(data, ProgramHeaderType::PhDr as u32).map(|_| ProgramHeaderType::PhDr),
            expect_u32(data, ProgramHeaderType::Tls as u32).map(|_| ProgramHeaderType::Tls),
            expect_u32(data, ProgramHeaderType::LoOs as u32).map(|_| ProgramHeaderType::LoOs),
            expect_u32(data, ProgramHeaderType::HiOs as u32).map(|_| ProgramHeaderType::HiOs),
            expect_u32(data, ProgramHeaderType::LoProc as u32).map(|_| ProgramHeaderType::LoProc),
            expect_u32(data, ProgramHeaderType::HiProc as u32).map(|_| ProgramHeaderType::HiProc),
        ])
        .parse(input)
    }
}

impl<'a> parcel::Parser<'a, &'a [u8], ProgramHeaderType>
    for ProgramHeaderTypeParser<LittleEndianDataEncoding>
{
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], ProgramHeaderType> {
        self.parse_type(EiData::Little, input)
    }
}

impl<'a> parcel::Parser<'a, &'a [u8], ProgramHeaderType>
    for ProgramHeaderTypeParser<BigEndianDataEncoding>
{
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], ProgramHeaderType> {
        self.parse_type(EiData::Big, input)
    }
}

/// Represents a Program
pub trait ProgramHeader {}

/// Program header represents a Elf Program header for the 32-bit arrangement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProgramHeader32Bit {
    r#type: ProgramHeaderType,
    offset: u32,
    vaddr: u32,
    paddr: u32,
    filesz: u32,
    memsz: u32,
    flags: u32,
    align: u32,
}

impl ProgramHeader for ProgramHeader32Bit {}

/// ProgramHeaderParser takes an address width and a data encoding that
/// represents endianness and implements various parsers for each valid variant.
pub struct ProgramHeaderParser<A, E>
where
    A: AddressWidth,
    E: DataEncoding,
{
    address_width: std::marker::PhantomData<A>,
    endianness: std::marker::PhantomData<E>,
}

impl<A, E> ProgramHeaderParser<A, E>
where
    A: AddressWidth,
    E: DataEncoding,
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl<A, E> Default for ProgramHeaderParser<A, E>
where
    A: AddressWidth,
    E: DataEncoding,
{
    fn default() -> Self {
        Self {
            address_width: std::marker::PhantomData,
            endianness: std::marker::PhantomData,
        }
    }
}

impl<'a, E> parcel::Parser<'a, &'a [u8], ProgramHeader32Bit> for ProgramHeaderParser<Elf32Addr, E>
where
    EiData: From<E>,
    E: DataEncoding + Default + 'static,
    ProgramHeaderTypeParser<E>: Parser<'a, &'a [u8], ProgramHeaderType>,
{
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], ProgramHeader32Bit> {
        let encoding = EiData::from(E::default());
        parcel::join(
            ProgramHeaderTypeParser::<E>::new(),
            parcel::take_n(match_u32(encoding), 7),
        )
        .map(|(r#type, four_byte_fields)| {
            (
                r#type,
                four_byte_fields[0],
                four_byte_fields[1],
                four_byte_fields[2],
                four_byte_fields[3],
                four_byte_fields[4],
                four_byte_fields[5],
                four_byte_fields[6],
            )
        })
        .map(
            |(r#type, offset, vaddr, paddr, filesz, memsz, flags, align)| ProgramHeader32Bit {
                r#type,
                offset,
                vaddr,
                paddr,
                filesz,
                memsz,
                flags,
                align,
            },
        )
        .parse(&input)
    }
}

impl<'a, E> parcel::Parser<'a, &'a [u8], ProgramHeader64Bit> for ProgramHeaderParser<Elf64Addr, E>
where
    EiData: From<E>,
    E: DataEncoding + Default + 'static,
    ProgramHeaderTypeParser<E>: Parser<'a, &'a [u8], ProgramHeaderType>,
{
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], ProgramHeader64Bit> {
        let encoding = EiData::from(E::default());
        parcel::join(
            ProgramHeaderTypeParser::<E>::new(),
            parcel::join(match_u32(encoding), parcel::take_n(match_u64(encoding), 6)),
        )
        .map(|(r#type, (flags, eight_byte_fields))| {
            (
                r#type,
                flags,
                eight_byte_fields[0],
                eight_byte_fields[1],
                eight_byte_fields[2],
                eight_byte_fields[3],
                eight_byte_fields[4],
                eight_byte_fields[5],
            )
        })
        .map(
            |(r#type, flags, offset, vaddr, paddr, filesz, memsz, align)| ProgramHeader64Bit {
                r#type,
                flags,
                offset,
                vaddr,
                paddr,
                filesz,
                memsz,
                align,
            },
        )
        .parse(&input)
    }
}

/// Program header represents a Elf Program header for the 64-bit arrangement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProgramHeader64Bit {
    r#type: ProgramHeaderType,
    flags: u32,
    offset: u64,
    vaddr: u64,
    paddr: u64,
    filesz: u64,
    memsz: u64,
    align: u64,
}

impl ProgramHeader for ProgramHeader64Bit {}

/// ShType reprents all representable formats of the sh_type filed of a section
/// header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ShType {
    Null = 0x00,
}

/// ShFlags reprents all representable formats of the sh_flags filed of a
/// section header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ShFlags {
    Write = 0x01,
}

/// Section header represents a Elf Program header for the 32-bit arrangement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SectionHeader<A>
where
    A: AddressWidth,
{
    pub sh_name: A,
    pub sh_type: ShType,
    pub sh_flags: ShFlags,
    pub sh_addr: A,
    pub sh_offset: A,
    pub sh_size: A,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addr_align: A,
    pub sh_entsize: A,
}

/// ElfHeader captures the full ELF file header into a single struct along
/// with the Identification information separated from the file header.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElfHeader<E, PH>
where
    E: DataEncoding,
{
    pub ei_ident: EiIdent,
    pub file_header: FileHeader<E>,
    pub program_header: PH,
}

impl<E, PH> ElfHeader<E, PH>
where
    E: DataEncoding,
    PH: ProgramHeader,
{
    pub fn new(ei_ident: EiIdent, file_header: FileHeader<E>, program_header: PH) -> Self {
        Self {
            ei_ident,
            file_header,
            program_header,
        }
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

    macro_rules! generate_file_header {
        () => {
            vec![
                // File Header
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
            ]
        };
    }

    macro_rules! generate_program_header {
        () => {
            vec![
                // Program Header
                0x00, 0x00, 0x00, 0x00, // p_type
                0x00, 0x00, 0x00, 0x00, // p_offset
                0x00, 0x00, 0x00, 0x00, // p_vaddr
                0x00, 0x00, 0x00, 0x00, // p_paddr
                0x00, 0x00, 0x00, 0x00, // p_filesz
                0x00, 0x00, 0x00, 0x00, // p_memsz
                0x00, 0x00, 0x00, 0x00, // p_flags
                0x00, 0x00, 0x00, 0x00, // p_align
            ]
        };
    }

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
            EiIdentParser
                .parse(&thirty_two_bit_input)
                .map(|ms| ms.unwrap().ei_class)
        );
        assert_eq!(
            Ok(EiClass::SixtyFourBit),
            EiIdentParser
                .parse(&sixty_four_bit_input)
                .map(|ms| ms.unwrap().ei_class)
        );
        assert!(EiIdentParser
            .parse(&invalid_input)
            .and_then(|ms| match ms {
                MatchStatus::Match(_) => Err("invalid input shouldn't match".to_string()),
                MatchStatus::NoMatch(_) => Ok(()),
            })
            .is_ok());
    }

    #[test]
    fn parse_known_good_file_header() {
        let input: Vec<u8> = generate_file_header!();

        assert_eq!(
            FileHeader::<Elf32Addr> {
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
            },
            FileHeaderParser::<Elf32Addr, LittleEndianDataEncoding>::new()
                .parse(&input)
                .unwrap()
                .unwrap(),
        )
    }

    #[test]
    fn parse_known_good_program_header() {
        let input: Vec<u8> = generate_program_header!();

        assert_eq!(
            ProgramHeader32Bit {
                r#type: ProgramHeaderType::Null,
                offset: 0x00,
                vaddr: 0x00,
                paddr: 0x00,
                filesz: 0x00,
                memsz: 0x00,
                flags: 0x00,
                align: 0x00,
            },
            ProgramHeaderParser::<Elf32Addr, LittleEndianDataEncoding>::new()
                .parse(&input)
                .unwrap()
                .unwrap()
        )
    }
}
