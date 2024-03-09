# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

# [Unreleased]

- Added `events` feature to enable / disable the `events` module.
- Added `dynamic` feature to enable / disable dynamic type features
  engine-wide.
- Added `wolf_engine_events` crate.
  - Added public re-export of `wolf_engine_events` as `events`.
- Added `wolf_engine_codegen` crate.

## [wolf_engine_events]

### [0.1]

- Added `dynamic` feature to enable / disable the `dynamic` module.
- Added `dynamic` module.
  - Added `event_loop` module.
    - Added `EventLoop` struct.
      - Added `EventReceiver` impl.
      - Added `Default` impl.
      - Added `event_sender()` method.
  - Added `events` module.
    - Added `Quit` event.
    - Added `EventsCleared` event.
  - Added `DynamicEvent` trait.
  - Added `DynamicEventSender` trait.
    - Added auto-impl for `EventSender<DynamicEventBox>` types.
    - Added a provided `send_event()` method.
  - Added `DynamicEventBox` type-def of `Box<dyn DynamicEvent>`.
  - Added `DynamicEvent` derive macro.
- Added `mpsc` module.
  - Added `MpscEventReceiver` struct.
    - Added `EventReceiver` impl.
  - Added `MpscEvenSender` struct. 
    - Added `EventReceiver` impl.
  - Added `event_queue()` function.
- Added `ReceiverDroppedError` struct.
- Added `EventReceiver` trait.
  - Added `next_event()` method.
- Added `EventSender` trait.
  - Added `send_event()` method.

## [wolf_engine_codegen]

### [0.1]

- Added `DynamicEvent` derive macro.

# [0.1 - 0.25] 2021-08-11 - 2024-03-02

These are the old, prototype versions of Wolf Engine, and changes to them
are mostly undocumented.  You can find these versions in the
[wolf_engine_old](https://github.com/AlexiWolf/wolf_engine_old) repo.

This change log will start at v0.26, and document all code brought over from
the old codebase as if it were new additions.
