use std::process::ExitCode;

use libtest_mimic::{Arguments, Failed, Trial};
use wolf_engine_window::event::{Event, WindowEvent};
use wolf_engine_window::*;

pub fn main() -> ExitCode {
    let mut args = Arguments::from_args();
    args.test_threads = Some(1);
    let tests = vec![Trial::test(
        "should_not_send_events_for_dropped_windows",
        test,
    )];
    libtest_mimic::run(&args, tests).exit_code()
}

fn test() -> Result<(), Failed> {
    let event_loop = wolf_engine_window::init().build().unwrap();
    let mut window = None;

    event_loop.run(|event, context| match event {
        Event::Started => {
            window = Some(
                context
                    .create_window(WindowSettings::default().with_visible(false))
                    .expect("window creation succeeded"),
            );
        }
        Event::EventsCleared => {
            if let Some(window) = window.as_ref() {
                window.redraw();
            } else {
                context.exit();
            }
            window = None;
        }
        Event::WindowEvent(_, WindowEvent::RedrawRequested) => {
            let window = window.as_ref().expect("this event is not sent to the user");
            window.redraw();
        }
        _ => (),
    });
    Ok(())
}
