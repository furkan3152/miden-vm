use alloc::string::ToString;
#[cfg(feature = "std")]
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use super::*;

#[test]
fn print_ln_event_roundtrips() {
    assert_eq!(Event::from(PRINTLN_EVENT.to_event_id()), Event::PrintLn);
    assert_eq!(Event::PrintLn.as_event_id(), *PRINTLN_EVENT_ID);
    assert_eq!(Event::from(PRINTLN_EVENT), Event::PrintLn);
}

#[test]
fn event_metadata_covers_builtin_custom_and_unknown_events() {
    let custom_name = EventName::new("test::custom");
    let custom = Event::from(custom_name);
    let unknown = Event::Unknown(EventId::from_u64(99));

    for event in [Event::FrameStart, Event::FrameEnd, Event::PrintLn] {
        assert!(event.as_event_name().is_some());
        assert!(event.has_builtin_handler());
        assert!(!event.to_string().is_empty());
    }
    assert!(custom.as_event_name().is_some());
    assert!(!custom.has_builtin_handler());
    assert_eq!(custom.to_string(), "test::custom");
    assert!(unknown.as_event_name().is_none());
    assert!(!unknown.has_builtin_handler());
    assert_eq!(unknown.to_string(), "99");

    assert!(Event::FrameStart.is_frame_start());
    assert!(!Event::FrameEnd.is_frame_start());
    assert!(Event::FrameEnd.is_frame_end());
    assert!(!Event::PrintLn.is_frame_end());
    assert_eq!(EventId::from(Event::FrameStart), *FRAME_START_EVENT_ID);
}

#[test]
fn event_order_is_total_and_agrees_with_equality() {
    let named = EventName::new("test::a");
    let events = [
        Event::FrameStart,
        Event::FrameEnd,
        Event::PrintLn,
        Event::from(named.clone()),
        Event::from(EventName::new("test::b")),
        Event::Unknown(named.to_event_id()),
        Event::Unknown(PRINTLN_EVENT.to_event_id()),
        Event::Unknown(EventId::from_u64(0)),
        Event::Unknown(EventId::from_u64(u64::MAX)),
    ];

    for a in &events {
        for b in &events {
            assert_eq!(a.cmp(b) == core::cmp::Ordering::Equal, a == b, "{a:?} vs {b:?}");
            assert_eq!(a.cmp(b), b.cmp(a).reverse(), "{a:?} vs {b:?}");
            assert_eq!(a.partial_cmp(b), Some(a.cmp(b)));
            for c in &events {
                if a <= b && b <= c {
                    assert!(a <= c, "{a:?} <= {b:?} <= {c:?} but not {a:?} <= {c:?}");
                }
            }
        }
    }

    // The case from #4264: a named event, the same event decoded from its ID, and a third event.
    let a = Event::from(named.clone());
    let c = Event::from(named.to_event_id());
    let b = Event::from(EventName::new("test::b"));
    assert_eq!(a.cmp(&c), core::cmp::Ordering::Equal);
    assert_eq!(a.cmp(&b), c.cmp(&b));
}

#[test]
#[cfg(feature = "std")]
fn equal_events_hash_equally() {
    let named = EventName::new("test::custom");
    let hash = |event: &Event| {
        let mut hasher = DefaultHasher::new();
        event.hash(&mut hasher);
        hasher.finish()
    };
    assert_eq!(hash(&Event::from(named.clone())), hash(&Event::Unknown(named.to_event_id())));
    assert_eq!(hash(&Event::PrintLn), hash(&Event::Unknown(PRINTLN_EVENT.to_event_id())));
}
