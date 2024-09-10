use wolf_engine_events::{
    dynamic::{AnyEvent, AnyEventSender},
    mpsc, EventReceiver,
};

#[derive(Debug)]
pub enum TestEvent {
    A,
    B,
}

#[derive(Debug)]
pub enum ExampleEvent {
    C,
    D,
}

fn main() {
    let (event_sender, mut event_loop) = mpsc::event_queue::<AnyEvent>();
    let _ = event_sender.send_any_event(TestEvent::A);
    let _ = event_sender.send_any_event(TestEvent::B);
    let _ = event_sender.send_any_event(ExampleEvent::C);
    let _ = event_sender.send_any_event(ExampleEvent::D);

    while let Some(any_event) = event_loop.next_event() {
        if let Some(event) = any_event.downcast_ref::<TestEvent>() {
            match event {
                TestEvent::A => println!("A"),
                TestEvent::B => println!("B"),
            }
        } else if let Some(event) = any_event.downcast_ref::<ExampleEvent>() {
            match event {
                ExampleEvent::C => println!("C"),
                ExampleEvent::D => println!("D"),
            }
        }
    }
}
