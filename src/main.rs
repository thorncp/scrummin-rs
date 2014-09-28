extern crate scrummin;
extern crate chrono;

#[cfg(not(test))]
use scrummin::{Meeting, Participant};
#[cfg(not(test))]
use std::os;

#[cfg(not(test))]
fn main() {
    let mut meeting = Meeting::new();
    let participants: Vec<Participant> =
        os::args().iter().skip(1).map(|a| Participant::new(a.as_slice())).collect();

    for participant in participants.iter() {
        meeting.add_participant(participant);
    }

    meeting.start();

    for participant in meeting {
        println!("{}", participant.name);
    }
}
