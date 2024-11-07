# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

# [Unreleased]

- Added `wolf_engine_input` crate.
  - Added public re-export of `wolf_engine_input` as `input`.
  - Added `input` feature to enable / disable the `input` module.
    - Added `input` feature to the default feature-set.
- Added `wolf_engine_events` crate.
  - Added public re-export of `wolf_engine_events` as `events`.
- Added `wolf_engine_window` crate.
  - Added public re-export of `wolf_engine_window` as `window`.
  - Added `window` feature to enable the `window` module.
- Added `wolf_engine_winit` crate.
  - Added `winit` feature to enable / disable Winit integration engine-wide.

## [wolf_engine_window]

### [0.4] 2024-11-03

- Changed to event-driven window creation.
- Rewrote the window system with support for alternative backends.
  - Addded `backend` module, and `WindowSystem` trait.
  - Removed the `EventLoop` (replaced it with `WindowSystem::run()`.)
  - Removed `init()` function.  An equivalent to this function should be
    provided by the window backend crate.

### [0.3] - 2024-08-12

- Added support for multiple windows.
  - Moved window-creation, and config from `init()` to 
    `WindowContext::create_window()`.
  - Moved the window config-builder methods from `EventLoopBuilder` to 
    `WindowSettings`.
  - Moved general application-level events to `Event`, and window-specific 
    events to `WindowEvent`
- Added additional window config options.
  - Added `Window::set_title()` method.  
- Added a more clear application life-cycle. 
  - Added `EventsCleared` event to indicate when a new frame should begin.
  - Renamed `Resumed` event to `Started` to better match the `Exited` event.
- Added better error-handling when initializing the window system, and 
  creating windows.
- Added support for borderless-fullscreen windows.

- Changed to a separate `WindowContext`, and `EventLoop`.
  - Renamed `WindowContextBuilder` to `EventLoopBuilder`.
  - Moved `run()` method from `WindowContext` to `EventLoop`.
  - Removed `context_state` module.

- Removed automatic re-drawing of windows.
  - Added `Window::redraw()` method. 
- Removed public re-export of the `winit` crate.

### [0.2] - 2024-07-25

- Added type-states to `WindowContext`.
  - Added `context_state` mod.
    - Added `Inactive` struct.
    - Added `Active` struct.
  - Added generic `state` parameter to `WindowContext`.
  - Moved the following methods to `Inactive` state.
    - `run()`.
- Added `Window` struct.
  - Added `WindowContext::window()` accessor to `Active` state.
  - Moved window-related methods from `WindowContext` to the `Window` struct.
    - `size()`.
    - `close()`.
    - All `raw_window_handle` trait impls.


### [0.1] - 2024-07-09

- Added `init()` fn.
- Added `WindowContext` struct.
- Added `WindowContextBuilder` struct.
- Added `WindowSettings` struct.
- Added `WindowEvent` enum
  - Added `Resized` variant.
  - Added `RedrawRequested` variant.
  - Added `Resized` variant.
  - Added `Closed` variant.
  - Added `Input` variant.
- Added `raw_window_handle` module.
  - Added `rwh_05` module.
  - Added `rwh_06` module.
  - Added `HasRawWindowHandles` trait.
  - Added `HasRwh5Handles` trait.
  - Added `HasRwh6Handles` trait.
- Added public re-export of the `winit` crate.


## [wolf_engine_winit]

## [Unreleased]

- Added EventLoop / main-loop implementation based on `winit`.
- Added basic window-system features:
  - Added window creation / deletion.
  - Added window resizing.
  - Added multi-window support.
  - Added Raw Window Handle integration.
  - Added window inputs.

## [wolf_engine_input]

### [Unreleased]

- Changed "raw" keyboard input to use the normal `Keyboard` input.
  - Removed `Input::RawKeyboard` variant.
- Changed to pressed / released events for buttons.
  - Removed `ButtonState`.
- Renamed events to be in past-tense.

### [0.1.2] - 2023-07-09

- Updated `winit` integration to `v0.30.x`.

### [0.1.1] - 2024-03-24

- Updated `winit` integration to `v0.29.x`. 

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

### [0.2.1] - 2024-11-03

- Add missing docs / changelog entry. 

### [0.2] - 2024-11-03

- Simplified the dynamic event system a bit.
  - Removed the `dynamic` feature flag, since most things are likely going to
    use the dynamic events system anyways.
  - Renamed `DynaimcEventBox` to `AnyEvent`.
  - Removed derive macro for `DynamicEvent`.
  - Removed `DynamicEventSender`.
  - Removed the `dynamic::events` module.
  - Removed the `dynamic::event_loop` module.
  - Changed the `EventLoop` struct into a trait.


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
