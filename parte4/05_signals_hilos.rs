use signal_hook::consts::{SIGINT, SIGTERM};
use signal_hook::iterator::Signals;
use std::sync::mpsc;
use std::thread;

fn main() -> std::io::Result<()> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut signals = Signals::new(&[SIGINT, SIGTERM]).unwrap();
        for _ in signals.forever() {
            tx.send(()).unwrap();
        }
    });

    println!("Esperando señal...");

    rx.recv().unwrap();

    println!("Shutdown limpio");
    Ok(())
}