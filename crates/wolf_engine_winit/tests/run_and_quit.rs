use std::process::ExitCode;

use libtest_mimic::{Arguments, Failed, Trial};
use wolf_engine_events::dynamic::AnyEvent;
use wolf_engine_events::mpsc;
use wolf_engine_window::event::{Event, WindowEvent};
use wolf_engine_window::*;

pub fn main() -> ExitCode {
    let mut args = Arguments::from_args();
    args.test_threads = Some(1);
    let tests = vec![Trial::test("should_run_and_quit", test)];
    libtest_mimic::run(&args, tests).exit_code()
}

fn test() -> Result<(), Failed> {
    let (event_sender, event_reciever) = mpsc::event_queue::<AnyEvent>();
    let context = wolf_engine_window::init(event_sender.clone());
    let _window = context.create_window(WindowSettings::default().with_visible(false));

    let mut has_quit = false;

    let result = wolf_engine_winit::run(event_reciever, |event| match event {
        Event::Started => {
            context.exit();
        }
        Event::WindowEvent(_, WindowEvent::Ready(window_result)) => {
            window_result.expect("Window creation succeeded");
        }
        Event::Exited => *&mut has_quit = true,
        _ => (),
    });

    assert!(result.is_ok(), "The window system returned an error");
    assert!(has_quit, "The has_quit flag was not set");

    Ok(())
}
