
use std::thread::spawn;
mod eating_philosophers;
use eating_philosophers::Philosopher;

fn main() {
        let (forks, philosophers) = eating_philosophers::init();
        let mut handles: Vec<_> = philosophers.into_iter().map(|p| {
            let forks_for_child = forks.clone();

            spawn(move || {
                p.eat(&forks_for_child);
            })
        }).collect();

        for h in handles {
            h.join().unwrap();
        }
}
