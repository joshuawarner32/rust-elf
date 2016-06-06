use std::fmt;

pub enum Maybe<T, V> {
    Known(T),
    Unknown(V)
}

use Maybe::Known;
use Maybe::Unknown;

impl<T:fmt::Display, V:fmt::Display> fmt::Display for Maybe<T, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &Known(ref val) => write!(f, "{}", val),
            &Unknown(ref val) => write!(f, "Unknown({})", val),
        }
    }
}

#[repr(u8)]
pub enum OsAbi {
    SystemV = 0,
    HpUx = 1,
    NetBsd = 2,
    Linux = 3,
    Solaris = 6,
    Aix = 7,
    Irix = 8,
    FreeBsd = 9,
    Tru64Unix = 10,
    Modesto = 11,
    OpenBsd = 12,
    ArmEabi = 64,
    Arm = 97,
    Standalone = 255,
}

impl OsAbi {
    pub fn from(val: u8) -> Maybe<OsAbi, u8> {
        match val {
            0 => Known(OsAbi::SystemV),
            1 => Known(OsAbi::HpUx),
            2 => Known(OsAbi::NetBsd),
            3 => Known(OsAbi::Linux),
            6 => Known(OsAbi::Solaris),
            7 => Known(OsAbi::Aix),
            8 => Known(OsAbi::Irix),
            9 => Known(OsAbi::FreeBsd),
            10 => Known(OsAbi::Tru64Unix),
            11 => Known(OsAbi::Modesto),
            12 => Known(OsAbi::OpenBsd),
            64 => Known(OsAbi::ArmEabi),
            97 => Known(OsAbi::Arm),
            255 => Known(OsAbi::Standalone),
            _ => Unknown(val)
        }
    }
}

impl fmt::Display for OsAbi {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            OsAbi::SystemV => write!(f, "SystemV"),
            OsAbi::HpUx => write!(f, "HpUx"),
            OsAbi::NetBsd => write!(f, "NetBsd"),
            OsAbi::Linux => write!(f, "Linux"),
            OsAbi::Solaris => write!(f, "Solaris"),
            OsAbi::Aix => write!(f, "Aix"),
            OsAbi::Irix => write!(f, "Irix"),
            OsAbi::FreeBsd => write!(f, "FreeBsd"),
            OsAbi::Tru64Unix => write!(f, "Tru64Unix"),
            OsAbi::Modesto => write!(f, "Modesto"),
            OsAbi::OpenBsd => write!(f, "OpenBsd"),
            OsAbi::ArmEabi => write!(f, "ArmEabi"),
            OsAbi::Arm => write!(f, "Arm"),
            OsAbi::Standalone => write!(f, "Standalone"),
        }
    }
}

#[repr(u8)]
pub enum Bits {
    Bits32 = 1,
    Bits64 = 2,
}

impl Bits {
    pub fn from(val: u8) -> Maybe<Bits, u8> {
        match val {
            1 => Known(Bits::Bits32),
            2 => Known(Bits::Bits64),
            _ => Unknown(val)
        }
    }
}

impl fmt::Display for Bits {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Bits::Bits32 => write!(f, "32-bit"),
            Bits::Bits64 => write!(f, "64-bit"),
        }
    }
}

#[repr(u8)]
pub enum Endian {
    Little = 1,
    Big = 2
}

impl Endian {
    pub fn from(val: u8) -> Maybe<Endian, u8> {
        match val {
            1 => Known(Endian::Little),
            2 => Known(Endian::Big),
            _ => Unknown(val)
        }
    }
}

impl fmt::Display for Endian {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Endian::Little => write!(f, "little-endian"),
            Endian::Big => write!(f, "big-endian"),
        }
    }
}

#[repr(u16)]
pub enum ObjectType {
    NoType = 0,
    Relocatable = 1,
    Executable = 2,
    SharedObject = 3,
    CoreDump = 4,
}

impl ObjectType {
    pub fn from(val: u16) -> Maybe<ObjectType, u16> {
        match val {
            0 => Known(ObjectType::NoType),
            1 => Known(ObjectType::Relocatable),
            2 => Known(ObjectType::Executable),
            3 => Known(ObjectType::SharedObject),
            4 => Known(ObjectType::CoreDump),
            _ => Unknown(val)
        }
    }
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ObjectType::NoType => write!(f, "NoType"),
            ObjectType::Relocatable => write!(f, "Relocatable"),
            ObjectType::Executable => write!(f, "Executable"),
            ObjectType::SharedObject => write!(f, "SharedObject"),
            ObjectType::CoreDump => write!(f, "CoreDump"),
        }
    }
}

