use std::process::ExitCode;

use event::WindowEvent;
use libtest_mimic::{Arguments, Failed, Trial};
use wolf_engine_events::event_loop::EventLoop;
use wolf_engine_window::{backend::WindowSystem, *};

pub fn main() -> ExitCode {
    let mut args = Arguments::from_args();
    args.test_threads = Some(1);
    let tests = vec![Trial::test("winit_smoketest", test)];
    libtest_mimic::run(&args, tests).exit_code()
}

fn test() -> Result<(), Failed> {
    let window_system = wolf_engine_winit::init().unwrap();
    let context = window_system.context();

    let mut has_quit = false;

    window_system.run(|event| {
        if let Some(window_event) = event.downcast_ref::<WindowEvent>() {
            match window_event {
                WindowEvent::Started => {
                    let _window =
                        context.create_window(WindowSettings::default().with_visible(false));
                }
                WindowEvent::WindowCreated(_, window_result) => {
                    window_result.as_ref().expect("Window creation succeeded");
                    context.exit();
                }
                WindowEvent::Exited => *&mut has_quit = true,
                _ => (),
            }
        }
    });

    assert!(has_quit, "The has_quit flag is not set");

    Ok(())
}
