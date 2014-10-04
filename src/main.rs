extern crate scrummin;
extern crate chrono;

#[cfg(not(test))]
use scrummin::{Meeting};
#[cfg(not(test))]
use std::os;

#[cfg(not(test))]
fn main() {
    let args = os::args();
    let names = args.tail();

    let mut meeting = Meeting::new();

    meeting.build_participants(names);
    meeting.start();

    for participant in meeting.participants() {
        println!("{}", participant.name);
    }
}