#[derive(Debug)]
#[repr(u16)]
pub enum Machine {
    NoMachine = 0,
    AtatWe32100 = 1,
    Sparc = 2,
    I386 = 3,
    Motorola68K = 4,
    Motorola88K = 5,
    Intel80860 = 7,
    Mips = 8,
    IbmSystem370 = 9,
    MipsRs3Le = 10,
    HppaPaRisc = 15,
    FujitsuVpp500 = 17,
    Sparc32Plus = 18,
    Intel80960 = 19,
    PowerPc = 20,
    PowerPc64 = 21,
    IbmSystem390 = 22,
    NecV800 = 36,
    FujitsuFr20 = 37,
    TrwRh32 = 38,
    MotorolaRce = 39,
    Arm = 40,
    FakeAlpha = 41,
    HitachiSh = 42,
    SparcV9 = 43,
    SiemensTricore = 44,
    ArgonautRiscCore = 45,
    HitachiH8_300 = 46,
    HitachiH8_300H = 47,
    HitachiH8S = 48,
    HitachiH8_500 = 49,
    IA_64 = 50,
    StanfordMipsX = 51,
    MototolaColdfire = 52,
    Mototola68Hc12 = 53,
    FujitsuMultimediaAccelerator = 54,
    SiemensPcp = 55,
    SonyNcpu = 56,
    DensoNdr1 = 57,
    MotorolaStarCore = 58,
    ToyotaMe16 = 59,
    STMicroelectronicST100 = 60,
    TinyJ = 61,
    X86_64 = 62,
    SonyDsp = 63,
    SiemensFx66 = 66,
    STMicroelectronicsSt9Plus = 67,
    STMicroelectronicsSt7 = 68,
    Motorola68Hc16 = 69,
    Motorola68Hc11 = 70,
    Motorola68Hc08 = 71,
    Motorola68Hc05 = 72,
    SiliconGraphicsSVx = 73,
    STMicroelectronicsSt19 = 74,
    DigitalVax = 75,
    AxisCommunicationsCris = 76,
    InfineonJavelin = 77,
    Element14Firepath = 78,
    LsiZsp = 79,
    DonaldKnuthMmix = 80,
    HarvardUniversityAny = 81,
    SiTeraPrism = 82,
    AtmelAvr = 83,
    FujitsuFr30 = 84,
    MitsubishiD10V = 85,
    MitsubishiD30V = 86,
    NecV850 = 87,
    MitsubishiM32R = 88,
    MatsushitaMn10300 = 89,
    MatsushitaMn10200 = 90,
    PicoJava = 91,
    OpenRISC = 92,
    ArcTangentA5 = 93,
    TensilicaXtensa = 94,
    AlteraNios2 = 113,
    Aarch64 = 183,
    TilePro = 188,
    XilinxMicroBlaze = 189,
    TileraTileGx = 191,
}

