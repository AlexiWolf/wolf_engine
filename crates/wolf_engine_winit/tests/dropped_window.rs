use std::process::{exit, ExitCode};
use std::thread;
use std::time::Duration;

use libtest_mimic::{Arguments, Failed, Trial};
use wolf_engine_events::EventLoop;
use wolf_engine_window::event::{Event, WindowEvent};
use wolf_engine_window::*;

pub fn main() -> ExitCode {
    let mut args = Arguments::from_args();
    args.test_threads = Some(1);
    let tests = vec![Trial::test(
        "should_not_send_events_for_dropped_windows",
        test,
    )];
    thread::spawn(|| {
        thread::sleep(Duration::from_secs(1));
        println!("Error: Test timed out");
        exit(1);
    });
    libtest_mimic::run(&args, tests).exit_code()
}

fn test() -> Result<(), Failed> {
    let window_system = wolf_engine_winit::init().unwrap();
    let context = window_system.context();
    let mut window = Some(context.create_window(WindowSettings::default().with_visible(false)));
    window_system.run(|event| {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::Started => {
                    context.create_window(WindowSettings::default().with_visible(false));
                }
                Event::EventsCleared => {
                    if let Some(window) = window.as_ref() {
                        window.redraw();
                    } else {
                        context.exit()
                    }
                    window = None;
                }
                Event::WindowEvent(_, WindowEvent::Ready(window_result)) => {
                    window_result
                        .as_ref()
                        .expect("window creation should have succeeded");
                }
                Event::WindowEvent(_, WindowEvent::RedrawRequested) => {
                    let window = window
                        .as_ref()
                        .expect("this event should not be sent to the user");
                    window.redraw();
                }
                _ => (),
            }
        }
    });
    Ok(())
}
