use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::thread::spawn;
use std::time::Duration;

pub struct Philosopher {
    name: String,
    left_hand: u8,
    right_hand: u8,
}

impl Philosopher {
    fn new(name: String, left_hand: u8, right_hand: u8) -> Philosopher {
        Philosopher {
            name,
            left_hand,
            right_hand,
        }
    }

    fn eat(&self, forks: &[AtomicBool; 5]) {
        while forks[self.left_hand as usize].load(Ordering::SeqCst) == true {}
        forks[self.left_hand as usize].store(true, Ordering::SeqCst);
        while forks[self.right_hand as usize].load(Ordering::SeqCst) == true {}
        forks[self.right_hand as usize].store(true, Ordering::SeqCst);

        println!("{} Started eating.", self.name);

        thread::sleep(Duration::from_millis(2000));

        println!("{} Finished eating.", self.name);
        forks[self.right_hand as usize].store(false, Ordering::SeqCst);
        forks[self.left_hand as usize].store(false, Ordering::SeqCst);
    }
}

fn init() -> (Arc<[AtomicBool; 5]>, Vec<Philosopher>) {
    let mut forks: [AtomicBool; 5] = [
        AtomicBool::new(false),
        AtomicBool::new(false),
        AtomicBool::new(false),
        AtomicBool::new(false),
        AtomicBool::new(false),
    ];
    let mut philosophers = Vec::new();

    for i in 0..5 {
        philosophers.push(Philosopher::new(
            (i + 1).to_string(),
            i,
            if i == 4 { 0 } else { i + 1 },
        ));
    }

    (Arc::new(forks), philosophers)
}

pub fn eating_philosophers_run() {
    let (forks, philosophers) = init();
    let mut handles: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            let forks_for_child = forks.clone();

            spawn(move || {
                p.eat(&forks_for_child);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}
