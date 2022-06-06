use core::fmt;

#[derive(Debug)]
pub enum Class {
    IN,
    CS,
    CH,
    HS,
}
impl Class {
    pub fn from_u16(value: u16) -> Option<Class> {
        match value {
            1 => Some(Class::IN),
            2 => Some(Class::CS),
            3 => Some(Class::CH),
            4 => Some(Class::HS),
            _ => None,
        }
    }
}
impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

// TODO implement all https://en.wikipedia.org/wiki/List_of_DNS_record_types
#[derive(Debug)]
pub enum Type {
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
}
impl Type {
    pub fn from_u16(value: u16) -> Option<Type> {
        match value {
            1 => Some(Type::A),
            2 => Some(Type::NS),
            3 => Some(Type::MD),
            4 => Some(Type::MF),
            5 => Some(Type::CNAME),
            6 => Some(Type::SOA),
            7 => Some(Type::MB),
            8 => Some(Type::MG),
            9 => Some(Type::MR),
            10 => Some(Type::NULL),
            11 => Some(Type::WKS),
            12 => Some(Type::PTR),
            13 => Some(Type::HINFO),
            14 => Some(Type::MINFO),
            15 => Some(Type::MX),
            16 => Some(Type::TXT),
            28 => Some(Type::AAAA),
            _ => None,
        }
    }
}
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Record {
    pub name: String,
    pub r#type: Type,
    pub class: Class,
    pub ttl: u32,
    // TODO give own struct with known structure for example for A records specify an Ipv4 struct
    pub rdata: String,
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\t{}{}\t{}\t{}\t{}",
            self.name,
            if self.name.len() < 30 { "\t" } else { "" }, // TODO use a more solid way of formatting, using this hack now to pass tests
            self.ttl,
            self.class,
            self.r#type,
            self.rdata
        )
    }
}
