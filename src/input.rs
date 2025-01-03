use std::sync::{ atomic::{ AtomicBool, Ordering }, Arc };
use std::time::Duration;
use crossterm::event::{ self, Event, KeyCode, KeyModifiers };

pub fn start_input_listener(running: Arc<AtomicBool>) {
    println!("Press ALT+C to quit.");
    while running.load(Ordering::Relaxed) {
        if event::poll(Duration::from_millis(100)).unwrap_or(false) {
            if let Event::Key(key_event) = event::read().unwrap() {
                if
                    key_event.modifiers.contains(KeyModifiers::ALT) &&
                    key_event.code == KeyCode::Char('c')
                {
                    running.store(false, Ordering::Relaxed);
                    println!("Exiting...");
                    break;
                }
            }
        }
    }
}
