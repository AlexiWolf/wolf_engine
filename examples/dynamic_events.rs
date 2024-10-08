use wolf_engine_events::{
    dynamic::{AnyEvent, Event},
    mpsc, EventReceiver, EventSender,
};

#[derive(Debug)]
pub enum TestEvent {
    A,
    B,
}
impl Event for TestEvent {}

#[derive(Debug)]
pub enum ExampleEvent {
    C,
    D,
}
impl Event for ExampleEvent {}

fn main() {
    let (event_sender, mut event_loop) = mpsc::event_queue::<AnyEvent>();
    let _ = event_sender.send_event(Box::new(TestEvent::A));
    let _ = event_sender.send_event(Box::new(TestEvent::B));
    let _ = event_sender.send_event(Box::new(ExampleEvent::C));
    let _ = event_sender.send_event(Box::new(ExampleEvent::D));

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
