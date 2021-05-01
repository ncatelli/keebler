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

/// Serialize defines a trait for serializing a type to a corresponding binary format.
pub trait Serialize<A, E> {
    fn serialize(&self) -> Vec<u8>;
}

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

/// EiData stores an 8-bit value representing if the header is in little-endian
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

impl From<EiIdent> for Vec<u8> {
    fn from(src: EiIdent) -> Self {
        vec![
            src.ei_class as u8,
            src.ei_class as u8,
            src.ei_version as u8,
            src.ei_osabi as u8,
            src.ei_abiversion as u8,
        ]
    }
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
pub struct FileHeader<A> {
    pub r#type: Type,
    pub machine: Machine,
    pub version: Version,
    pub entry_point: A,
    pub ph_offset: A,
    pub sh_offset: A,
    pub flags: u32,
    pub eh_size: u16,
    pub phent_size: u16,
    pub phnum: u16,
    pub shent_size: u16,
    pub shnum: u16,
    pub shstrndx: u16,
}

impl Serialize<Elf32Addr, LittleEndianDataEncoding> for FileHeader<Elf32Addr> {
    fn serialize(&self) -> Vec<u8> {
        vec![
            Into::<u16>::into(self.r#type).to_le_bytes().to_vec(),
            Into::<u16>::into(self.machine).to_le_bytes().to_vec(),
            Into::<u32>::into(self.version).to_le_bytes().to_vec(),
            self.entry_point.to_le_bytes().to_vec(),
            self.ph_offset.to_le_bytes().to_vec(),
            self.sh_offset.to_le_bytes().to_vec(),
            self.flags.to_le_bytes().to_vec(),
            self.eh_size.to_le_bytes().to_vec(),
            self.phent_size.to_le_bytes().to_vec(),
            self.phnum.to_le_bytes().to_vec(),
            self.shent_size.to_le_bytes().to_vec(),
            self.shnum.to_le_bytes().to_vec(),
            self.shstrndx.to_le_bytes().to_vec(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl Serialize<Elf32Addr, BigEndianDataEncoding> for FileHeader<Elf32Addr> {
    fn serialize(&self) -> Vec<u8> {
        vec![
            Into::<u16>::into(self.r#type).to_be_bytes().to_vec(),
            Into::<u16>::into(self.machine).to_be_bytes().to_vec(),
            Into::<u32>::into(self.version).to_be_bytes().to_vec(),
            self.entry_point.to_be_bytes().to_vec(),
            self.ph_offset.to_be_bytes().to_vec(),
            self.sh_offset.to_be_bytes().to_vec(),
            self.flags.to_be_bytes().to_vec(),
            self.eh_size.to_be_bytes().to_vec(),
            self.phent_size.to_be_bytes().to_vec(),
            self.phnum.to_be_bytes().to_vec(),
            self.shent_size.to_be_bytes().to_vec(),
            self.shnum.to_be_bytes().to_vec(),
            self.shstrndx.to_be_bytes().to_vec(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl Serialize<Elf64Addr, LittleEndianDataEncoding> for FileHeader<Elf64Addr> {
    fn serialize(&self) -> Vec<u8> {
        vec![
            Into::<u16>::into(self.r#type).to_le_bytes().to_vec(),
            Into::<u16>::into(self.machine).to_le_bytes().to_vec(),
            Into::<u32>::into(self.version).to_le_bytes().to_vec(),
            self.entry_point.to_le_bytes().to_vec(),
            self.ph_offset.to_le_bytes().to_vec(),
            self.sh_offset.to_le_bytes().to_vec(),
            self.flags.to_le_bytes().to_vec(),
            self.eh_size.to_le_bytes().to_vec(),
            self.phent_size.to_le_bytes().to_vec(),
            self.phnum.to_le_bytes().to_vec(),
            self.shent_size.to_le_bytes().to_vec(),
            self.shnum.to_le_bytes().to_vec(),
            self.shstrndx.to_le_bytes().to_vec(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl Serialize<Elf64Addr, BigEndianDataEncoding> for FileHeader<Elf64Addr> {
    fn serialize(&self) -> Vec<u8> {
        vec![
            Into::<u16>::into(self.r#type).to_be_bytes().to_vec(),
            Into::<u16>::into(self.machine).to_be_bytes().to_vec(),
            Into::<u32>::into(self.version).to_be_bytes().to_vec(),
            self.entry_point.to_be_bytes().to_vec(),
            self.ph_offset.to_be_bytes().to_vec(),
            self.sh_offset.to_be_bytes().to_vec(),
            self.flags.to_be_bytes().to_vec(),
            self.eh_size.to_be_bytes().to_vec(),
            self.phent_size.to_be_bytes().to_vec(),
            self.phnum.to_be_bytes().to_vec(),
            self.shent_size.to_be_bytes().to_vec(),
            self.shnum.to_be_bytes().to_vec(),
            self.shstrndx.to_be_bytes().to_vec(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

/// FileHeaderParser defines a parser for parsing a raw bitstream into a FileHeader.
pub struct FileHeaderParser<A, E>
where
    A: AddressWidth,
    E: DataEncoding,
{
    address_width: std::marker::PhantomData<A>,
    endianness: std::marker::PhantomData<E>,
}

impl<E> FileHeaderParser<Elf32Addr, E>
where
    E: DataEncoding,
{
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            address_width: std::marker::PhantomData,
            endianness: std::marker::PhantomData,
        }
    }
}

impl<E> FileHeaderParser<u64, E>
where
    E: DataEncoding,
{
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<A, E> Default for FileHeaderParser<A, E>
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

/// ProgramHeaderType represents each type of program header.
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
    GnuEhFrame = 0x6474E550,
    GnuStack = 0x6474E551,
    GnuRelro = 0x6474E552,
}

impl std::fmt::Display for ProgramHeaderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            ProgramHeaderType::Null => "Null",
            ProgramHeaderType::Load => "Load",
            ProgramHeaderType::Dynamic => "Dynamic",
            ProgramHeaderType::Interp => "Interp",
            ProgramHeaderType::Note => "Note",
            ProgramHeaderType::ShLib => "SH_LIB",
            ProgramHeaderType::PhDr => "PH_DR",
            ProgramHeaderType::Tls => "TLS",
            ProgramHeaderType::LoOs => "LO_OS",
            ProgramHeaderType::HiOs => "HI_OS",
            ProgramHeaderType::LoProc => "LO_PROC",
            ProgramHeaderType::HiProc => "HI_PROC",
            ProgramHeaderType::GnuEhFrame => "GNU_EH_FRAME",
            ProgramHeaderType::GnuStack => "GNU_STACK",
            ProgramHeaderType::GnuRelro => "GNU_RELRO",
        };

        write!(f, "{}", repr)
    }
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
            expect_u32(data, ProgramHeaderType::GnuEhFrame as u32)
                .map(|_| ProgramHeaderType::GnuEhFrame),
            expect_u32(data, ProgramHeaderType::GnuStack as u32)
                .map(|_| ProgramHeaderType::GnuStack),
            expect_u32(data, ProgramHeaderType::GnuRelro as u32)
                .map(|_| ProgramHeaderType::GnuRelro),
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

/// Represents any kind of ProgramHeader, functioning as a way to link the 32
/// and 64-bit ProgramHeader types.
pub trait ProgramHeader {}

/// Program header represents a Elf Program header for the 32-bit arrangement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProgramHeader32Bit {
    pub r#type: ProgramHeaderType,
    pub offset: u32,
    pub vaddr: u32,
    pub paddr: u32,
    pub filesz: u32,
    pub memsz: u32,
    pub flags: u32,
    pub align: u32,
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
    pub r#type: ProgramHeaderType,
    pub flags: u32,
    pub offset: u64,
    pub vaddr: u64,
    pub paddr: u64,
    pub filesz: u64,
    pub memsz: u64,
    pub align: u64,
}

impl ProgramHeader for ProgramHeader64Bit {}

/// ShType reprents all representable formats of the sh_type filed of a section
/// header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum ShType {
    Null = 0x00,
    ProgBits = 0x01,
    SymTab = 0x02,
    StrTab = 0x03,
    Rela = 0x04,
    Hash = 0x05,
    Dynamic = 0x06,
    Note = 0x07,
    NoBits = 0x08,
    Rel = 0x09,
    ShLib = 0x0a,
    DynSym = 0x0b,
    InitArray = 0x0e,
    FiniArray = 0x0f,
    PreInitArray = 0x10,
    Group = 0x11,
    SymTabShndx = 0x12,
    Num = 0x13,
}

impl std::fmt::Display for ShType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            ShType::Null => "NULL",
            ShType::ProgBits => "PROG_Bits",
            ShType::SymTab => "SYM_TAB",
            ShType::StrTab => "STR_TAB",
            ShType::Rela => "RELA",
            ShType::Hash => "HASH",
            ShType::Dynamic => "DYNAMIC",
            ShType::Note => "NOTE",
            ShType::NoBits => "NO_BITS",
            ShType::Rel => "REL",
            ShType::ShLib => "SH_LIB",
            ShType::DynSym => "DYN_SYM",
            ShType::InitArray => "INIT_ARRAY",
            ShType::FiniArray => "FINI_ARRAY",
            ShType::PreInitArray => "PRE_INIT_ARRAY",
            ShType::Group => "GROUP",
            ShType::SymTabShndx => "SYM_TAB_SHNDX",
            ShType::Num => "NUM",
        };

        write!(f, "{}", repr)
    }
}

/// Provides a byte parser for a ShType from a given endian source.
pub struct ShTypeParser<E>
where
    E: DataEncoding,
{
    endianness: std::marker::PhantomData<E>,
}

impl<E> ShTypeParser<E>
where
    E: DataEncoding,
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl<E> Default for ShTypeParser<E>
where
    E: DataEncoding,
{
    fn default() -> Self {
        Self {
            endianness: std::marker::PhantomData,
        }
    }
}

impl<'a, E> parcel::Parser<'a, &'a [u8], ShType> for ShTypeParser<E>
where
    EiData: From<E>,
    E: DataEncoding + Default + 'static,
{
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], ShType> {
        let encoding = EiData::from(E::default());

        parcel::one_of(vec![
            expect_u32(encoding, ShType::Null as u32).map(|_| ShType::Null),
            expect_u32(encoding, ShType::ProgBits as u32).map(|_| ShType::ProgBits),
            expect_u32(encoding, ShType::SymTab as u32).map(|_| ShType::SymTab),
            expect_u32(encoding, ShType::StrTab as u32).map(|_| ShType::StrTab),
            expect_u32(encoding, ShType::Rela as u32).map(|_| ShType::Rela),
            expect_u32(encoding, ShType::Hash as u32).map(|_| ShType::Hash),
            expect_u32(encoding, ShType::Dynamic as u32).map(|_| ShType::Dynamic),
            expect_u32(encoding, ShType::Note as u32).map(|_| ShType::Note),
            expect_u32(encoding, ShType::NoBits as u32).map(|_| ShType::NoBits),
            expect_u32(encoding, ShType::Rel as u32).map(|_| ShType::Rel),
            expect_u32(encoding, ShType::ShLib as u32).map(|_| ShType::ShLib),
            expect_u32(encoding, ShType::DynSym as u32).map(|_| ShType::DynSym),
            expect_u32(encoding, ShType::InitArray as u32).map(|_| ShType::InitArray),
            expect_u32(encoding, ShType::FiniArray as u32).map(|_| ShType::FiniArray),
            expect_u32(encoding, ShType::PreInitArray as u32).map(|_| ShType::PreInitArray),
            expect_u32(encoding, ShType::Group as u32).map(|_| ShType::Group),
            expect_u32(encoding, ShType::SymTabShndx as u32).map(|_| ShType::SymTabShndx),
            expect_u32(encoding, ShType::Num as u32).map(|_| ShType::Num),
        ])
        .or(move || match_u32(encoding).map(|_| ShType::Null))
        .parse(input)
    }
}

/// ShFlags32Bit reprents all representable formats of the sh_flags filed of a
/// section header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ShFlags32Bit {
    Write = 0x01,
    Other = 0x9999,
}

/// ShFlags64Bit reprents all representable formats of the sh_flags filed of a
/// section header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum ShFlags64Bit {
    Write = 0x01,
    Other = 0x9999,
}

/// Provides a parser for ShFlags for a given address width and endianness.
pub struct ShFlagsParser<A, E>
where
    A: AddressWidth,
    E: DataEncoding,
{
    address_width: std::marker::PhantomData<A>,
    endianness: std::marker::PhantomData<E>,
}

impl<A, E> ShFlagsParser<A, E>
where
    A: AddressWidth,
    E: DataEncoding,
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl<A, E> Default for ShFlagsParser<A, E>
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

impl<'a, E> parcel::Parser<'a, &'a [u8], ShFlags32Bit> for ShFlagsParser<Elf32Addr, E>
where
    EiData: From<E>,
    E: DataEncoding + Default + 'static,
{
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], ShFlags32Bit> {
        let encoding = EiData::from(E::default());

        parcel::one_of(vec![
            expect_u32(encoding, ShFlags32Bit::Write as u32).map(|_| ShFlags32Bit::Write)
        ])
        .or(move || match_u32(encoding).map(|_| ShFlags32Bit::Other))
        .parse(input)
    }
}

impl<'a, E> parcel::Parser<'a, &'a [u8], ShFlags64Bit> for ShFlagsParser<Elf64Addr, E>
where
    EiData: From<E>,
    E: DataEncoding + Default + 'static,
{
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], ShFlags64Bit> {
        let encoding = EiData::from(E::default());

        parcel::one_of(vec![
            expect_u64(encoding, ShFlags64Bit::Write as u64).map(|_| ShFlags64Bit::Write)
        ])
        .or(move || match_u64(encoding).map(|_| ShFlags64Bit::Other))
        .parse(input)
    }
}

/// Provides a trait for identifying Section headers. Functionally this works
/// to link the 32-bit and 64-bit SectionHeader types.
pub trait SectionHeader {}

/// Section header represents a Elf Program header.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SectionHeader32Bit {
    pub sh_name: u32,
    pub sh_type: ShType,
    pub sh_flags: ShFlags32Bit,
    pub sh_addr: u32,
    pub sh_offset: u32,
    pub sh_size: u32,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addr_align: u32,
    pub sh_entsize: u32,
}

impl SectionHeader for SectionHeader32Bit {}

/// Section header represents a Elf Program header.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SectionHeader64Bit {
    pub sh_name: u32,
    pub sh_type: ShType,
    pub sh_flags: ShFlags64Bit,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addr_align: u64,
    pub sh_entsize: u64,
}

impl SectionHeader for SectionHeader64Bit {}

/// Implements a parser for SectionHeaders of a given endianness and address width.
pub struct SectionHeaderParser<A, E>
where
    A: AddressWidth,
    E: DataEncoding,
{
    address_width: std::marker::PhantomData<A>,
    endianness: std::marker::PhantomData<E>,
}

impl<A, E> SectionHeaderParser<A, E>
where
    A: AddressWidth,
    E: DataEncoding,
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl<A, E> Default for SectionHeaderParser<A, E>
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

impl<'a, E> parcel::Parser<'a, &'a [u8], SectionHeader32Bit> for SectionHeaderParser<Elf32Addr, E>
where
    EiData: From<E>,
    E: DataEncoding + Default + 'static,
{
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], SectionHeader32Bit> {
        let encoding = EiData::from(E::default());

        parcel::join(
            match_u32(encoding),
            parcel::join(
                ShTypeParser::<E>::new(),
                parcel::join(
                    ShFlagsParser::<Elf32Addr, E>::new(),
                    parcel::take_n(match_u32(encoding), 7),
                ),
            ),
        )
        .map(|(sh_name, (sh_type, (sh_flags, u32_seq)))| {
            (
                sh_name, sh_type, sh_flags, u32_seq[0], u32_seq[1], u32_seq[2], u32_seq[3],
                u32_seq[4], u32_seq[5], u32_seq[6],
            )
        })
        .map(
            |(
                sh_name,
                sh_type,
                sh_flags,
                sh_addr,
                sh_offset,
                sh_size,
                sh_link,
                sh_info,
                sh_addr_align,
                sh_entsize,
            )| SectionHeader32Bit {
                sh_name,
                sh_type,
                sh_flags,
                sh_addr,
                sh_offset,
                sh_size,
                sh_link,
                sh_info,
                sh_addr_align,
                sh_entsize,
            },
        )
        .parse(input)
    }
}

impl<'a, E> parcel::Parser<'a, &'a [u8], SectionHeader64Bit> for SectionHeaderParser<Elf64Addr, E>
where
    EiData: From<E>,
    E: DataEncoding + Default + 'static,
{
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], SectionHeader64Bit> {
        let encoding = EiData::from(E::default());

        parcel::join(
            match_u32(encoding),
            parcel::join(
                ShTypeParser::<E>::new(),
                parcel::join(
                    ShFlagsParser::<Elf64Addr, E>::new(),
                    parcel::join(
                        parcel::take_n(match_u64(encoding), 3),
                        parcel::join(
                            parcel::take_n(match_u32(encoding), 2),
                            parcel::take_n(match_u64(encoding), 2),
                        ),
                    ),
                ),
            ),
        )
        .map(
            |(sh_name, (sh_type, (sh_flags, (u64_seq_one, (u32_seq_one, u64_seq_two)))))| {
                (
                    sh_name,
                    sh_type,
                    sh_flags,
                    u64_seq_one[0],
                    u64_seq_one[1],
                    u64_seq_one[2],
                    u32_seq_one[0],
                    u32_seq_one[1],
                    u64_seq_two[0],
                    u64_seq_two[1],
                )
            },
        )
        .map(
            |(
                sh_name,
                sh_type,
                sh_flags,
                sh_addr,
                sh_offset,
                sh_size,
                sh_link,
                sh_info,
                sh_addr_align,
                sh_entsize,
            )| SectionHeader64Bit {
                sh_name,
                sh_type,
                sh_flags,
                sh_addr,
                sh_offset,
                sh_size,
                sh_link,
                sh_info,
                sh_addr_align,
                sh_entsize,
            },
        )
        .parse(input)
    }
}

/// ElfHeader represents an ELF Header and functions to link the 32-bit and
/// 64-bit ElfHeader types.
pub trait ElfHeader {}

/// ElfHeader32Bit captures the full ELF file header into a single struct along
/// with the Identification information separated from the file header.
#[derive(Debug, Clone, PartialEq)]
pub struct ElfHeader32Bit<E>
where
    E: DataEncoding + Default + 'static,
{
    endianness: std::marker::PhantomData<E>,
    pub ei_ident: EiIdent,
    pub file_header: FileHeader<Elf32Addr>,
    pub program_headers: Vec<ProgramHeader32Bit>,
    pub section_headers: Vec<SectionHeader32Bit>,
}

impl<E> ElfHeader32Bit<E>
where
    E: DataEncoding + Default + 'static,
{
    pub fn new(
        ei_ident: EiIdent,
        file_header: FileHeader<Elf32Addr>,
        program_headers: Vec<ProgramHeader32Bit>,
        section_headers: Vec<SectionHeader32Bit>,
    ) -> Self {
        Self {
            endianness: std::marker::PhantomData,
            ei_ident,
            file_header,
            program_headers,
            section_headers,
        }
    }
}

impl ElfHeader for ElfHeader32Bit<LittleEndianDataEncoding> {}

impl<E> From<ElfHeader32Bit<E>> for Vec<u8>
where
    FileHeader<Elf32Addr>: Serialize<Elf32Addr, E>,
    E: DataEncoding + Default + 'static,
{
    fn from(src: ElfHeader32Bit<E>) -> Self {
        let ident_bytes = Into::<Vec<u8>>::into(src.ei_ident);
        let fh_bytes: Vec<u8> = Serialize::<Elf32Addr, E>::serialize(&src.file_header);

        vec![ident_bytes, fh_bytes].into_iter().flatten().collect()
    }
}

/// ElfHeader64Bit captures the full ELF file header into a single struct along
/// with the Identification information separated from the file header.
#[derive(Debug, Clone, PartialEq)]
pub struct ElfHeader64Bit<E>
where
    E: DataEncoding,
{
    endianness: std::marker::PhantomData<E>,
    pub ei_ident: EiIdent,
    pub file_header: FileHeader<Elf64Addr>,
    pub program_headers: Vec<ProgramHeader64Bit>,
    pub section_headers: Vec<SectionHeader64Bit>,
}

impl<E> ElfHeader64Bit<E>
where
    E: DataEncoding,
{
    pub fn new(
        ei_ident: EiIdent,
        file_header: FileHeader<Elf64Addr>,
        program_headers: Vec<ProgramHeader64Bit>,
        section_headers: Vec<SectionHeader64Bit>,
    ) -> Self {
        Self {
            endianness: std::marker::PhantomData,
            ei_ident,
            file_header,
            program_headers,
            section_headers,
        }
    }
}

impl<E: DataEncoding> ElfHeader for ElfHeader64Bit<E> {}

/// ElfHeaderParser implements a parser for ElfHeader types for each variant
/// of address width from a source of a given endianness.
pub struct ElfHeaderParser<A, E>
where
    A: AddressWidth,
    E: DataEncoding,
{
    address_width: std::marker::PhantomData<A>,
    endianness: std::marker::PhantomData<E>,
}

impl<A, E> ElfHeaderParser<A, E>
where
    A: AddressWidth,
    E: DataEncoding,
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl<A, E> Default for ElfHeaderParser<A, E>
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

impl<'a, E> parcel::Parser<'a, &'a [u8], ElfHeader32Bit<E>> for ElfHeaderParser<Elf32Addr, E>
where
    E: DataEncoding + Default + 'static,
    FileHeaderParser<Elf32Addr, E>: Parser<'a, &'a [u8], FileHeader<Elf32Addr>>,
    ProgramHeaderParser<Elf32Addr, E>: Parser<'a, &'a [u8], ProgramHeader32Bit>,
    SectionHeaderParser<Elf32Addr, E>: Parser<'a, &'a [u8], SectionHeader32Bit>,
{
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], ElfHeader32Bit<E>> {
        let preparse_input = &input[0..];
        match EiIdentParser.parse(&input)? {
            MatchStatus::Match((_, ei)) => FileHeaderParser::<Elf32Addr, E>::new()
                .and_then(|fh| {
                    let phnum = fh.phnum as usize;
                    ProgramHeaderParser::<Elf32Addr, E>::new()
                        .take_n(phnum)
                        .map(move |phs| (fh, phs))
                })
                .and_then(|(fh, phs)| {
                    let shnum = fh.shnum as usize;
                    SectionHeaderParser::<Elf32Addr, E>::new()
                        .take_n(shnum)
                        .map(move |shs| (fh, phs.to_owned(), shs))
                })
                .map(move |(fh, phs, shs)| ElfHeader32Bit::new(ei, fh, phs, shs))
                .parse(&preparse_input),
            MatchStatus::NoMatch(rem) => Ok(MatchStatus::NoMatch(rem)),
        }
    }
}

impl<'a, E> parcel::Parser<'a, &'a [u8], ElfHeader64Bit<E>> for ElfHeaderParser<Elf64Addr, E>
where
    E: DataEncoding + Default + 'static,
    FileHeaderParser<Elf64Addr, E>: Parser<'a, &'a [u8], FileHeader<Elf64Addr>>,
    ProgramHeaderParser<Elf64Addr, E>: Parser<'a, &'a [u8], ProgramHeader64Bit>,
    SectionHeaderParser<Elf64Addr, E>: Parser<'a, &'a [u8], SectionHeader64Bit>,
{
    fn parse(&self, input: &'a [u8]) -> parcel::ParseResult<'a, &'a [u8], ElfHeader64Bit<E>> {
        let preparse_input = &input[0..];
        match EiIdentParser.parse(&input)? {
            MatchStatus::Match((_, ei)) => FileHeaderParser::<Elf64Addr, E>::new()
                .and_then(|fh| {
                    let phnum = fh.phnum as usize;
                    ProgramHeaderParser::<Elf64Addr, E>::new()
                        .take_n(phnum)
                        .map(move |phs| (fh, phs))
                })
                .and_then(|(fh, phs)| {
                    let shnum = fh.shnum as usize;
                    SectionHeaderParser::<Elf64Addr, E>::new()
                        .take_n(shnum)
                        .map(move |shs| (fh, phs.to_owned(), shs))
                })
                .map(move |(fh, phs, shs)| ElfHeader64Bit::new(ei, fh, phs, shs))
                .parse(&preparse_input),
            MatchStatus::NoMatch(rem) => Ok(MatchStatus::NoMatch(rem)),
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

/// Matches a single provided static u64, returning a match if the next
/// eight bytes in the array match the expected u64. Otherwise, a `NoMatch` is
/// returned.
fn expect_u64<'a>(endianness: EiData, expected: u64) -> impl Parser<'a, &'a [u8], u64> {
    move |input: &'a [u8]| {
        let preparse_input = input;
        match match_u64(endianness).parse(input) {
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
