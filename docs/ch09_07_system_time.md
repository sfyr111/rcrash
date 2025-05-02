# 9.7 System Time in Rust

This chapter demonstrates how to use `std::time::SystemTime` for system clock, timestamps, and duration calculations.

---

## When to Use SystemTime?

- To get the current system time (wall clock)
- To calculate durations between time points
- To measure elapsed time for profiling
- To convert to/from UNIX timestamps

---

## Example Code

```rust
use std::time::{SystemTime, UNIX_EPOCH, Duration};

fn main() {
    // 1. Get current system time
    let now = SystemTime::now();
    println!("Current system time: {:?}", now);

    // 2. Convert to UNIX timestamp (seconds since 1970-01-01 00:00:00 UTC)
    match now.duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            println!("UNIX timestamp: {} seconds", duration.as_secs());
            println!("UNIX timestamp: {} ms", duration.as_millis());
        },
        Err(e) => println!("SystemTime before UNIX EPOCH! Error: {:?}", e),
    }

    // 3. Add and subtract durations
    let five_secs = Duration::new(5, 0);
    let later = now + five_secs;
    let earlier = now - five_secs;
    println!("5 seconds later: {:?}", later);
    println!("5 seconds earlier: {:?}", earlier);

    // 4. Measure elapsed time
    let start = SystemTime::now();
    // Simulate work
    std::thread::sleep(Duration::from_millis(100));
    let elapsed = start.elapsed().expect("Time went backwards");
    println!("Elapsed: {} ms", elapsed.as_millis());

    // 5. Create SystemTime from UNIX timestamp
    let unix_ts = 1_000_000_000u64; // seconds
    let t = UNIX_EPOCH + Duration::from_secs(unix_ts);
    println!("SystemTime for UNIX timestamp {}: {:?}", unix_ts, t);
}
```

## Common SystemTime Operations

- `SystemTime::now()` — get the current time
- `duration_since(UNIX_EPOCH)` — convert to UNIX timestamp
- `SystemTime + Duration`, `SystemTime - Duration` — arithmetic
- `elapsed()` — measure duration since a time point
- Create from UNIX timestamp: `UNIX_EPOCH + Duration::from_secs(...)`

---

## Key Points

- `SystemTime` is for wall-clock time, not monotonic time (see `Instant` for monotonic timing)
- All times are in UTC
- Always handle possible errors (system clock can go backwards)
- For formatting/parsing, consider using the `chrono` or `time` crate

---

Run the code:

```sh
cargo run --bin ch09_07_system_time
```
