extern crate scrummin;
extern crate chrono;

#[cfg(not(test))]
use scrummin::Participant;

#[cfg(not(test))]
fn main() {
    let participant = Participant::new("Bob");
    println!("{}", participant.name);
}
