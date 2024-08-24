use std::process::ExitCode;

use libtest_mimic::{Arguments, Failed, Trial};
use wolf_engine_window::event::{Event, WindowEvent};
use wolf_engine_window::*;

pub fn main() -> ExitCode {
    let mut args = Arguments::from_args();
    args.test_threads = Some(1);
    let tests = vec![Trial::test("should_run_and_quit", test)];
    libtest_mimic::run(&args, tests).exit_code()
}

fn test() -> Result<(), Failed> {
    let (event_loop, context) = init().build().unwrap();
    let _window = context.create_window(WindowSettings::default().with_visible(false));

    let mut has_quit = false;

    event_loop.run(|event| match event {
        Event::Started => {
            context.exit();
        }
        Event::WindowEvent(_, WindowEvent::Created(window_result)) => {
            let _window = window_result.expect("Window creation succeeded");
        }
        Event::Exited => *&mut has_quit = true,
        _ => (),
    });

    assert!(
        has_quit,
        "The window system has not quit, or did not run properly."
    );
    Ok(())
}
