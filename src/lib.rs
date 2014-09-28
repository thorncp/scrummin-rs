extern crate chrono;

use chrono::{DateTime, UTC};
use std::time::duration::Duration;

pub struct Participant {
    pub name: String,
    pub started_at: Option<DateTime<UTC>>,
    pub ended_at: Option<DateTime<UTC>>,
}

impl Participant {
    pub fn new(name: &str) -> Participant {
        Participant {
            name: name.to_string(),
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

pub struct Meeting<'m> {
    participants: Vec<&'m Participant>,
    // TODO: store Iterator instead of hand rolling?
    current: uint
}

impl<'m> Meeting<'m> {
    pub fn new() -> Meeting<'m> {
        Meeting { participants: Vec::new(), current: 0 }
    }

    pub fn add_participant(&mut self, participant: &'m Participant) {
        self.participants.push(participant);
    }

    pub fn start(&mut self) {
    }
}

impl<'m> Iterator<&'m Participant> for Meeting<'m> {
    fn next(&mut self) -> Option<&'m Participant> {
        if self.current < self.participants.len() {
            self.current += 1;
            Some(self.participants[self.current - 1])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Meeting, Participant};
    use chrono::UTC;

    #[test]
    fn test_new_has_name_with_no_time() {
        let participant = Participant::new("Bob");
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
        let mut participant = Participant::new("Bob");
        assert_eq!(participant.duration(), None);

        participant.started_at = Some(UTC::now());
        assert_eq!(participant.duration(), None);

        participant.started_at = None;
        participant.ended_at = Some(UTC::now());
        assert_eq!(participant.duration(), None);
    }

    #[test]
    fn test_can_add_participants_to_meeting() {
        let bob = Participant::new("Bob");
        let sue = Participant::new("Sue");

        let mut meeting = Meeting::new();
        meeting.add_participant(&bob);
        meeting.add_participant(&sue);

        assert!(meeting.participants.iter().any(|p| p.name.as_slice() == "Bob"));
        assert!(meeting.participants.iter().any(|p| p.name.as_slice() == "Sue"));
    }

    #[test]
    fn test_move_to_next_participant() {
        let bob = Participant::new("Bob");
        let sue = Participant::new("Sue");

        let mut meeting = Meeting::new();
        meeting.add_participant(&bob);
        meeting.add_participant(&sue);

        meeting.start();
        assert_eq!(meeting.next().unwrap().name.as_slice(), "Bob");
        assert_eq!(meeting.next().unwrap().name.as_slice(), "Sue");
    }
}
