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

#[cfg(test)]
mod tests {
    use super::Participant;
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
}
