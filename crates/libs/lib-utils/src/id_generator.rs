use chrono::{TimeZone, Utc};
use sonyflake::Sonyflake;
use std::fmt::Display;

pub struct IdGenerator {
    worker_id: u16,
    sonyflake: Sonyflake,
}

impl Display for IdGenerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.worker_id)
    }
}

impl IdGenerator {
    pub fn new(worker_id: u16) -> Result<IdGenerator, sonyflake::Error> {
        let start_time = Utc.with_ymd_and_hms(2025, 3, 13, 0, 0, 0).unwrap();

        let sonyflake = Sonyflake::builder()
            .start_time(start_time)
            .machine_id(&|| Ok(worker_id))
            .finalize()?;
        Ok(IdGenerator {
            worker_id,
            sonyflake,
        })
    }

    pub fn next_id(&self) -> Result<i64, sonyflake::Error> {
        self.sonyflake.next_id().map(|id| id.cast_signed())
    }
}
