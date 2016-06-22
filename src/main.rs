#![feature(plugin)]
#![plugin(clippy)]

use std::fmt;

enum Identifier {
    UUID(String),
    Numeric(u64),
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Identifier::UUID(ref id) => write!(f, "{}", id),
            Identifier::Numeric(ref id) => write!(f, "{}", id),
        }
    }
}

impl From<String> for Identifier {
    fn from(id: String) -> Identifier {
        Identifier::UUID(id)
    }
}

impl<'a> From<&'a str> for Identifier {
    fn from(id: &'a str) -> Identifier {
        Identifier::UUID(id.into())
    }
}

impl From<u64> for Identifier {
    fn from(id: u64) -> Identifier {
        Identifier::Numeric(id)
    }
}

struct Record {
    id: Option<Identifier>,
}

impl Record {
    fn new<I: Into<Identifier>>(id: Option<I>) -> Record {
        Record {
            id: id.map(|v| v.into()),
        }
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.id {
            Some(ref id) => write!(f, "{}", id),
            None => write!(f, "<NEW>"),
        }
    }
}

fn main() {
    let rec = Record::new(Some(1));
    println!("We have record {}!", rec);
}

#[test]
fn should_accept_u64() {
    Record::new(Some(0 as u64));
}

#[test]
fn should_accept_uuid_string() {
    Record::new(Some("49a6319c-38a9-11e6-8ed5-97272550a380".to_owned()));
}

#[test]
fn should_accept_uuid_str() {
    Record::new(Some("49a6319c-38a9-11e6-8ed5-97272550a380"));
}

#[test]
fn should_accept_none() {
    Record::new::<String>(None);
    Record::new::<u64>(None);
}
