use rdev::{Event, EventType, Key, listen, unhook};
use std::thread;

static MOUSE_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn main() {
    println!("=== rdev Unhook Example ===");
    println!("This example demonstrates how to stop event listening using unhook()");
    println!();
    println!("Instructions:");
    println!("- All keyboard and mouse events will be printed");
    println!("- Press 'o' key to stop listening and exit");
    println!("- Mouse move events are throttled to avoid spam");
    println!();
    println!("Starting event listener...");
    
    // Start listening in a separate thread so we can handle the unhook
    let handle = thread::spawn(|| {
        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error)
        }
    });
    
    // Wait for the listener thread to finish
    if let Err(e) = handle.join() {
        println!("Thread panicked: {:?}", e);
    }
    
    println!("Event listening stopped. Goodbye!");
}

fn callback(event: Event) {
    // Print all events to console
    match event.event_type {
        EventType::KeyPress(key) => {
            println!("Key pressed: {:?} at {:?}", key, event.time);
            
            // Check if 'o' key was pressed
            if key == Key::KeyO {
                println!("'o' key detected! Stopping event listener...");
                
                // Call unhook to stop listening
                if unhook() {
                    println!("Successfully called unhook()");
                } else {
                    println!("Failed to call unhook()");
                }
            }
        },
        EventType::KeyRelease(key) => {
            println!("Key released: {:?} at {:?}", key, event.time);
        },
        EventType::ButtonPress(button) => {
            println!("Mouse button pressed: {:?} at {:?}", button, event.time);
        },
        EventType::ButtonRelease(button) => {
            println!("Mouse button released: {:?} at {:?}", button, event.time);
        },
        EventType::MouseMove { x, y } => {
            // Only print mouse move events occasionally to avoid spam
            let count = MOUSE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            if count % 50 == 0 {
                println!("Mouse moved to ({}, {}) at {:?}", x, y, event.time);
            }
        },
        EventType::Wheel { delta_x, delta_y } => {
            println!("Mouse wheel: delta_x={}, delta_y={} at {:?}", delta_x, delta_y, event.time);
        }
    }
}
