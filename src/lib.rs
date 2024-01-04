use chrono::prelude::*;
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TimeSourceError {
    DateTimeNotSet,
}

pub trait TimeSource<Tz: TimeZone>: Debug + Send + Sync + Clone {
    fn now(&self) -> Result<DateTime<Tz>, TimeSourceError>;
}

#[derive(Debug, Copy, Clone)]
pub struct UtcTimeSource;

impl TimeSource<Utc> for UtcTimeSource {
    fn now(&self) -> Result<DateTime<Utc>, TimeSourceError> {
        Ok(Utc::now())
    }
}

#[derive(Debug, Clone)]
pub struct ManualTimeSource<Tz: TimeZone> {
    instant: Option<DateTime<Tz>>,
}

impl<Tz: TimeZone> ManualTimeSource<Tz> {
    pub fn new() -> Self {
        ManualTimeSource { instant: None }
    }
    pub fn set_now(&mut self, now: DateTime<Tz>) {
        self.instant = Some(now);
    }
}

impl<Tz: TimeZone + Debug> TimeSource<Tz> for ManualTimeSource<Tz>
where
    <Tz as TimeZone>::Offset: Sync + Send + Clone + Debug,
{
    fn now(&self) -> Result<DateTime<Tz>, TimeSourceError> {
        self.instant.clone().ok_or(TimeSourceError::DateTimeNotSet)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};

    #[test]
    fn manual_now() {
        let mut manual_time_source: ManualTimeSource<Utc> = ManualTimeSource::new();
        manual_time_source.set_now(Utc.with_ymd_and_hms(1970, 1, 1, 0, 1, 1).unwrap());
        assert_eq!(
            manual_time_source.now(),
            Ok(Utc.with_ymd_and_hms(1970, 1, 1, 0, 1, 1).unwrap())
        );
    }

    #[test]
    fn manual_uninitialised() {
        let manual_time_source: ManualTimeSource<Utc> = ManualTimeSource::new();
        assert_eq!(
            manual_time_source.now(),
            Err(TimeSourceError::DateTimeNotSet)
        );
    }

    #[test]
    fn use_in_struct_rc() {
        let manual_time_source: ManualTimeSource<Utc> = ManualTimeSource::new();
        let shared_ts: Rc<RefCell<ManualTimeSource<Utc>>> =
            Rc::new(RefCell::new(manual_time_source));
        struct TimeUser<TS: TimeSource<Utc>> {
            ts: Rc<RefCell<TS>>,
        }
        let tu: TimeUser<ManualTimeSource<Utc>> = TimeUser {
            ts: shared_ts.clone(),
        };

        {
            let mut ts = shared_ts.borrow_mut();
            ts.set_now(Utc.with_ymd_and_hms(1970, 1, 1, 0, 1, 1).unwrap());
        }

        assert_eq!(
            tu.ts.borrow().now(),
            Ok(Utc.with_ymd_and_hms(1970, 1, 1, 0, 1, 1).unwrap())
        );
    }

    #[test]
    fn use_in_struct_arc() {
        let manual_time_source: ManualTimeSource<Utc> = ManualTimeSource::new();
        let shared_ts: Arc<Mutex<ManualTimeSource<Utc>>> = Arc::new(Mutex::new(manual_time_source));
        struct TimeUser<TS: TimeSource<Utc>> {
            ts: Arc<Mutex<TS>>,
        }
        let tu: TimeUser<ManualTimeSource<Utc>> = TimeUser {
            ts: shared_ts.clone(),
        };

        {
            let mut ts = shared_ts.lock().unwrap();
            ts.set_now(Utc.with_ymd_and_hms(1970, 1, 1, 0, 1, 1).unwrap());
        }

        assert_eq!(
            tu.ts.lock().unwrap().now(),
            Ok(Utc.with_ymd_and_hms(1970, 1, 1, 0, 1, 1).unwrap())
        );
    }
}
