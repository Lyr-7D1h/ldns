use core::fmt;

#[derive(Debug, Clone, Copy)]
pub enum QClass {
    IN = 1, // Internet
    CS = 2, // CSNET (Deprecated)
    CH = 3, // Chaos
    HS = 4, // Hesoid
    None = 254,
    Any = 255,
}

impl TryFrom<u16> for QClass {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(QClass::IN),
            2 => Ok(QClass::CS),
            3 => Ok(QClass::CH),
            4 => Ok(QClass::HS),
            254 => Ok(QClass::None),
            255 => Ok(QClass::Any),
            _ => Err("Unknown value for qclass"),
        }
    }
}

impl fmt::Display for QClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

// TODO implement all https://en.wikipedia.org/wiki/List_of_DNS_record_types
#[derive(Debug, Clone, Copy)]
pub enum QType {
    A,
    AAAA,
    NS,
    MD,
    MF,
    CNAME,
    SOA,
    MB,
    MG,
    MR,
    NULL,
    WKS,
    PTR,
    HINFO,
    MINFO,
    MX,
    TXT,
    AXFR,
    MAILB,
    MAILA,
    ALL,
}
impl QType {
    pub fn from_u16(value: u16) -> Option<QType> {
        match value {
            1 => Some(QType::A),
            2 => Some(QType::NS),
            3 => Some(QType::MD),
            4 => Some(QType::MF),
            5 => Some(QType::CNAME),
            6 => Some(QType::SOA),
            7 => Some(QType::MB),
            8 => Some(QType::MG),
            9 => Some(QType::MR),
            10 => Some(QType::NULL),
            11 => Some(QType::WKS),
            12 => Some(QType::PTR),
            13 => Some(QType::HINFO),
            14 => Some(QType::MINFO),
            15 => Some(QType::MX),
            16 => Some(QType::TXT),
            28 => Some(QType::AAAA),
            252 => Some(QType::AXFR),
            253 => Some(QType::MAILB),
            254 => Some(QType::MAILA),
            255 => Some(QType::ALL),
            _ => None,
        }
    }
}
impl fmt::Display for QType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Question {
    pub qname: String,
    pub qtype: QType,
    pub qclass: QClass,
}
