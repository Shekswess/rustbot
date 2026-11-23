// Added in the cppalliance fork of RustBot. See "Changes in this fork" in README.md.

//! The time that a turn reads.
//!
//! A caller can give a bot a fixed time with `Bot::set_now`, so that a reply
//! with a date in it comes out the same on every day. One turn reads more
//! than one line: the line the user wrote, earlier lines of the
//! conversation, and reworded forms of both. Each of those readings must see
//! the same time. So the turn holds its time here, for its own thread, for as
//! long as it runs.
//!
//! Time zones come from the database inside the executable. The zone files
//! of the host can be older, newer or absent, and a reply must be the same on
//! every host.

use jiff::{
    Zoned,
    fmt::temporal::DateTimeParser,
    tz::{TimeZone, TimeZoneDatabase},
};
use std::{cell::RefCell, sync::LazyLock};

thread_local! {
    static TURN_TIME: RefCell<Option<Zoned>> = const { RefCell::new(None) };
}

/// The time zone database inside the executable.
static ZONES: LazyLock<TimeZoneDatabase> = LazyLock::new(TimeZoneDatabase::bundled);

/// The time zone with this IANA name, such as `Europe/London`.
pub(crate) fn zone(name: &str) -> Option<TimeZone> {
    ZONES.get(name).ok()
}

/// Reads a time with a time zone, such as `2026-11-22T00:00:00+00:00[UTC]`.
pub(crate) fn read(text: &str) -> Result<Zoned, jiff::Error> {
    static PARSER: DateTimeParser = DateTimeParser::new();
    PARSER.parse_zoned_with(&ZONES, text)
}

/// Holds the time of a running turn. When it is dropped, the time that was
/// held before comes back, also when the turn ends in a panic.
pub(crate) struct TurnTime {
    earlier: Option<Zoned>,
}

impl TurnTime {
    /// Holds `now` for the turn on this thread. `None` leaves the turn on the
    /// system clock.
    pub(crate) fn hold(now: Option<Zoned>) -> Self {
        Self {
            earlier: TURN_TIME.replace(now),
        }
    }
}

impl Drop for TurnTime {
    fn drop(&mut self) {
        TURN_TIME.set(self.earlier.take());
    }
}

/// The time of the turn that runs on this thread: the time its bot was given,
/// or the system clock.
pub(crate) fn now() -> Zoned {
    TURN_TIME
        .with_borrow(Clone::clone)
        .unwrap_or_else(Zoned::now)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn time(text: &str) -> Zoned {
        text.parse().unwrap()
    }

    #[test]
    fn a_held_time_is_the_time_until_it_is_dropped() {
        let fixed = time("2001-02-03T04:05:06+00:00[UTC]");
        {
            let _held = TurnTime::hold(Some(fixed.clone()));
            assert_eq!(now(), fixed);
            assert_eq!(now(), fixed, "the clock stands still inside a turn");
        }
        assert!(now().year() > 2001, "the system clock is back");
    }

    #[test]
    fn a_turn_inside_a_turn_gives_the_outer_time_back() {
        let outer = time("2001-02-03T04:05:06+00:00[UTC]");
        let inner = time("2010-11-12T13:14:15+00:00[UTC]");
        let _held = TurnTime::hold(Some(outer.clone()));
        {
            let _held = TurnTime::hold(Some(inner.clone()));
            assert_eq!(now(), inner);
        }
        assert_eq!(now(), outer);
        {
            let _held = TurnTime::hold(None);
            assert!(
                now().year() > 2010,
                "a turn without a time reads the system clock"
            );
        }
        assert_eq!(now(), outer);
    }

    #[test]
    fn a_panic_gives_the_earlier_time_back() {
        let fixed = time("2001-02-03T04:05:06+00:00[UTC]");
        let outcome = std::panic::catch_unwind(|| {
            let _held = TurnTime::hold(Some(time("2001-02-03T04:05:06+00:00[UTC]")));
            panic!("a rule failed");
        });
        assert!(outcome.is_err());
        assert_ne!(now(), fixed);
    }

    #[test]
    fn zones_come_from_the_database_inside_the_executable() {
        let london = zone("Europe/London").unwrap();
        let new_york = zone("America/New_York").unwrap();
        let winter: jiff::Timestamp = "2026-11-22T00:00:00Z".parse().unwrap();
        assert_eq!(winter.to_zoned(london.clone()).offset().seconds(), 0);
        assert_eq!(winter.to_zoned(new_york).offset().seconds(), -5 * 3600);
        let summer: jiff::Timestamp = "2026-07-01T00:00:00Z".parse().unwrap();
        assert_eq!(summer.to_zoned(london).offset().seconds(), 3600);
        assert!(zone("Nowhere/Nothing").is_none());
    }

    #[test]
    fn a_time_is_read_with_a_zone_from_the_same_database() {
        let utc = read("2026-11-22T00:00:00+00:00[UTC]").unwrap();
        let new_york = read("2026-11-21T19:00:00-05:00[America/New_York]").unwrap();
        assert_eq!(utc.timestamp(), new_york.timestamp());
        assert_eq!(new_york.date().day(), 21);
        // A zone that the database lacks, and an offset that its zone never has.
        assert!(read("2026-11-22T00:00:00+00:00[Nowhere/Nothing]").is_err());
        assert!(read("2026-11-22T00:00:00+03:00[America/New_York]").is_err());
    }

    #[test]
    fn each_thread_holds_its_own_time() {
        let fixed = time("2001-02-03T04:05:06+00:00[UTC]");
        let _held = TurnTime::hold(Some(fixed.clone()));
        let other = std::thread::spawn(now).join().unwrap();
        assert!(other.year() > 2001, "another thread reads the system clock");
        assert_eq!(now(), fixed);
    }
}
