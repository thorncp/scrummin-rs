extern crate scrummin;
extern crate chrono;

#[cfg(not(test))]
use scrummin::{Meeting, Participant};

#[cfg(not(test))]
fn main() {
    let mut meeting = Meeting::new();
    let bob = &Participant::new("Bob");
    let sue = &Participant::new("Sue");
    let joanne = &Participant::new("Joanne");

    meeting.add_participant(bob);
    meeting.add_participant(sue);
    meeting.add_participant(joanne);

    meeting.start();

    for participant in meeting {
        println!("{}", participant.name);
    }
}
