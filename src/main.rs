#![feature(plugin)]
#![plugin(clippy)]

struct Record {
    id: u64,
}

impl Record {
    fn new(id: u64) -> Record {
        Record {
            id: id,
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
