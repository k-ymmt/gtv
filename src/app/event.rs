use super::termion::event;

use super::crossbeam::Sender;
use super::termion::input::TermRead;
use crate::app::{AppError, Result};
use signal_hook::iterator::Signals;
use std::io::stdin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::SendError;
use std::sync::Arc;
use std::thread;

pub enum Event {
    Key(event::Key),
    Mouse(termion::event::MouseEvent),
    Resize,
}

pub fn watch_resize_event(sender: Sender<()>, running: Arc<AtomicBool>) -> Result<()> {
    let term = running.clone();
    let signals = Signals::new(&[signal_hook::SIGWINCH])?;
    thread::spawn(move || {
        while term.load(Ordering::Relaxed) {
            if signals.wait().count() > 0 {
                sender.send(());
            }
        }
    });

    Ok(())
}

pub fn watch_input_event(sender: Sender<(Option<Event>)>) {
    thread::spawn(move || {
        let stdin = stdin();
        for key in stdin.keys() {
            let result = key.map(|key| Event::Key(key));
            sender.send(result.ok());
        }
    });
}
