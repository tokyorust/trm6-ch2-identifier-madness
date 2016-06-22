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
    id: Identifier,
}

impl Record {
    fn new<I: Into<Identifier>>(id: I) -> Record {
        Record {
            id: id.into(),
        }
    }
}

fn main() {
    let rec = Record::new(1);
    println!("We have record {}!", rec.id);
}

#[test]
fn should_accept_u64() {
    Record::new(0 as u64);
}

#[test]
fn should_accept_uuid_string() {
    Record::new("49a6319c-38a9-11e6-8ed5-97272550a380".to_owned());
}

#[test]
fn should_accept_uuid_str() {
    Record::new("49a6319c-38a9-11e6-8ed5-97272550a380");
}
