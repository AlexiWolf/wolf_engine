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
    let tests = vec![Trial::test("should_run_and_quit", test)];
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
    let _window = context.create_window(WindowSettings::default().with_visible(false));

    let mut has_quit = false;

    window_system.run(|event| {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::Started => {
                    context.exit();
                }
                Event::WindowEvent(_, WindowEvent::Ready(window_result)) => {
                    window_result.as_ref().expect("Window creation succeeded");
                }
                Event::Exited => *&mut has_quit = true,
                _ => (),
            }
        }
    });

    assert!(has_quit, "The has_quit flag was not set");

    Ok(())
}
