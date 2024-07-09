# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

# [Unreleased]

- Added `wolf_engine_input` crate.
  - Added public re-export of `wolf_engine_input` as `input`.
  - Added `input` feature to enable / disable the `input` module.
    - Added `input` feature to the default feature-set.
  - Added `winit` feature to enable / disable Winit integration engine-wide.
- Added `wolf_engine_events` crate.
  - Added public re-export of `wolf_engine_events` as `events`.
- Added `wolf_engine_window` crate.
  - Added public re-export of `wolf_engine_window` as `window`.
  - Added `window` feature to enable the `window` module.

## [wolf_engine_window]

### [Unreleased]

- Added `init()` fn.
- Added `WindowContext` struct.
- Added `WindowContextBuilder` struct.
- Added `WindowSettings` struct.
- Added `WindowEvent` enum
  - Added `Resized` variant.
  - Added `RedrawRequested` variant.
  - Added `Resized` variant.
  - Added `Closed` variant.
- Added `raw_window_handle` module.
  - Added `rwh_05` module.
  - Added `rwh_06` module.
  - Added `HasRawWindowHandles` trait.
  - Added `HasRwh5Handles` trait.
  - Added `HasRwh6Handles` trait.

## [wolf_engine_input]

### [0.1.2] - 2023-07-09

- Updated `winit` integration to `v0.30.x`.

### [0.1.1] - 2024-03-24

- Changed winit dependency version to any `0.29` release.

### [0.1] - 2024-03-24

- Added `ButtonState` enum.
  - Added `Down` variant.
  - Added `Up` variant.
- Added `Input` enum.
  - Added `Keyboard` variant.
    - Added `state` field.
    - Added `scancode` field.
    - Added `keycode` field.
    - Added `is_repeat` field.
  - Added `RawKeyboard` variant.
    - Added `state` field.
    - Added `scancode` field.
    - Added `keycode` field.
  - Added `MouseMove` variant.
    - Added `x` field.
    - Added `y` field.
  - Added `RawMouseMove` variant.
    - Added `delta_x` field.
    - Added `delta_y` field.
  - Added `MouseButton` variant.
    - Added `state` field.
    - Added `button` field.
  - Added `MouseScroll` variant.
    - Added `delta_x` field.
    - Added `delta_y` field.
- Added `ToInput` trait.
  - Added `to_input()` method.
- Added `keyboard` module.
  - Added `Key` struct.
    - Added `scancode` field. 
    - Added `keycode` field.
  - Added `KeyCode` enum.
    - Added keys corrisponding to a 104-key US QWERTY keyboard.
      - No, I will not list them all out.
 - Added `mouse` module.
    - Added `Mousebutton` enum.
      - Added `Left` variant.
      - Added `Middle` variant.
      - Added `Right` variant.
      - Added `Forward` variant.
      - Added `Back` variant.
      - Added `Other` variant.
        - Added `u32` value.

## [wolf_engine_events]

### [Unreleased]

- Removed the `dynamic` feature.
  - Removed the `dynamic` module.
  - Removed `dynamic` feature to enable / disable the `dynamic` module.
  - Removed `dynamic` module.
    - Removed `event_loop` module.
      - Removed `EventLoop` struct.
        - Removed `EventReceiver` impl.
        - Removed `Default` impl.
        - Removed `event_sender()` method.
    - Removed `events` module.
      - Removed `Quit` event.
      - Removed `EventsCleared` event.
    - Removed `DynamicEvent` trait.
    - Removed `DynamicEventSender` trait.
      - Removed auto-impl for `EventSender<DynamicEventBox>` types.
      - Removed a provided `send_event()` method.
    - Removed `DynamicEventBox` type-def of `Box<dyn DynamicEvent>`.
    - Removed `DynamicEvent` derive macro.

### [0.1] - 2024-03-07

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

### [0.1] - 2024-03-07

- Added `DynamicEvent` derive macro.

# [0.1 - 0.25] - 2021-08-11 - 2024-03-02

These are the old, prototype versions of Wolf Engine, and changes to them
are mostly undocumented.  You can find these versions in the
[wolf_engine_old](https://github.com/AlexiWolf/wolf_engine_old) repo.

This change log will start at v0.26, and document all code brought over from
the old codebase as if it were new additions.