impl Machine {
    fn from(val: u16) -> Maybe<Machine, u16> {
        match val {
            0 => Known(Machine::NoMachine),
            1 => Known(Machine::AtatWe32100),
            2 => Known(Machine::Sparc),
            3 => Known(Machine::I386),
            4 => Known(Machine::Motorola68K),
            5 => Known(Machine::Motorola88K),
            7 => Known(Machine::Intel80860),
            8 => Known(Machine::Mips),
            9 => Known(Machine::IbmSystem370),
            10 => Known(Machine::MipsRs3Le),
            15 => Known(Machine::HppaPaRisc),
            17 => Known(Machine::FujitsuVpp500),
            18 => Known(Machine::Sparc32Plus),
            19 => Known(Machine::Intel80960),
            20 => Known(Machine::PowerPc),
            21 => Known(Machine::PowerPc64),
            22 => Known(Machine::IbmSystem390),
            36 => Known(Machine::NecV800),
            37 => Known(Machine::FujitsuFr20),
            38 => Known(Machine::TrwRh32),
            39 => Known(Machine::MotorolaRce),
            40 => Known(Machine::Arm),
            41 => Known(Machine::FakeAlpha),
            42 => Known(Machine::HitachiSh),
            43 => Known(Machine::SparcV9),
            44 => Known(Machine::SiemensTricore),
            45 => Known(Machine::ArgonautRiscCore),
            46 => Known(Machine::HitachiH8_300),
            47 => Known(Machine::HitachiH8_300H),
            48 => Known(Machine::HitachiH8S),
            49 => Known(Machine::HitachiH8_500),
            50 => Known(Machine::IA_64),
            51 => Known(Machine::StanfordMipsX),
            52 => Known(Machine::MototolaColdfire),
            53 => Known(Machine::Mototola68Hc12),
            54 => Known(Machine::FujitsuMultimediaAccelerator),
            55 => Known(Machine::SiemensPcp),
            56 => Known(Machine::SonyNcpu),
            57 => Known(Machine::DensoNdr1),
            58 => Known(Machine::MotorolaStarCore),
            59 => Known(Machine::ToyotaMe16),
            60 => Known(Machine::STMicroelectronicST100),
            61 => Known(Machine::TinyJ),
            62 => Known(Machine::X86_64),
            63 => Known(Machine::SonyDsp),
            66 => Known(Machine::SiemensFx66),
            67 => Known(Machine::STMicroelectronicsSt9Plus),
            68 => Known(Machine::STMicroelectronicsSt7),
            69 => Known(Machine::Motorola68Hc16),
            70 => Known(Machine::Motorola68Hc11),
            71 => Known(Machine::Motorola68Hc08),
            72 => Known(Machine::Motorola68Hc05),
            73 => Known(Machine::SiliconGraphicsSVx),
            74 => Known(Machine::STMicroelectronicsSt19),
            75 => Known(Machine::DigitalVax),
            76 => Known(Machine::AxisCommunicationsCris),
            77 => Known(Machine::InfineonJavelin),
            78 => Known(Machine::Element14Firepath),
            79 => Known(Machine::LsiZsp),
            80 => Known(Machine::DonaldKnuthMmix),
            81 => Known(Machine::HarvardUniversityAny),
            82 => Known(Machine::SiTeraPrism),
            83 => Known(Machine::AtmelAvr),
            84 => Known(Machine::FujitsuFr30),
            85 => Known(Machine::MitsubishiD10V),
            86 => Known(Machine::MitsubishiD30V),
            87 => Known(Machine::NecV850),
            88 => Known(Machine::MitsubishiM32R),
            89 => Known(Machine::MatsushitaMn10300),
            90 => Known(Machine::MatsushitaMn10200),
            91 => Known(Machine::PicoJava),
            92 => Known(Machine::OpenRISC),
            93 => Known(Machine::ArcTangentA5),
            94 => Known(Machine::TensilicaXtensa),
            113 => Known(Machine::AlteraNios2),
            183 => Known(Machine::Aarch64),
            188 => Known(Machine::TilePro),
            189 => Known(Machine::XilinxMicroBlaze),
            191 => Known(Machine::TileraTileGx),
            _ => Unknown(val)
        }
    }
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

// struct Header {
//   magic: [u8; 4],	// "\x7fELF"
//   bits: u8, // 1 = 32-bit, 2 = 64-bit
//   endian: u8, // 1 = little-endian, 2 = big-endian
//   version: u8, // 1 = only valid value
//   abi: u8, // 0 = UNIX System V
//               // 1 = HP-UX
//               // 2 = NetBSD
//               // 3 = Linux
//               // 6 = Solaris
//               // 7 = IBM AIX
//               // 8 = SGI Irix
//               // 9 = FreeBSD
//               // 10 = Compaq TRU64 UNIX
//               // 11 = Novell Modesto
//               // 12 = OpenBSD
//               // 64 = ARM EABI
//               // 97 = ARM
//               // 255 = Standalone (embedded) application
//   abi_version: u8, // options depend on abi above
//   padding: [u8; 7],
//
//   // ...
// }

pub struct Object<'a> {
    data: &'a [u8]
}

impl<'a> Object<'a> {
    pub fn new(data: &'a [u8]) -> Result<Object<'a>, ()> {
        if data.len() < 8 {
            return Err(());
        }

        if &data[..4] != b"\x7fELF" {
            return Err(());
        }

        if data[6] != 1 {
            return Err(());
        }

        Ok(Object { data: data })
    }

    pub fn bits(&self) -> Maybe<Bits, u8> {
        Bits::from(self.data[4])
    }

    pub fn endian(&self) -> Maybe<Endian, u8> {
        Endian::from(self.data[5])
    }

    pub fn abi(&self) -> Maybe<OsAbi, u8> {
        OsAbi::from(self.data[7])
    }

    pub fn abi_version(&self) -> u8 {
        self.data[8]
    }

    fn read_u16(&self, offset: usize) -> u16 {
        self.data[offset] as u16 + ((self.data[offset + 1] as u16) << 8)
    }

    pub fn object_type(&self) -> Maybe<ObjectType, u16> {
        ObjectType::from(self.read_u16(16))
    }

    pub fn machine(&self) -> Maybe<Machine, u16> {
        Machine::from(self.read_u16(18))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
