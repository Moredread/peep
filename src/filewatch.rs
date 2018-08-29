extern crate inotify;

use std::sync::mpsc;
use std::thread::spawn;
use event::PeepEvent;

pub fn inotifier(file_path: &str,
             event_sender: mpsc::Sender<PeepEvent>) {
    let mut inotify = inotify::Inotify::init().expect("Failed to initialize inotify");

    if file_path == "-" {
        use std::{thread, time};
        // from pipe
        // FIXME: how to watch stdin?
        let _ = spawn(move || {
            loop {
                thread::sleep(time::Duration::from_millis(500));
                event_sender.send(PeepEvent::FileUpdated).unwrap();
            }
        });
    } else {
        // from file
        inotify
            .add_watch(file_path, inotify::WatchMask::MODIFY)
            .expect("Failed to add inotify watch");

        let _ = spawn(move || {
            let mut buffer = [0u8; 1024];
            loop {
                let events = inotify
                    .read_events_blocking(&mut buffer)
                    .expect("Failed to read inotify events");
                for event in events {
                    if event.mask.contains(inotify::EventMask::MODIFY) {
                        event_sender.send(PeepEvent::FileUpdated).unwrap();
                    }
                }
            }
        });
    }
}
