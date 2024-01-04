# TimeSource for chrono

This library should help improving testing so that it's possible to manually
set the time during the tests.

How to use the library:

```
use chrono::prelude::*;
use chrono_timesource::{TimeSource, ManualTimeSource};
use std::sync::{Arc, Mutex};

// ...

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
```


