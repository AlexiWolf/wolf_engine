use std::process::ExitCode;

use libtest_mimic::{Arguments, Failed, Trial};
use wolf_engine_window::event::WindowEvent;
use wolf_engine_window::*;

pub fn main() -> ExitCode {
    let mut args = Arguments::from_args();
    args.test_threads = Some(1);
    let tests = vec![Trial::test("should_run_and_quit", test)];
    libtest_mimic::run(&args, tests).exit_code()
}

fn test() -> Result<(), Failed> {
    let window_system = wolf_engine_winit::init().build().unwrap();
    let context = window_system.context();

    let mut has_quit = false;

    window_system.run(|event| match event {
        WindowEvent::Started => {
            let _window = context.create_window(WindowSettings::default().with_visible(false));
        }
        WindowEvent::WindowEvent(_, WindowEvent::Created(window_result)) => {
            let _window = window_result.expect("Window creation succeeded");
            context.exit();
        }
        WindowEvent::Exited => *&mut has_quit = true,
        _ => (),
    });
    Ok(())
}
