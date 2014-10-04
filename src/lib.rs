extern crate chrono;

use chrono::{DateTime, UTC};
use std::time::duration::Duration;
use std::rand::{task_rng, Rng};
use std::slice::Items;

pub struct Participant {
    pub name: String,
    pub started_at: Option<DateTime<UTC>>,
    pub ended_at: Option<DateTime<UTC>>,
}

impl Participant {
    pub fn new(name: String) -> Participant {
        Participant {
            name: name,
            started_at: None,
            ended_at: None
        }
    }

    pub fn duration(&self) -> Option<Duration> {
        match (self.started_at, self.ended_at) {
            (Some(start), Some(end)) => Some(end - start),
            _ => None
        }
    }
}

pub struct Meeting {
    participants: Vec<Participant>
}

impl Meeting {
    pub fn new() -> Meeting {
        Meeting { participants: Vec::new() }
    }

    pub fn add_participant(&mut self, participant: Participant) {
        self.participants.push(participant);
    }

    pub fn start(&mut self) {
        let mut rng = task_rng();
        rng.shuffle(self.participants.as_mut_slice());
    }

    pub fn build_participants(&mut self, names: &[String]) {
        for name in names.iter() {
            self.participants.push(Participant::new(name.clone()))
        }
    }

    pub fn participants<'a>(&'a mut self) -> Items<'a, Participant> {
        self.participants.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::{Meeting, Participant};
    use chrono::UTC;

    #[test]
    fn test_new_has_name_with_no_time() {
        let participant = Participant::new("Bob".to_string());
        assert_eq!(participant.name.as_slice(), "Bob");
        assert_eq!(participant.started_at, None);
        assert_eq!(participant.ended_at, None);
    }

    #[test]
    fn test_knows_duration() {
        let participant = Participant {
            name: "Bob".to_string(),
            started_at: Some(UTC::today().and_hms(0, 1, 5)),
            ended_at: Some(UTC::today().and_hms(0, 6, 45)),
        };

        // five minutes forty seconds
        assert_eq!(participant.duration().unwrap().num_seconds(), 5 * 60 + 40);
    }

    #[test]
    fn test_duration_requires_times() {
        let mut participant = Participant::new("Bob".to_string());
        assert_eq!(participant.duration(), None);

        participant.started_at = Some(UTC::now());
        assert_eq!(participant.duration(), None);

        participant.started_at = None;
        participant.ended_at = Some(UTC::now());
        assert_eq!(participant.duration(), None);
    }

    #[test]
    fn test_can_add_participants_to_meeting() {
        let bob = Participant::new("Bob".to_string());
        let sue = Participant::new("Sue".to_string());

        let mut meeting = Meeting::new();
        meeting.add_participant(bob);
        meeting.add_participant(sue);

        assert!(meeting.participants.iter().any(|p| p.name.as_slice() == "Bob"));
        assert!(meeting.participants.iter().any(|p| p.name.as_slice() == "Sue"));
    }

    #[test]
    fn test_move_to_next_participant() {
        let bob = Participant::new("Bob".to_string());
        let sue = Participant::new("Sue".to_string());

        let mut meeting = Meeting::new();
        meeting.add_participant(bob);
        meeting.add_participant(sue);

        meeting.start();

        for participant in meeting.participants() {
            let name = participant.name.as_slice();
            assert!(name == "Bob" || name == "Sue");
        }
    }
}
